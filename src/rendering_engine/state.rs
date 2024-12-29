use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontRef};

use winit::{event, keyboard::{Key, NamedKey}, window::Window};
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use crate::vertex::Vertex;
use crate::texture::{create_texture_bind_group, Texture};
use crate::world::World;
use crate::camera::Camera;
use std::num::{NonZeroU64, NonZeroU32};
use std::fs;


const BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};


pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub keys_down: HashMap<String, bool>,
    pub level_editor: bool,
    pub left_mouse_button_down: bool,
    pub right_mouse_button_down: bool,
    pub text_brush: TextBrush<FontRef<'a>>,
    window: &'a Window,
}
impl<'a> State<'a> { 
    pub async fn new(window: &'a Window, sprites_to_load_json: Vec<String>) -> State<'a> {
        let size = window.inner_size();
        let keys_down = HashMap::new();
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
        let vertex_size = size_of::<Vertex>();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::TEXTURE_BINDING_ARRAY |  wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
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
        let mut texture_paths = Vec::new();
        for path in sprites_to_load_json.iter() {
            texture_paths.push(path.as_str());
        }
        let (texture_bind_group_layout, diffuse_bind_group): (wgpu::BindGroupLayout, wgpu::BindGroup) =
            create_texture_bind_group!(&device, &queue, &texture_paths);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let bytes = include_bytes!("./img/font.ttf");
        let brush: TextBrush<FontRef<'_>> = BrushBuilder::using_font_bytes(bytes).unwrap().build(&device, config.width, config.height, config.format);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });
            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vert_main"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: vertex_size as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Sint32],
                    }],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("non_uniform_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
        Self {
            window: window,
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            size: size,
            render_pipeline: render_pipeline,
            diffuse_bind_group: diffuse_bind_group,
            keys_down: keys_down,
            level_editor: false,
            left_mouse_button_down: false,
            right_mouse_button_down: false,
            text_brush: brush,
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
            self.text_brush.resize_view(self.config.width as f32, self.config.height as f32, &self.queue);
        }
    }
    pub fn update(&self, world: &mut World, camera: &mut Camera) {
        camera.update_ui(world);
        world.generate_collision_cache();
        world.process_input(self.keys_down.clone());
        camera.update_camera_position(&world);
        world.update_entities();
    }

    pub fn input(&mut self, event: winit::event::KeyEvent) {
        match event.logical_key {
            Key::Named(NamedKey::ArrowLeft) => {
                self.keys_down.insert("ArrowLeft".to_string(), event.state == event::ElementState::Pressed);
            },
            Key::Named(NamedKey::ArrowRight) => {
                self.keys_down.insert("ArrowRight".to_string(), event.state == event::ElementState::Pressed);
            },
            Key::Named(NamedKey::ArrowUp) => {
                self.keys_down.insert("ArrowUp".to_string(), event.state == event::ElementState::Pressed);
            },
            Key::Named(NamedKey::ArrowDown) => {
                self.keys_down.insert("ArrowDown".to_string(), event.state == event::ElementState::Pressed);
            }
            _ => {}
        }
        let key = event.logical_key.to_text();
        if key.is_none(){
            return;
        }
        let string_key = key.unwrap().to_string().to_lowercase();
        let press = match event.state {
            event::ElementState::Pressed => true,
            event::ElementState::Released => false,
        };
        
        self.keys_down.insert(string_key, press);
    }

    pub fn render(&mut self, world: &mut World, camera: &mut Camera) -> Result<(), wgpu::SurfaceError> {
        let render_data = if self.level_editor {&camera.level_editor_render(world)} else {&camera.render(world)};
        
        let vertices = &render_data.vertex;
        if vertices.len() < 1 {
            return Ok(());
        }
        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let indicies = &render_data.index;
        let num_indicies = indicies.len() as u32;

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indicies),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let sections = camera.get_sections(self.config.width as f32, self.config.height as f32);
            self.text_brush.queue(&self.device, &self.queue, sections).unwrap();
            
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[0]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..num_indicies,0, 0..1);
            self.text_brush.draw(&mut render_pass)
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
        
    }
}