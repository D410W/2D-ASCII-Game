#[allow(dead_code)]

use std::sync::Arc;
use wgpu::{*};
use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::Window
};

// AsciiInterface trait
pub trait AsciiInterface: std::io::Write + crossterm::QueueableCommand {}

impl<T> AsciiInterface for T 
where T: std::io::Write + crossterm::QueueableCommand {}

// WindowState. Holds the objects linked to the wgpu API.
pub struct WindowState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
}

impl WindowState {
  pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
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

// WindowWrapper. An abstraction layer between the Game's DrawBuffer and the WindowState object. Equivalent to Stdout.
pub struct WindowWrapper {
  window_state: Option<WindowState>,
}

impl WindowWrapper {
  pub fn new(event_loop: &EventLoop<WindowState>) -> Self {
    Self {
      window_state: None,
    }
  }

}

use winit::window::WindowId;

impl ApplicationHandler for WindowWrapper {

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
  
  fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed; stopping");
        event_loop.exit();
      },
      WindowEvent::RedrawRequested => {
        // Redraw the application.
        //
        // It's preferable for applications that do not render continuously to render in
        // this event rather than in AboutToWait, since rendering in here allows
        // the program to gracefully handle redraws requested by the OS.

        // Draw.

        // Queue a RedrawRequested event.
        //
        // You only need to call this if you've determined that you need to redraw in
        // applications which do not always need to. Applications that redraw continuously
        // can render here instead.
        self.window_state.as_ref().unwrap().window.request_redraw();
      }
      _ => (),
    }
  }
  
}

impl std::io::Write for WindowWrapper {
  fn write(&mut self, _: &[u8]) -> std::result::Result<usize, std::io::Error> { todo!() }
  
  fn flush(&mut self) -> std::result::Result<(), std::io::Error> { todo!() }
}

// TODO later
// impl std::io::Write for WindowWrapper {
// }

// impl crossterm::QueueableCommand for WindowWrapper {
// }

