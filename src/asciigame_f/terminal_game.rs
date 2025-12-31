use crate::{GameState, Engine, Character};

use crossterm::{terminal, execute, cursor, queue, event::KeyCode,
  style::{SetBackgroundColor, SetForegroundColor, Print},
};
use std::time::{Duration, Instant};
use std::io::{stdout};
use anyhow::Result;

// TerminalWrapper. Holds the main game loop and manages all terminal interactions.
pub struct TerminalGame<GS> 
where GS: GameState {
  // pub window_size: (u16, u16),
  pub start_of_frame: Instant,

  engine: Engine<GS>,
  game_state: GS,
}

impl<GS> TerminalGame<GS>
where GS: GameState {

  pub fn new() -> Result<Self> {
    let (cols, rows) = terminal::size()?;
    
    let mut eng = Engine::<GS>::new((cols as u32, rows as u32));
    let gs = GameState::new(&mut eng);
    
    let s = Self {
      // window_size: (cols, rows),
      start_of_frame: Instant::now(),
    
      engine: eng,
      game_state: gs, 
    };
    
    Ok(s)
  }
  
  fn sync_frame(&mut self) {
    use std::thread;

    let end_of_frame = Instant::now();
    let passed_duration = end_of_frame.duration_since(self.start_of_frame);
    
    let target_duration = std::time::Duration::from_nanos(1_000_000_000/self.engine.framerate);
    let remaining_duration = target_duration.saturating_sub(passed_duration);
    
    thread::sleep(remaining_duration);
    
    self.engine.frame_counter += 1;
    self.start_of_frame += target_duration;
  }
  
  fn draw(&mut self) -> Result<()> {
    use std::io::{Write, stdout};
  
    let mut writing_handle = std::io::BufWriter::new(stdout().lock());
  
    let (db_width, db_height) = self.engine.db.get_size_usize();
  
    for y in 0..db_height {
      for x in 0..db_width {

        let x_u16: u16 = u16::try_from(x)?;
        let y_u16: u16 = u16::try_from(y)?;
        
        let c: &Character = &self.engine.db[(x, y)];
        
        queue!(
          writing_handle,
          cursor::MoveTo(x_u16, y_u16),
          SetForegroundColor(c.color.into()),
          SetBackgroundColor(c.color_back.into()),
          Print(c.symbol),
        )?;
        
      }
        
    }
    
    writing_handle.flush()?;
    
    Ok(())
  }
  
  fn process_events(&mut self) -> Result<()> {
    use crossterm::event::{poll, read, Event, KeyEventKind, KeyCode};
    use std::time::Duration;
    
    self.engine.inp_man.cycle_events();
    
    while poll(Duration::ZERO)? {
      // It's guaranteed that the 'read()' won't block when the 'poll()' function returns 'true' // match read()? { // Event::FocusGained => println!("FocusGained"), // Event::FocusLost => println!("FocusLost"), // Event::Mouse(event) => println!("{:?}", event), // #[cfg(feature = "bracketed-paste")] // Event::Paste(data) => println!("Pasted {:?}", data), // Event::Resize(width, height) => println!("New size {}x{}", width, height), // }
      if let Event::Key(key_event) = read()? {
      
        self.engine.inp_man.process_crossterm_key(key_event);
        
      }
      
    }
    
    Ok(())
  
  }
  
  fn game_loop(&mut self) -> Result<()> {
    self.start_of_frame = Instant::now();
        
    // Running the game
    while self.game_state.should_run() {
      self.process_events()?;
      self.engine.inp_dis.dispatch(&mut self.engine.inp_man, &mut self.game_state);
      
      self.game_state.update(&mut self.engine);
      
      self.game_state.draw(&mut self.engine);
      self.draw()?;

      self.sync_frame();
    }
    
    Ok(())
  }
  
  pub fn run(&mut self) -> Result<()> {
    // 1. Setup
    terminal::enable_raw_mode()?;
    execute!(stdout(),
      terminal::EnterAlternateScreen,
      terminal::Clear(crossterm::terminal::ClearType::All),
      cursor::Hide,
      crossterm::event::PushKeyboardEnhancementFlags(
        crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
      ),
    )?;
    
    let loop_result = self.game_loop();
    
    // 3. Teardown
    let _ = execute!(stdout(),
      crossterm::event::PopKeyboardEnhancementFlags,
      cursor::Show,
      terminal::LeaveAlternateScreen,
    );
    let _ = terminal::disable_raw_mode();

    loop_result
  }

}
