#[allow(dead_code)]

use std::sync::Arc;
use wgpu::{*};
use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::{Window, WindowId},
};

use crate::{GameState, Engine, Character};

use std::time::{Duration, Instant};
use anyhow::Result;

// WindowState. Holds the objects linked to the wgpu API.
pub struct WindowState {
  // window fields
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  window: Arc<Window>,
  
  // shader fields
  bg_pipeline: wgpu::RenderPipeline,
  bg_instance_buffer: wgpu::Buffer,
  num_bg_instances: u32,
  
  // glyphon fields
  font_system: glyphon::FontSystem,
  text_renderer: glyphon::TextRenderer,
  text_atlas: glyphon::TextAtlas,
  swash_cache: glyphon::SwashCache,
  view_port: glyphon::Viewport,
  buffer: glyphon::Buffer,
  metrics: glyphon::Metrics, // dictates the font size. needs to be saved for consistency across resizes.
    
  span_cache: Vec<(String, glyphon::Attrs<'static>)>,
}

impl WindowState {

  pub async fn new(window: Arc<Window>) -> Result<Self> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY, // only vulkan, dx12 and metal
                                         // backends: wgpu::Backends::VULKAN | wgpu::Backends::DX12 | wgpu::Backends::METAL, 
      ..Default::default()
    });
    
    let surface = instance.create_surface(window.clone())?;
    
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      compatible_surface: Some(&surface),
      force_fallback_adapter: false,
    }).await?;
    
    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        label: Some("WGPU Device"),
        memory_hints: wgpu::MemoryHints::default(),
        required_features: wgpu::Features::default(),
        required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        trace: wgpu::Trace::Off,
    }).await?;
    
    // configuring surface
    let caps = surface.get_capabilities(&adapter);
    let surface_format = caps.formats[0];

    let size = window.inner_size();

    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    
    surface.configure(&device, &surface_config);
    
    // shader setup
    let shader = device.create_shader_module(wgpu::include_wgsl!("rect_shader.wgsl"));
    
    let bg_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Background Rect Pipeline"),
      layout: None,
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        buffers: &[crate::RectInstance::desc()], // Describe our struct
      },
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleStrip, // 4 verts per rect
        ..Default::default()
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        targets: &[Some(wgpu::ColorTargetState {
          format: surface_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL,
        })],
      }),
      multiview_mask: None,
      cache: None,
    });
    
    let bg_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Instance Buffer"),
        size: (std::mem::size_of::<crate::RectInstance>() * 5000) as u64, // 5000 is theoretically the maximum size
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    
    // glyphon setup
    let mut font_system = glyphon::FontSystem::new_with_locale_and_db(
      "en-US".into(), 
      glyphon::cosmic_text::fontdb::Database::new()
    );
    
    let cache = glyphon::Cache::new(&device);
    
    let mut text_atlas = glyphon::TextAtlas::new(
        &device,
        &queue,
        &cache,
        caps.formats[0], // Use the format from surface.get_capabilities
    );
    
    let text_renderer = glyphon::TextRenderer::new(
        &mut text_atlas,
        &device,
        wgpu::MultisampleState::default(),
        None, // Depth stencil
    );
    
    let view_port = glyphon::Viewport::new(&device, &cache);
    
    let mut buffer = glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(30.0, 42.0));
    buffer.set_size(&mut font_system, Some(size.width as f32), Some(size.height as f32));
    
    let _ = Self::load_font(&mut font_system)?;
    
    return Ok(WindowState{
      surface: surface,
      device: device,
      queue: queue,
      config: surface_config,
      size: size,
      window: window,
      
      bg_pipeline,
      bg_instance_buffer,
      num_bg_instances: 0,
      
      font_system,
      text_renderer,
      text_atlas,
      swash_cache: glyphon::SwashCache::new(),
      view_port,
      buffer,
      metrics: glyphon::Metrics{ font_size: 32.0, line_height: 32.0 },
      
      span_cache: Vec::with_capacity(5000),
    });
  }
  
  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, grid_cols: u32, grid_rows: u32) {
    if new_size.width > 0 && new_size.height > 0 {
    
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      
      self.view_port.update(
        &self.queue, 
        glyphon::Resolution { width: new_size.width, height: new_size.height }
      );
      
      // calculating new font size
      let win_w = new_size.width as f32;
      let win_h = new_size.height as f32;
      
      let font_ratio = win_h / win_w + 0.001;
      
      let width_per_char = win_w * font_ratio / grid_cols as f32;
      let height_per_char = win_h / font_ratio / grid_rows as f32;
      
      let new_font_size = width_per_char.min(height_per_char);
      
      self.metrics = glyphon::Metrics::new(new_font_size, new_font_size);
      self.buffer.set_metrics(&mut self.font_system, self.metrics);
      
      self.buffer.set_size(
        &mut self.font_system,
        Some(new_size.width as f32),
        Some(new_size.height as f32)
      );
      
      // final config
      self.surface.configure(&self.device, &self.config);
      
      self.text_atlas.trim();
    }
  }
  
  pub fn render(&mut self) {
    self.window.request_redraw();
  }
  
  fn load_font(font_system: &mut glyphon::FontSystem) -> Result<String> {
    let font_data = include_bytes!("assets/PressStart2P-Regular.ttf").to_vec();
    
    font_system.db_mut().load_font_data(font_data);
    
    let mut font_name = "";
    
    if let Some(face) = font_system.db().faces().last() {

      if let Some((name, _)) = face.families.first() {
        font_name = name;
      }
      
    }
    
    return Ok(font_name.to_string());
    
    // return Ok(font_name);
  }
  
}

