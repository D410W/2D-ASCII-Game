#[allow(dead_code)]

use crate::{GameState, InputManager};

use std::collections::HashMap;
use crossterm::event::KeyCode;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum KeyState {
  Pressed,  // User just pressed the key in this frame.
  Held,     // User is holding the key for more than one frame.
  Released, // User just released the key in this frame.
  Unactive, // Key isn't begin used.
  
  PressedAndReleased, // Pressed and Released in the same frame. 
  Down, // Pressed or Held or PressedAndReleased
}

pub struct InputDispatcher<S> {
  // A map from a 'Key' to a function that modifies 'T'
  bindings: HashMap<(KeyCode, KeyState), Box<dyn FnMut(&mut S)>>,
}

impl<S> InputDispatcher<S> {
  pub fn new() -> Self {
    Self {
      bindings: HashMap::new(),
    }
  }
  
  /// Subscribe a function to a key.
  pub fn bind<F>(&mut self, key: KeyCode, key_state: KeyState, callback: F) -> ()
  where F: FnMut(&mut S) + 'static {
    self.bindings.insert((key, key_state), Box::new(callback));
  }
  
  pub fn dispatch(&mut self, manager: &mut InputManager, target: &mut S) {
    for ((key_code, target_state), callback) in &mut self.bindings {
      let current_state = manager.key(*key_code);
      
      let is_triggered = match target_state {
        KeyState::Down => current_state != KeyState::Unactive && current_state != KeyState::Released,
        
        KeyState::Released => current_state == KeyState::Released || current_state == KeyState::PressedAndReleased,
        KeyState::Pressed => current_state == KeyState::Pressed || current_state == KeyState::PressedAndReleased,
        
        _ => current_state == *target_state,
      };
        
      if is_triggered {
        callback(target);
      }
    }
  }
  
  pub fn dispatch_single(&mut self, key_code: KeyCode, target: &mut S) { // No KeyState, since we assume it was a 'Pressed' and 'Released' in the same frame.
    if let Some(callback) = self.bindings.get_mut(&(key_code, KeyState::Pressed)) {
      callback(target);
    }
  }
  
}
