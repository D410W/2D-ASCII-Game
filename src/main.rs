use asciigame::{*};

use winit::event_loop::EventLoop;
use anyhow::Result;

mod gamelogic;
use gamelogic::{*};

fn main() -> Result<()> {
  
  let args: Vec<String> = std::env::args().collect();
    
  let c: char = if args.len() > 1 {
    args[1].chars().next().unwrap().to_ascii_lowercase()
  } else {
    't'
  };
  
  if c != 'w' {
    let mut game = TerminalGame::<Walker>::new();
    if let Ok(mut g) = game {
      if let Err(e) = g.run() {
        game = Err(e);
      } else {
        game = Ok(g);
      }
    }
    
    // 4. Report
    if let Err(e) = game {
      eprintln!("Game Error: {:?}", e);
      return Err(e);
    }
  } else {
    let mut game = WindowGame::<Walker>::new();
    if let Ok(mut g) = game {
      let event_loop = EventLoop::new()?;
      if let Err(e) = event_loop.run_app(&mut g) {
        game = Err(e.into());
      } else {
        game = Ok(g);
      }
    }
    
    // 4. Report
    if let Err(e) = game {
      eprintln!("Game Error: {:?}", e);
      return Err(e);
    }
  }
  
  Ok(())
}