/// WindowGame. An abstraction layer between the User and the Engine. Is responsible for the game loop, rendering and input catching.
pub struct WindowGame<GS>
where GS: GameState {
  window_state: Option<WindowState>,  
  
  // game stuff
  current_time: Instant,
  fixed_time_step: Duration,
  accumulator: Duration,
  
  engine: Engine<GS>,
  game_state: GS,
  
}

impl<GS> WindowGame<GS>
where GS: GameState {
  pub fn new() -> Result<Self> {    
    let (cols, rows) = (32, 18);
    
    let mut eng = Engine::<GS>::new((cols as u32, rows as u32));
    let gs = GameState::new(&mut eng);
    
    Ok(Self{
      window_state: None,
      
      current_time: Instant::now(),
      fixed_time_step: Duration::new(0,0),
      accumulator: Duration::new(0,0),
      
      engine: eng,
      game_state: gs,
    })
  }
  
  fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
  
    let ws = self.window_state.as_mut().unwrap();
    
    let drawable = ws.surface.get_current_texture()?; // SurfaceTexture
    let image_view_descriptor = TextureViewDescriptor::default();
    let image_view = drawable.texture.create_view(&image_view_descriptor); // TextureView
    
    let command_enconder_descriptor = CommandEncoderDescriptor{ // CommandEncoderDescriptor
      label: Some("Render Encoder"),
    };
    let mut command_encoder = ws.device.create_command_encoder(&command_enconder_descriptor); // CommandEncoder
    
    let window_width = ws.config.width as f32;
    let window_height = ws.config.height as f32;
    
    let left_offset = ( window_width - ws.metrics.font_size * self.engine.db.width as f32 ) / 2.0;
    let top_offset = ( window_height - ws.metrics.line_height * self.engine.db.height as f32 ) / 2.0;
  
    // glyphon preparation
    if self.engine.db.text_changed {
      
      // text update:
      ws.span_cache.clear();
      
      for row in &self.engine.db.characters {
        for char_struct in row {
          // Convert the Engine Color to Glyphon Color
          let g_color = glyphon::Color::rgb(
            char_struct.color.r,
            char_struct.color.g,
            char_struct.color.b
          );

          // Create attributes for this specific character
          let attrs = glyphon::Attrs::new()
            // .stretch(glyphon::Stretch::Normal)
            .family(glyphon::Family::Name("Press Start 2P"))
            .color(g_color)
            .metrics(ws.metrics);
          
          // Push to cache
          // Note: char.to_string() is fast, but if you want micro-optimization later,
          // we can discuss "Cow" strings. For now, this is fine.
          ws.span_cache.push((char_struct.symbol.to_string(), attrs));
        }

        // Add a newline at the end of every row
        ws.span_cache.push(("\n".to_string(), glyphon::Attrs::new()));
      }
      
      ws.buffer.set_rich_text(
        &mut ws.font_system,
        ws.span_cache.iter().map(|(s, attrs)| (s.as_str(), attrs.clone())),
        &glyphon::Attrs::new(),
        glyphon::Shaping::Advanced,
        None, // Some(glyphon::cosmic_text::Align::Center),
      );
      
      ws.buffer.shape_until_scroll(&mut ws.font_system, false);
      
      // background update
      let mut bg_data: Vec<crate::RectInstance> = Vec::new();
      
      let cell_w_pixels = ws.metrics.font_size;
      let cell_h_pixels = ws.metrics.font_size;
      
      
      // convert Pixel Size to Clip Space Size (0.0 to 2.0 range)
      let cell_w_clip = (cell_w_pixels / window_width) * 2.0;
      let cell_h_clip = (cell_h_pixels / window_height) * 2.0;
      let x_offset = (left_offset / window_width) * 2.0;
      let y_offset = (top_offset / window_height) * 2.0;
      
      let nudge_y_offset = (-1.5 / window_height) * 2.0;

      for (row_idx, row) in self.engine.db.characters.iter().enumerate() {
        for (col_idx, char_struct) in row.iter().enumerate() {
          
          // skip if background is pure black (optimization)
          if char_struct.color_back.r == 0 && char_struct.color_back.g == 0 && char_struct.color_back.b == 0 {
              continue;
          }
          
          let x = -1.0 + (col_idx as f32 * cell_w_clip + x_offset);
          let y = 1.0 - (row_idx as f32 * cell_h_clip + y_offset + nudge_y_offset) - cell_h_clip; 

          bg_data.push(crate::RectInstance {
            position: [x, y],
            color: [char_struct.color_back.r as f32 / 255.0, char_struct.color_back.g as f32 / 255.0, char_struct.color_back.b as f32 / 255.0],
            size: [cell_w_clip, cell_h_clip],
          });
        }
      }
      
      // check if buffer is big enough
      let needed_size = (bg_data.len() * std::mem::size_of::<crate::RectInstance>()) as u64;
      if needed_size > ws.bg_instance_buffer.size() {
        // println!("Warning: resizing background buffer!");
        ws.bg_instance_buffer = ws.device.create_buffer(&wgpu::BufferDescriptor {
          label: Some("Instance Buffer"),
          size: needed_size * 2, // Grow x2
          usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
          mapped_at_creation: false,
        });
      }
      
      // Upload to GPU
      ws.queue.write_buffer(&ws.bg_instance_buffer, 0, bytemuck::cast_slice(&bg_data));
      ws.num_bg_instances = bg_data.len() as u32;
      
      self.engine.db.text_changed = false;
    }
    
    let text_area = glyphon::TextArea {
      buffer: &ws.buffer,
      left: left_offset,
      top: top_offset,
      scale: 1.0,
      bounds: glyphon::TextBounds {
        left: 0,
        top: 0,
        right: window_width as i32,
        bottom: window_height as i32,
      },
      default_color: glyphon::Color::rgb(255, 255, 255),
      custom_glyphs: &[],
    };
    
    // upload textures to gpu
    ws.text_renderer.prepare(
      &ws.device,
      &ws.queue,
      &mut ws.font_system,
      &mut ws.text_atlas,
      &ws.view_port,
      vec![text_area],
      &mut ws.swash_cache,
    ).unwrap(); // TODO
    
    // preparing the render pass
    
    let color_attachment = RenderPassColorAttachment{
      view: &image_view,
      resolve_target: None,
      depth_slice: None,
      ops: Operations{
        load: LoadOp::Clear(Color{
          r: 0.0,
          g: 0.0,
          b: 0.0,
          a: 0.0,
        }),
        store: StoreOp::Store,
      },
    };
  
    let render_pass_descriptor = RenderPassDescriptor{
      label: Some("Render Pass"),
      color_attachments: &[Some(color_attachment)],
      depth_stencil_attachment: None,
      occlusion_query_set: None,
      timestamp_writes: None,
      multiview_mask: None,
    };
  
    // render pass
    {
    
      let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
      
      // background
      render_pass.set_pipeline(&ws.bg_pipeline);
      render_pass.set_vertex_buffer(0, ws.bg_instance_buffer.slice(..));
      // draw 4 vertices (TriangleStrip) * N instances
      render_pass.draw(0..4, 0..ws.num_bg_instances);
    
      // letters
      ws.text_renderer.render(&ws.text_atlas, &ws.view_port, &mut render_pass).unwrap();

    }
    
    ws.text_atlas.trim();
    
    ws.queue.submit(std::iter::once(command_encoder.finish()));
    
    drawable.present();
  
    Ok(())
  }
  
  fn game_step(&mut self, event_loop: &ActiveEventLoop) {
    // game loop logic is placed here
    
    self.engine.frame_counter += 1;
    
    if !(self.game_state.should_run()) {
      event_loop.exit();
      
      return;
    }
    
    self.engine.inp_dis.dispatch(&mut self.engine.inp_man, &mut self.game_state);
    
    self.game_state.update(&mut self.engine);
    
    self.engine.inp_man.cycle_events();
    
  }
  
}

