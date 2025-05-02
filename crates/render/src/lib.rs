use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::{debug, info};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{event::WindowEvent, window::Window};

pub struct Renderer {
    instance: Instance,
    surface: Surface<'static>,
    adapter: Adapter,
    device: Arc<Device>,
    queue: Arc<Queue>,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        info!("Creating renderer");

        let instance: Instance = {
            let l_instance = Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN,
                ..Default::default()
            });
            debug!("Created wgpu instance");
            l_instance
        };

        let surface: Surface<'static> = {
            let l_surface = instance
                .create_surface(Arc::clone(&window))
                .context("Failed to create surface")?;
            debug!("Created wgpu surface");
            l_surface
        };

        let adapter: Adapter = {
            let l_adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .context("Failed to find suitable GPU adapter")?;
            debug!("Found suitable GPU adapter");

            let info = l_adapter.get_info();
            info!(
                "Using adapter: {} ({:?}, {:?})",
                info.name, info.device_type, info.backend
            );

            l_adapter
        };

        let (device, queue) = {
            let (l_device, l_queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                })
                .await
                .context("Failed to create device")?;
            debug!("Created device and queue");
            debug!("Device limits: {:?}", l_device.limits());
            (l_device, l_queue)
        };

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .context("Surface not supported by adapter")?;
        surface.configure(&device, &config);

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            window,
        })
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
