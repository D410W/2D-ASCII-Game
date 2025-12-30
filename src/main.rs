mod asciigame;
use asciigame::{*};

use crossterm::event::{KeyCode};
use winit::event_loop::EventLoop;
use anyhow::Result;

struct Walker {
  pos: (u32, u32),
  player: Character,
  
  should_run: bool,
}

impl Walker {
  
}

impl GameState for Walker {
  fn new(ctx: &mut Engine<Self>) -> Self {
    ctx.set_framerate(10);
    
    let (swidth, sheight) = (30, 15);
    
    ctx.db.resize(swidth, sheight);
  
    let walker = Walker{
      pos: (10, 10),
      player: Character{ symbol: '@', ..Default::default() },
      should_run: true,
    };
    
    ctx.bind(KeyCode::Esc, KeyState::Pressed, |gs| { gs.should_run = false; } );
    
    ctx.bind(KeyCode::Char('w'), KeyState::Down, move |gs| { if gs.pos.1 > 0 { gs.pos.1 -= 1; } } );
    ctx.bind(KeyCode::Char('s'), KeyState::Down, move |gs| { if gs.pos.1 < sheight - 1 { gs.pos.1 += 1; } } );
    ctx.bind(KeyCode::Char('d'), KeyState::Down, move |gs| { if gs.pos.0 < swidth - 1 { gs.pos.0 += 1; } } );
    ctx.bind(KeyCode::Char('a'), KeyState::Down, move |gs| { if gs.pos.0 > 0 { gs.pos.0 -= 1; } } );
    
    return walker;
  }
  
  fn update(&mut self, ctx: &mut Engine<Walker>) {
    
    if ctx.frame_counter > 20 { /* self.should_run = false; */ }
    
  }
  
  fn draw(&mut self, ctx: &mut Engine<Walker>) {

    let (width, height) = ctx.db.get_size();
    
    let lettrs: Vec<char> = (('0' as u32)..=('z' as u32)).map(|n| char::from_u32(n).unwrap()).collect();
    
    for w in 1..=width {
      for h in 1..=height {
      
        let bcolor = ( (h+w) as u8 % 2 ) * 100;
      
        ctx.db.set_char(w-1, h-1, Character{
          symbol: lettrs[(w-1) as usize],
          color: Color{r: 100, g: 100, b: 100},
          color_back: Color{r: bcolor, g: 0, b: 0},
        });
        
      }
    }
    
    ctx.db.set_char(self.pos.0, self.pos.1, self.player);
    
  }
  
  fn should_run(&mut self) -> bool {
    self.should_run
  }
}

fn main() -> Result<()> {
  
  let args: Vec<String> = std::env::args().collect();
    
  let c: char;
  if args.len() > 1 {
    c = args[1].chars().nth(0).unwrap().to_ascii_lowercase()
  } else {
    c = 't';
  }
  
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
