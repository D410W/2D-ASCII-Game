#[allow(dead_code)]

use crate::{WindowBuffer, InputManager, RuntimeError, GameState, InputDispatcher, KeyState};

use crossterm::{terminal, execute, cursor, event::KeyCode};
use std::time::{Duration, Instant};
use std::io::{stdout};

// trait Drawable {
  
// }

/// 
pub struct Game<S> {
  pub frame_counter: u64,
  pub framerate: u64,
  pub window_size: (u16, u16),
  start_of_frame: Instant,
  
  pub wb: WindowBuffer<std::io::Stdout>,
  inp_man: InputManager,
  pub inp_dis: InputDispatcher<S>,
  
}

impl<S: GameState> Game<S> {
  pub fn new() -> Result<Self, RuntimeError> {
    use std::io::stdout;

    let (term_w, term_h): (u16, u16) = terminal::size()?;
  
    let game = Game::<S> { 
      frame_counter: 0,
      framerate: 1,
      window_size: (term_w, term_h),
      start_of_frame: Instant::now(),
      wb: WindowBuffer::new(term_w, term_h, stdout())?,
      inp_man: InputManager::new(),
      inp_dis: InputDispatcher::<S>::new(),
    };
    
    Ok(game)
  }
  
  // pub fn key_pressed(&mut self, kc: crossterm::event::KeyCode) -> bool {
  //   self.inp_man.key(kc)
  // }
  
  fn sync_frame(&mut self) -> Result<(), RuntimeError> {
    use std::thread;

    let end_of_frame = Instant::now();
    let passed_duration = end_of_frame.duration_since(self.start_of_frame);
    
    let target_duration = std::time::Duration::from_nanos(1_000_000_000/self.framerate);
    let remaining_duration = target_duration.saturating_sub(passed_duration);
    
    thread::sleep(remaining_duration);
    
    self.frame_counter += 1;
    self.start_of_frame += target_duration;
    
    Ok(())
  }
  
  fn game_loop(&mut self, state: &mut S) -> Result<(), RuntimeError> {
    self.start_of_frame = Instant::now();
    
    // 2. Run
    while state.should_run() {
      state.update(self)?;
      
      self.wb.clear();
      
      state.draw(self)?;

      self.wb.draw()?;
      self.sync_frame()?;
      self.inp_man.process_events()?;
      self.inp_dis.dispatch(&mut self.inp_man, state);
    }
    
    Ok(())
  }
  
  pub fn run(&mut self, state: &mut S) -> Result<(), RuntimeError> {
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
    
    let loop_result = self.game_loop(state);
    
    // 3. Teardown
    let _ = execute!(stdout(),
      crossterm::event::PopKeyboardEnhancementFlags,
      cursor::Show,
      terminal::LeaveAlternateScreen,
    );
    let _ = terminal::disable_raw_mode();

    return loop_result;
  }
  
  pub fn bind<F>(&mut self, key: KeyCode, key_state: KeyState, callback: F) -> ()
  where F: FnMut(&mut S) + 'static {
    self.inp_dis.bind(key, key_state, callback);
  }
  
}
