#[allow(dead_code)]
use crate::{InputDispatcher, KeyState};

use crossterm::event::{poll, read, Event};

use std::collections::HashMap;
use anyhow::Result;

pub struct InputManager {
  pub key_events: HashMap<crossterm::event::KeyCode, crate::KeyState>,
}

impl InputManager {
  pub fn new() -> InputManager {
    InputManager {
      key_events: Default::default(),
    }
  }

  pub fn key(&mut self, key: crossterm::event::KeyCode) -> KeyState {
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
  
  pub fn process_crossterm_key(&mut self, key_event: crossterm::event::KeyEvent) -> () {

    let current_state = self.key_events.entry(key_event.code).or_insert(KeyState::Unactive);
    
    match key_event.kind {
      crossterm::event::KeyEventKind::Press => {
        if *current_state == KeyState::Unactive || *current_state == KeyState::Released {
          *current_state = KeyState::Pressed;
        }
      },
      crossterm::event::KeyEventKind::Repeat => { *current_state = KeyState::Held; },
      crossterm::event::KeyEventKind::Release => {
        if *current_state == KeyState::Pressed {
          *current_state = KeyState::PressedAndReleased;
        } else {
          *current_state = KeyState::Released;
        }
      },
    }
    
  }
  
  pub fn process_winit_key(&mut self, key_event: winit::event::KeyEvent) -> () { // TODO
    use winit::keyboard::{Key, NamedKey};
    use crossterm::event::KeyCode as CKeyCode; // Crossterm

    let code = match key_event.logical_key {
      Key::Character(s) => {
        match s.chars().next().map(|c| CKeyCode::Char(c)) {
          Some(keycode) => keycode,
          None => CKeyCode::Null,
        }
      },
      Key::Named(named_key) => {
        match named_key {
          NamedKey::Escape => CKeyCode::Esc,
          NamedKey::Enter => CKeyCode::Enter,
          NamedKey::Backspace => CKeyCode::Backspace,
          NamedKey::Tab => CKeyCode::Tab,
          NamedKey::Space => CKeyCode::Char(' '),
          NamedKey::Delete => CKeyCode::Delete,
          
          NamedKey::ArrowUp => CKeyCode::Up,
          NamedKey::ArrowDown => CKeyCode::Down,
          NamedKey::ArrowLeft => CKeyCode::Left,
          NamedKey::ArrowRight => CKeyCode::Right,
          
          NamedKey::F1 => CKeyCode::F(1),
          NamedKey::F2 => CKeyCode::F(2),
          // ... add F3-F12 ...
          _ => CKeyCode::Null,
        }
      },
      _ => CKeyCode::Null,
    };

    let modifiers = crossterm::event::KeyModifiers::empty(); // shift, control, alt, etc. TODO

    let kind = match key_event.state { // press, release or repeat
      winit::event::ElementState::Pressed => {
        match key_event.repeat {
          false => crossterm::event::KeyEventKind::Press,
          true => crossterm::event::KeyEventKind::Repeat,
        }
      },
      winit::event::ElementState::Released => crossterm::event::KeyEventKind::Release,
    };
    
    let state = crossterm::event::KeyEventState::empty(); // signifies capslock, numlock, keypad, etc.
    
    let translated = crossterm::event::KeyEvent{
      code,
      modifiers,
      kind,
      state,
    };
    
    self.process_crossterm_key(translated);
    
  }
  
}