impl<GS> ApplicationHandler for WindowGame<GS>
where GS: GameState {
  
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {    
    let attributes = Window::default_attributes()
      .with_title("ASCII Engine")
      .with_transparent(false)
      .with_maximized(true)
      .with_active(true);
  
    let window = event_loop.create_window(attributes).unwrap();
    
    let state_result = pollster::block_on(WindowState::new(window.into()));
    match state_result {
      Ok(win_state) => self.window_state = Some(win_state),
      Err(e) => {
        eprintln!("Error initializing GPU: {}", e);
      },
    };
  }
  
  fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed. Stopping program.");
        event_loop.exit();
      },
      WindowEvent::Resized(new_size) => {
        if let Some(ws) = &mut self.window_state {
          ws.resize(new_size, self.engine.db.height, self.engine.db.width);
          self.engine.db.text_changed = true;
        }
      },
      WindowEvent::KeyboardInput{ device_id: _id, event, is_synthetic: synth } => {
        if synth { return; }
        
        self.engine.inp_man.process_winit_key(event);
      },
      WindowEvent::RedrawRequested => {
        
        let new_time = Instant::now();
        let frame_time = new_time - self.current_time;
        self.current_time = new_time;
        
        self.accumulator += frame_time;
        
        while self.accumulator >= self.engine.fixed_time_step {
          self.game_step(event_loop);
          
          self.accumulator -= self.engine.fixed_time_step;
        }
        
        self.game_state.draw(&mut self.engine);
        match self.draw() {
          Ok(_) => {}
          Err(SurfaceError::Lost) => { // If the swapchain is lost (e.g. driver update, monitor unplugged), recreate it
            if let Some(ws) = &mut self.window_state {
              ws.resize(ws.size, self.engine.db.height, self.engine.db.width);
              self.engine.db.text_changed = true;
            }
          },
          Err(SurfaceError::OutOfMemory) => event_loop.exit(), // The system is out of memory, we should quit
          Err(e) => eprintln!("{:?}", e), // All other errors (Outdated, Timeout) should be resolved by the next frame
        }        
        // end of game logic
        
        // Queue a RedrawRequested event.
        if let Some(ws) = &mut self.window_state {
          ws.render(); // calls window.request_redraw()
        }
      }
      _ => (),
    }
  }
  
}
