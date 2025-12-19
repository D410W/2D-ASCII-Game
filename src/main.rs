mod asciigame;
use asciigame::{*};

use crossterm::event::{KeyCode};
use anyhow::Result;

struct Walker {
  pos: (u16, u16),
  player: Character,
  
  should_run: bool,
}

impl Walker {
  fn initialize<W>(ctx: &mut Game<Self, W>) -> Self
  where W: AsciiInterface {
    ctx.framerate = 10;
  
    let walker = Walker{
      pos: (10, 10),
      player: Character{ symbol: '@', ..Default::default() },
      should_run: true,
    };
    
    return walker;
  }
}

impl<W> GameState<W> for Walker
where W: AsciiInterface {
  fn update(&mut self, ctx: &mut Game<Walker, W>) {
    
    ctx.bind(KeyCode::Esc, KeyState::Pressed, |gs| { gs.should_run = false; } );
    
    ctx.bind(KeyCode::Char('w'), KeyState::Down, |gs| { gs.pos.0 -= 1; } );
    ctx.bind(KeyCode::Char('s'), KeyState::Down, |gs| { gs.pos.0 += 1; } );
    ctx.bind(KeyCode::Char('d'), KeyState::Down, |gs| { gs.pos.1 += 1; } );
    ctx.bind(KeyCode::Char('a'), KeyState::Down, |gs| { gs.pos.1 -= 1; } );
    
  }
  
  fn draw(&mut self, ctx: &mut Game<Walker, W>) {
    ctx.db.set_char(self.pos.0, self.pos.1, self.player);
    
  }
  
  fn should_run(&mut self) -> bool {
    self.should_run
  }
}

fn main() -> Result<()> { // std::io::Result<()> {
  
  let mut game_result : Result<()> = Ok(());
  
  // let game = match Game::<Walker, WindowWrapper>::new(WindowWrapper::new()) {
  let game = match Game::<Walker, std::io::Stdout>::new(std::io::stdout()) {
    Ok(g) => Some(g),
    Err(e) => {game_result = Err(e); None},
  };
  

  if let Some(mut g) = game {
    // let mut w_state = Walker::initialize::<WindowWrapper>(&mut g);
    let mut w_state = Walker::initialize::<std::io::Stdout>(&mut g);
    game_result = g.run(&mut w_state);
  }
  
  // 4. Report
  if let Err(e) = game_result {
    eprintln!("Game Error: {:?}", e);
    return Err(e);
  }
  
  Ok(())
}
