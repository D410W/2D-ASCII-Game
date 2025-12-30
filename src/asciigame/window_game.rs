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
      
      font_system,
      text_renderer,
      text_atlas,
      swash_cache: glyphon::SwashCache::new(),
      view_port,
      buffer,
      metrics: glyphon::Metrics{ font_size: 32.0, line_height: 32.0 },
      
      span_cache: Vec::with_capacity(2000),
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
  pub start_of_frame: Instant,
  
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
      
      start_of_frame: Instant::now(),
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
  
    // glyphon preparation
    if self.engine.db.text_changed {
      
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
      
    }
    
    let window_width = ws.config.width as f32;
    let window_height = ws.config.height as f32;
    
    let left_offset = ( window_width - ws.metrics.font_size as f32 * self.engine.db.width as f32 ) / 2.0;
    let top_offset = ( window_height - ws.metrics.font_size as f32 * self.engine.db.height as f32 ) / 2.0;
    
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
      
      ws.text_renderer.render(&ws.text_atlas, &ws.view_port, &mut render_pass).unwrap();

    }
    
    ws.text_atlas.trim();
    
    ws.queue.submit(std::iter::once(command_encoder.finish()));
    
    drawable.present();
  
    Ok(())
  }
  
  fn game_step(&mut self, event_loop: &ActiveEventLoop) {
    // game loop logic is placed here
    
    // Missing:
    //   sync_frame,
    
    self.start_of_frame = Instant::now();
    
    self.engine.frame_counter += 1;

    if !(self.game_state.should_run()) {
      event_loop.exit();
      
      return;
    }
    
    self.engine.inp_dis.dispatch(&mut self.engine.inp_man, &mut self.game_state);
    
    self.game_state.update(&mut self.engine);
    
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
    // self.engine.db.clear();
    
    self.engine.inp_man.cycle_events();

    // self.sync_frame()?; // TODO
    
  }
  
}

impl<GS> ApplicationHandler for WindowGame<GS>
where GS: GameState {
  
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {    
    let attributes = Window::default_attributes()
      .with_title("ASCII Engine")
      .with_transparent(false)
      // .with_fullscreen(None) // Some(winit::window::Fullscreen::Borderless(None)))
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
        
        self.game_step(event_loop);
        
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
