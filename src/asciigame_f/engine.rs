use crate::{/*WindowWrapper, AsciiInterface,*/ DrawBuffer, InputManager, GameState, InputDispatcher, KeyState};

use crossterm::{terminal, execute, cursor, event::KeyCode};
use std::time::{Duration, Instant};
use std::io::{stdout};
use anyhow::Result;

/// Base engige struct. Controls input redirection and stores the ASCII screen.
pub struct Engine<GS> { // <GameState, Wrapper>
  pub framerate: u64,
  pub fixed_time_step: Duration,
  pub frame_counter: u64,
  
  pub db: DrawBuffer, // DrawBuffer<std::io::Stdout>,
  pub inp_man: InputManager,
  pub inp_dis: InputDispatcher<GS>,
}

impl<GS> Engine<GS>
where GS: GameState {
  pub fn new(screen_size: (usize, usize)) -> Self {
    // use std::io::stdout;

    let (term_w, term_h) = screen_size;
  
    Engine::<GS> {
      framerate: 10,
      fixed_time_step: Duration::from_secs_f32(1.0 / 10.0),
      frame_counter: 0,

      db: DrawBuffer::new(term_w, term_h),
      inp_man: InputManager::new(),
      inp_dis: InputDispatcher::<GS>::new(),
    }
  }
  
  pub fn set_framerate(&mut self, new_fps: u64) {
    self.framerate = new_fps;
    self.fixed_time_step = Duration::from_secs_f32(1.0 / new_fps as f32);
  }
  
  pub fn bind<F>(&mut self, key: KeyCode, key_state: KeyState, callback: F)
  where F: FnMut(&mut GS) + 'static {
    self.inp_dis.bind(key, key_state, callback);
  }
  
}
