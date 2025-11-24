use std::io::{stdout};

// use crossterm::{
//   execute,
//   style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
//   // ExecutableCommand,
//   event,
// };

mod asciigame;
use asciigame::{*};

fn main() -> std::io::Result<()> {
    
  // execute!(
  //   stdout(),
  //   SetForegroundColor(Color::Blue),
  //   SetBackgroundColor(Color::Red),
  //   Print("Styled text here."),
  //   ResetColor
  // )?;
  
  let mut wb = WindowBuffer::new(20,20, stdout());

  return Ok(());
}
