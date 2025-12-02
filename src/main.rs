use std::io::{stdout};

use crossterm::{
  execute,
  terminal,
  // style,
  // terminal::size,
  // style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
  // ExecutableCommand,
  // event,
  cursor,
};

mod asciigame;
use asciigame::{*};

fn main() -> Result<(), RuntimeError> { // std::io::Result<()> {
  
  // 1. Setup
  terminal::enable_raw_mode()?;
  execute!(stdout(),
    terminal::EnterAlternateScreen,
    terminal::Clear(crossterm::terminal::ClearType::All),
    cursor::Hide,
  )?;
  
  // 2. Run
  let mut game = Game::new()?;
  let game_result = game.run();
  
  // 3. Teardown
  let _ = execute!(stdout(),
    cursor::Show,
    terminal::LeaveAlternateScreen,
  );
  let _ = terminal::disable_raw_mode();
  
  // 4. Report
  if let Err(e) = game_result {
    eprintln!("Game Error: {:?}", e);
    return Err(e);
  }
  
  Ok(())
}
