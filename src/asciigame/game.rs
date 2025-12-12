#[allow(dead_code)]

use crate::{*};
use crossterm::terminal;
use std::time::{Duration, Instant};

// trait Drawable {
  
// }

/// 
pub struct Game {
  frame_counter: u64,
  framerate: u64,
  start_of_frame: Instant, 
  
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
      framerate: 10,
      start_of_frame: Instant::now(),
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
  
  pub fn wait(&mut self) -> Result<(), RuntimeError> {
    use std::thread;

    let end_of_frame = Instant::now();
    let passed_time = end_of_frame.duration_since(self.start_of_frame);
    
    let target_time = std::time::Duration::from_nanos(1_000_000_000/self.framerate);
    let remaining_time = target_time.saturating_sub(passed_time);
    
    thread::sleep(remaining_time);
    
    self.frame_counter += 1;
    self.start_of_frame += target_time;
    
    Ok(())
  }
  
  pub fn run(&mut self) -> Result<(), RuntimeError> {
    use crossterm::style::{self, Color};
    
    self.start_of_frame = Instant::now();
        
    while self.should_run {
      self.process_events()?;
    
      self.wb.fill_char(Character{
        symbol: ' ',
        color: Color::Rgb{r: 10, g: 10, b: 30},
        color_back: Color::Rgb{r: 30, g: 30, b: 30}
      });
      
      self.wb.set_char(0, 1, Character{
        symbol: char::from_digit((self.frame_counter % 10) as u32, 10).unwrap(),
        color: Color::Rgb{r: 255, g: 255, b:255},
        ..Default::default()
      });
      self.wb.set_char(0, 0, Character{
        symbol: char::from_digit(((self.frame_counter % 100)/10) as u32, 10).unwrap(),
        color: Color::Rgb{r: 255, g: 255, b:255},
        ..Default::default()
      });

      self.wb.draw()?;
      self.wait()?;
    }

    Ok(())
  }
  
}




