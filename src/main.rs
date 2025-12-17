mod asciigame;
use asciigame::{*};

use crossterm::event::{KeyCode};

struct Walker {
  pos: (u16, u16),
  player: Character,
  
  should_run: bool,
}

impl Walker {
  fn initialize(ctx: &mut Game<Walker>) -> Walker {
    ctx.framerate = 10;
  
    let w = Walker{
      pos: (10, 10),
      player: Character{ symbol: '@', ..Default::default() },
      should_run: true,
    };
    
    return w;
  }
}

impl GameState for Walker {
  fn update(&mut self, ctx: &mut Game<Walker>) -> Result<(), RuntimeError> {
    
    ctx.bind(KeyCode::Esc, KeyState::Pressed, |gs| { gs.should_run = false; } );
    
    ctx.bind(KeyCode::Char('w'), KeyState::Down, |gs| { gs.pos.0 -= 1; } );
    ctx.bind(KeyCode::Char('s'), KeyState::Down, |gs| { gs.pos.0 += 1; } );
    ctx.bind(KeyCode::Char('d'), KeyState::Down, |gs| { gs.pos.1 += 1; } );
    ctx.bind(KeyCode::Char('a'), KeyState::Down, |gs| { gs.pos.1 -= 1; } );
  
    Ok(())
  }
  
  fn draw(&mut self, ctx: &mut Game<Walker>) -> Result<(), RuntimeError> {
    ctx.wb.set_char(self.pos.0, self.pos.1, self.player);
    
    Ok(())
  }
  
  fn should_run(&mut self) -> bool {
    self.should_run
  }
}

fn main() -> Result<(), RuntimeError> { // std::io::Result<()> {
  
  let mut game_result : Result<(), RuntimeError> = Ok(());
  
  let mut game = match Game::new() {
    Ok(g) => Some(g),
    Err(e) => {game_result = Err(e); None},
  };
  

  if let Some(mut g) = game.take() {
    let mut w_state = Walker::initialize(&mut g);
    game_result = g.run(&mut w_state);
  }
  
  // 4. Report
  if let Err(e) = game_result {
    eprintln!("Game Error: {:?}", e);
    return Err(e);
  }
  
  Ok(())
}
