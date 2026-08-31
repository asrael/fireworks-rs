mod fireworks;
mod renderer;
mod world;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

use glam::Vec3;
use pixels::{Pixels, ScalingMode, SurfaceTexture};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use web_time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use fireworks::{EffectId, Fireworks, Range, CATALOG};
use renderer::{Renderer, HEIGHT, WIDTH};
use world::World;

const LAUNCH_X: Range = Range::new(-18.0, 18.0);

struct App {
    auto_show: bool,
    cursor: Option<PhysicalPosition<f64>>,
    fireworks: Fireworks,
    last_frame: Instant,
    next_launch: f32,
    pixels: Rc<RefCell<Option<Pixels<'static>>>>,
    renderer: Renderer,
    rng: SmallRng,
    selected: EffectId,
    size: Option<PhysicalSize<u32>>,
    window: Option<Arc<Window>>,
}

impl App {
    fn new() -> Self {
        Self {
            auto_show: true,
            cursor: None,
            fireworks: Fireworks::new(CATALOG, World::default(), 0x5EED),
            last_frame: Instant::now(),
            next_launch: 0.5,
            pixels: Rc::new(RefCell::new(None)),
            renderer: Renderer::new(),
            rng: SmallRng::seed_from_u64(0x2545_F491),
            selected: 0,
            size: None,
            window: None,
        }
    }

    fn launch(&mut self, id: EffectId) {
        let x = LAUNCH_X.sample(&mut self.rng);
        let up = CATALOG[id as usize].lift_speed.sample(&mut self.rng);

        self.fireworks
            .launch(id, Vec3::new(x, 0.0, 0.0), Vec3::new(0.0, up, 0.0));
    }

    fn update(&mut self, dt: f32) {
        self.next_launch -= dt;

        if self.next_launch <= 0.0 {
            let id = if self.auto_show {
                self.next_launch = 1.0;
                self.rng.random_range(0..CATALOG.len() as EffectId)
            } else {
                self.next_launch = 3.0;
                self.selected
            };

            self.launch(id);
        }

        self.fireworks.update(dt);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title("fireworks")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

        #[cfg(target_arch = "wasm32")]
        let attributes = {
            use winit::platform::web::WindowAttributesExtWebSys;
            attributes.with_append(true).with_prevent_default(false)
        };

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        #[cfg(target_arch = "wasm32")]
        {
            let cell = self.pixels.clone();
            let win = window.clone();

            wasm_bindgen_futures::spawn_local(async move {
                use pixels::wgpu::util::is_browser_webgpu_supported;
                use pixels::wgpu::{Backends, TextureFormat};

                let backends = if is_browser_webgpu_supported().await {
                    Backends::BROWSER_WEBGPU
                } else {
                    Backends::GL
                };

                let surface = SurfaceTexture::new(WIDTH, HEIGHT, win);
                let mut pixels = pixels::PixelsBuilder::new(WIDTH, HEIGHT, surface)
                    .wgpu_backend(backends)
                    .texture_format(TextureFormat::Rgba8Unorm)
                    .surface_texture_format(TextureFormat::Rgba8Unorm)
                    .build_async()
                    .await
                    .unwrap();

                pixels.set_scaling_mode(ScalingMode::Fill);
                *cell.borrow_mut() = Some(pixels);
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let surface = SurfaceTexture::new(WIDTH, HEIGHT, window.clone());
            let mut pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

            pixels.set_scaling_mode(ScalingMode::Fill);
            *self.pixels.borrow_mut() = Some(pixels);
        }

        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::CursorMoved { position, .. } => self.cursor = Some(position),

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                let clicked = self.cursor.and_then(|p| {
                    self.pixels
                        .borrow()
                        .as_ref()?
                        .window_pos_to_pixel((p.x as f32, p.y as f32))
                        .ok()
                });

                if let Some((sx, sy)) = clicked {
                    let pos = Renderer::unproject(sx as f32, sy as f32);
                    let id = self.rng.random_range(0..CATALOG.len() as EffectId);

                    self.fireworks.burst(id, pos);
                }
            }

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - self.last_frame).as_secs_f32().min(1.0 / 30.0);

                self.last_frame = now;
                self.update(dt);

                let Some(window) = self.window.as_ref() else {
                    return;
                };
                let size = window.inner_size();

                if size.width == 0 || size.height == 0 {
                    return;
                }

                if let Some(pixels) = self.pixels.borrow_mut().as_mut() {
                    if self.size != Some(size) {
                        if pixels.resize_surface(size.width, size.height).is_err() {
                            event_loop.exit();
                            return;
                        }

                        self.size = Some(size);
                    }

                    self.renderer.draw(&self.fireworks, pixels.frame_mut());

                    if pixels.render().is_err() {
                        event_loop.exit();
                    }
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(if cfg!(target_arch = "wasm32") {
        ControlFlow::Wait
    } else {
        ControlFlow::Poll
    });

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;
        console_error_panic_hook::set_once();
        event_loop.spawn_app(App::new());
    }

    #[cfg(not(target_arch = "wasm32"))]
    event_loop.run_app(&mut App::new())?;

    Ok(())
}
