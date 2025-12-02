#[allow(dead_code)]

use crate::{*};
use crossterm::terminal;

// trait Drawable {
  
// }

/// 
pub struct Game {
  frame_counter: u64,
  should_run: bool,
  
  wb: WindowBuffer<std::io::Stdout>,

  
  // drawable_objs: Vec<Box<Drawable>> //!< List of objects that should be rendered.
  
}

impl Game {
  pub fn new() -> Result<Self, RuntimeError> {
    use std::io::stdout;

    let (term_w, term_h): (u16, u16) = terminal::size()?;
  
    let game = Game { 
      frame_counter: 0,
      should_run: true,
      wb: WindowBuffer::new(term_w, term_h, stdout())?,
    };
    
    Ok(game)
  }
  
  pub fn process_events(&mut self) -> Result<(), RuntimeError> {
    use crossterm::event::{poll, read, Event, KeyCode};
    use std::time::Duration;
    if poll(Duration::ZERO)? {
      // It's guaranteed that the 'read()' won't block when the 'poll()'  // function returns 'true' // match read()? { // Event::FocusGained => println!("FocusGained"), // Event::FocusLost => println!("FocusLost"), // Event::Mouse(event) => println!("{:?}", event), // #[cfg(feature = "bracketed-paste")] // Event::Paste(data) => println!("Pasted {:?}", data), // Event::Resize(width, height) => println!("New size {}x{}", width, height), // }
      if let Event::Key(key_event) = read()? {
        if key_event.code == KeyCode::Esc {
          self.should_run = false;
        }
      }
    } else {
      // Timeout expired and no `Event` is available
    }
    
    Ok(())
  }

  pub fn update(&mut self) -> Result<(), RuntimeError> {
    Ok(())
  }
  
  pub fn draw(&self) -> Result<(), RuntimeError> {
    Ok(())
  }
  
  pub fn run(&mut self) -> Result<(), RuntimeError> {
    use std::time;
    use std::thread;
    use crossterm::style;
    
    let mut time_before = time::SystemTime::now();
    
    while self.should_run {
      self.process_events()?;
    
      let val: u8 = ((self.frame_counter*10)%255).try_into()?;
    
      self.wb.fill_char(Character{
          symbol: 'S',
          color: style::Color::Rgb{r: 255-val, g: 30, b: 30},
          color_back: style::Color::Rgb{r: 255-val, g: 30, b: 30}
      });

      self.wb.draw()?;
      
      let passed_time = time_before.elapsed()?; 
      let remaining_time = passed_time.saturating_sub(std::time::Duration::from_millis(1000/60));
      time_before += passed_time;
      
      self.frame_counter = self.frame_counter + 1;
      
      thread::sleep(remaining_time);
    }

    Ok(())
  }
  
}




