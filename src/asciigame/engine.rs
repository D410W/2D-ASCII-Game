#[allow(dead_code)]

use crate::{/*WindowWrapper, AsciiInterface,*/ DrawBuffer, InputManager, GameState, InputDispatcher, KeyState};

use crossterm::{terminal, execute, cursor, event::KeyCode};
use std::time::{Duration, Instant};
use std::io::{stdout};
use anyhow::Result;

// trait Drawable {
  
// }

/// 
pub struct Engine<GS> { // <GameState, Wrapper>
  pub framerate: u64,
  pub frame_counter: u64,
  
  pub db: DrawBuffer, // DrawBuffer<std::io::Stdout>,
  pub inp_man: InputManager,
  pub inp_dis: InputDispatcher<GS>,
  
}

impl<GS> Engine<GS>
where GS: GameState {
  pub fn new(screen_size: (u32, u32)) -> Self {
    // use std::io::stdout;

    let (term_w, term_h) = screen_size;
  
    let game = Engine::<GS> {
      framerate: 10,
      frame_counter: 0,

      db: DrawBuffer::new(term_w, term_h),
      inp_man: InputManager::new(),
      inp_dis: InputDispatcher::<GS>::new(),
    };
    
    game
  }
  
  // pub fn key_pressed(&mut self, kc: crossterm::event::KeyCode) -> bool {
  //   self.inp_man.key(kc)
  // }
  
  pub fn bind<F>(&mut self, key: KeyCode, key_state: KeyState, callback: F) -> ()
  where F: FnMut(&mut GS) + 'static {
    self.inp_dis.bind(key, key_state, callback);
  }
  
}
