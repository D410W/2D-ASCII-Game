#[allow(dead_code)]
use crate::{Game, RuntimeError, InputDispatcher, KeyState};

use crossterm::event::{poll, read, Event, KeyEvent, KeyCode};
use std::collections::HashMap;

pub struct InputManager {
  pub key_events: HashMap<KeyCode, KeyState>,
}

impl InputManager {
  pub fn new() -> InputManager {
    InputManager {
      key_events: Default::default(),
    }
  }

  pub fn key(&mut self, key: KeyCode) -> KeyState {
    self.key_events.entry(key).or_insert(KeyState::Unactive).clone()
  }
  
  pub fn process_events(&mut self) -> Result<(), RuntimeError> {
    use crossterm::event::{poll, read, Event, KeyEventKind, KeyCode};
    use std::time::Duration;
    
    for (_key, state) in &mut self.key_events {
      if *state == KeyState::Pressed {
        *state = KeyState::Held;
      } else if *state == KeyState::Released {
        *state = KeyState::Unactive;
      }
    }
    
    while poll(Duration::ZERO)? {
      // It's guaranteed that the 'read()' won't block when the 'poll()' function returns 'true' // match read()? { // Event::FocusGained => println!("FocusGained"), // Event::FocusLost => println!("FocusLost"), // Event::Mouse(event) => println!("{:?}", event), // #[cfg(feature = "bracketed-paste")] // Event::Paste(data) => println!("Pasted {:?}", data), // Event::Resize(width, height) => println!("New size {}x{}", width, height), // }
      if let Event::Key(key_event) = read()? {
        let current_state = self.key_events.entry(key_event.code).or_insert(KeyState::Unactive);
        
        match key_event.kind {
          KeyEventKind::Press => {
            if *current_state == KeyState::Unactive || *current_state == KeyState::Released {
              *current_state = KeyState::Pressed;
            }
          },
          KeyEventKind::Repeat => { *current_state = KeyState::Held; },
          KeyEventKind::Release => { *current_state = KeyState::Released; },
        }
      }
    }
    
    Ok(())
  }
  
}
