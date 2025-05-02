use anyhow::{Context, Result};
use pollster::FutureExt;
use render::Renderer;
use std::sync::Arc;
use tracing::{error, info, warn};
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub struct Engine {
    renderer: Renderer,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        info!("Created window");

        let renderer = Renderer::new(window.clone())
            .block_on()
            .context("Failed to create renderer")?;

        Ok(Self { renderer })
    }

    pub fn start(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn run(&mut self, window: Arc<Window>, event_loop: EventLoop<()>) -> Result<()> {
        let mut surface_configured = false;
        let _target = event_loop.run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !self.renderer.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            surface_configured = true;
                            self.renderer.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            window.request_redraw();

                            if !surface_configured {
                                return;
                            }

                            self.update();
                            match self.renderer.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    self.renderer.resize(self.renderer.size())
                                }
                                Err(
                                    wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other,
                                ) => {
                                    error!("OutOfMemory");
                                    control_flow.exit();
                                }
                                Err(wgpu::SurfaceError::Timeout) => {
                                    warn!("Surface timeout")
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });

        Ok(())
    }

    pub fn destroy(&mut self) -> Result<()> {
        Ok(())
    }

    fn update(&mut self) {
        // remove `todo!()`
    }
}
