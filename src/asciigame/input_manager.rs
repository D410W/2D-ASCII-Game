#[allow(dead_code)]
use crate::{InputDispatcher, KeyState};

use crossterm::event::{poll, read, Event, KeyEvent, KeyEventKind, KeyCode};
use std::collections::HashMap;
use anyhow::Result;

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
  
  pub fn cycle_events(&mut self) {
  
    for (_key, state) in &mut self.key_events {
      if *state == KeyState::Pressed {
        *state = KeyState::Held;
      } else if *state == KeyState::Released || *state == KeyState::PressedAndReleased {
        *state = KeyState::Unactive;
      }
    }
    
  }
  
  pub fn process_key(&mut self, key_event: KeyEvent) -> () {

    let current_state = self.key_events.entry(key_event.code).or_insert(KeyState::Unactive);
    
    match key_event.kind {
      KeyEventKind::Press => {
        if *current_state == KeyState::Unactive || *current_state == KeyState::Released {
          *current_state = KeyState::Pressed;
        }
      },
      KeyEventKind::Repeat => { *current_state = KeyState::Held; },
      KeyEventKind::Release => {
        if *current_state == KeyState::Pressed {
          *current_state = KeyState::PressedAndReleased;
        } else {
          *current_state = KeyState::Released;
        }
      },
    }
    
  }
  
}
