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
  // window stuff
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  window: Arc<Window>,
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

    return Ok(WindowState{
      surface: surface,
      device: device,
      queue: queue,
      config: surface_config,
      size: size,
      window: window,
    });
  }
  
  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      
      self.surface.configure(&self.device, &self.config);
    }
  }
  
  pub fn render(&mut self) {
    self.window.request_redraw();
  }

}

// WindowGame. An abstraction layer between the User and the Engine. Is responsible for the game loop, rendering and input redirection.
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
  
    let color_attachment = RenderPassColorAttachment{
      view: &image_view,
      resolve_target: None,
      depth_slice: None,
      ops: Operations{
        load: LoadOp::Clear(Color{
          r: 0.25,
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
    };
  
    {
    
      let _render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
      
      // FUTURE CODE: 
      // _render_pass.set_pipeline(...);
      // _render_pass.draw(...);
    }
    
    ws.queue.submit(std::iter::once(command_encoder.finish()));
    
    drawable.present();
  
    Ok(()) // TODO
  }
  
}

impl<GS> ApplicationHandler for WindowGame<GS>
where GS: GameState {
  
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let window = event_loop.create_window(Window::default_attributes()).unwrap();
    
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
          ws.resize(new_size);
        }
      },
      WindowEvent::KeyboardInput{ device_id: _id, event, is_synthetic: _synth } => {
        
      },
      WindowEvent::RedrawRequested => {
        // Game loop logic is placed here
        
        self.start_of_frame = Instant::now();


        if !(self.game_state.should_run()) {
          // TODO
        }
        
        // self.process_events()?;
        self.engine.inp_dis.dispatch(&mut self.engine.inp_man, &mut self.game_state);
        
        self.game_state.update(&mut self.engine);
        
        self.game_state.draw(&mut self.engine);
        match self.draw() {
          Ok(_) => {}
          Err(SurfaceError::Lost) => { // If the swapchain is lost (e.g. driver update, monitor unplugged), recreate it
              if let Some(ws) = &mut self.window_state {
                  ws.resize(ws.size);
              }
          },
          Err(SurfaceError::OutOfMemory) => event_loop.exit(), // The system is out of memory, we should quit
          Err(e) => eprintln!("{:?}", e), // All other errors (Outdated, Timeout) should be resolved by the next frame
        }
        self.engine.db.clear();
        
        self.engine.inp_man.cycle_events();

        // self.sync_frame()?;
        
        // End of game logic
        // Queue a RedrawRequested event.
        if let Some(ws) = &mut self.window_state {
          ws.render(); // calls window.request_redraw()
        }
      }
      _ => (),
    }
  }
  
}
