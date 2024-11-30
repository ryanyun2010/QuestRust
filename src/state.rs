use winit::window::Window;
use winit::event::*;
use wgpu::util::DeviceExt;
use crate::vertex::Vertex;
const TEST_INDICES: &[u16] = &[
    1,3,0, // bottom
    1,3,2,

    4,1,0, // front
    4,1,5,

    6,1,2, // right
    6,1,5,

    2,7,3, // back
    2,7,6,

    4,3,0, // left
    4,3,7,

    4,6,7, //top
    4,6,5
];
 
 

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indicies: u32,
    pub vertices: Vec<Vertex>,
    window: &'a Window,
}
impl<'a> State<'a> { 
    pub async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
            push_constant_ranges: &[],
            });
            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        Vertex::desc(),
                    ],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });
        let mut vertices: Vec<Vertex> = [
            Vertex { position: [0.0, 0.0, 0.0], color: [0.5, 0.0, 0.0] }, // A
            Vertex { position: [0.5, 0.0, 0.0], color: [0.5, 0.0, 0.0] }, // B
            Vertex { position: [0.5, 0.0, 0.5], color: [0.5, 0.0, 0.0] }, // C
            Vertex { position: [0.0, 0.0, 0.5], color: [0.5, 0.0, 0.0] }, // D
            Vertex { position: [0.0, 0.5, 0.0], color: [0.5, 0.0, 0.5] }, // E
            Vertex { position: [0.5, 0.5, 0.0], color: [0.5, 0.0, 0.5] }, // F
            Vertex { position: [0.5, 0.5, 0.5], color: [0.5, 0.0, 0.5] }, // G
            Vertex { position: [0.0, 0.5, 0.5], color: [0.5, 0.0, 0.5] }, // H
        ].to_vec();
        
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mut vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );
        

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(TEST_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indicies = TEST_INDICES.len() as u32;
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indicies,
            vertices
        }
 
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.vertices[0].position[0] += 0.005;
        self.vertices[0].position[1] += 0.0005;
        self.vertices[0].position[2] += 0.0005;
        if self.vertices[0].position[0] > 1.0 {
            self.vertices[0].position[0] = -1.0;
        }
        self.vertices[1].position[0] += 0.005;
        if self.vertices[1].position[0] > 1.0 {
            self.vertices[1].position[0] = -1.0;
        }
        self.vertices[2].position[0] += 0.005;
        if self.vertices[2].position[0] > 1.0 {
            self.vertices[2].position[0] = -1.0;
        }
        self.vertices[3].position[0] += 0.005;
        if self.vertices[3].position[0] > 1.0 {
            self.vertices[3].position[0] = -1.0;
        }      
        let staging_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
            label: Some("Staging Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::COPY_SRC,
            }
        );
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Staging Encoder"),
        });
        
        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.vertex_buffer,
            0,
            (self.vertices.len() * std::mem::size_of::<Vertex>()) as wgpu::BufferAddress,
        );
        
        self.queue.submit(std::iter::once(encoder.finish()));
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indicies, 0,0..1);
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
        
    }
}
