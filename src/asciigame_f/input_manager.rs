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

  pub fn get_key(&mut self, key: crossterm::event::KeyCode) -> KeyState {
    self.key_events.entry(key).or_insert(KeyState::Unactive).clone()
  }
  
  /// should be called at the end of a frame
  pub fn cycle_events(&mut self) {
  
    for state in &mut self.key_events.values_mut() {
      if *state == KeyState::Pressed {
        *state = KeyState::Held;
      } else if *state == KeyState::Released || *state == KeyState::PressedAndReleased {
        *state = KeyState::Unactive;
      }
    }
    
  }
  
  pub fn process_crossterm_key(&mut self, key_event: crossterm::event::KeyEvent) {
  
    // println!("{:?}, {:?}", key_event.code, key_event.modifiers);

    let event_code = normalize_crossterm_key(&key_event);

    let current_state = self.key_events.entry(event_code).or_insert(KeyState::Unactive);
    
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
  
  pub fn process_winit_key(&mut self, key_event: winit::event::KeyEvent) {
    use winit::keyboard::{Key, NamedKey};
    use crossterm::event::KeyCode as CKeyCode; // Crossterm

    let code = if let winit::keyboard::PhysicalKey::Code(keycode) = key_event.physical_key {
      translate_winit_physical(keycode)
    } else {
      CKeyCode::Null
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

use winit::keyboard::KeyCode as WCode;
use crossterm::event::{KeyCode as CCode, ModifierKeyCode};

pub fn translate_winit_physical(w_code: WCode) -> CCode {
  match w_code {
    // --- Letters ---
    WCode::KeyA => CCode::Char('a'),
    WCode::KeyB => CCode::Char('b'),
    WCode::KeyC => CCode::Char('c'),
    WCode::KeyD => CCode::Char('d'),
    WCode::KeyE => CCode::Char('e'),
    WCode::KeyF => CCode::Char('f'),
    WCode::KeyG => CCode::Char('g'),
    WCode::KeyH => CCode::Char('h'),
    WCode::KeyI => CCode::Char('i'),
    WCode::KeyJ => CCode::Char('j'),
    WCode::KeyK => CCode::Char('k'),
    WCode::KeyL => CCode::Char('l'),
    WCode::KeyM => CCode::Char('m'),
    WCode::KeyN => CCode::Char('n'),
    WCode::KeyO => CCode::Char('o'),
    WCode::KeyP => CCode::Char('p'),
    WCode::KeyQ => CCode::Char('q'),
    WCode::KeyR => CCode::Char('r'),
    WCode::KeyS => CCode::Char('s'),
    WCode::KeyT => CCode::Char('t'),
    WCode::KeyU => CCode::Char('u'),
    WCode::KeyV => CCode::Char('v'),
    WCode::KeyW => CCode::Char('w'),
    WCode::KeyX => CCode::Char('x'),
    WCode::KeyY => CCode::Char('y'),
    WCode::KeyZ => CCode::Char('z'),

    // --- Numbers (Top Row) ---
    WCode::Digit1 => CCode::Char('1'),
    WCode::Digit2 => CCode::Char('2'),
    WCode::Digit3 => CCode::Char('3'),
    WCode::Digit4 => CCode::Char('4'),
    WCode::Digit5 => CCode::Char('5'),
    WCode::Digit6 => CCode::Char('6'),
    WCode::Digit7 => CCode::Char('7'),
    WCode::Digit8 => CCode::Char('8'),
    WCode::Digit9 => CCode::Char('9'),
    WCode::Digit0 => CCode::Char('0'),

    // --- Functional Keys ---
    WCode::Space => CCode::Char(' '),
    WCode::Enter => CCode::Enter,
    WCode::Escape => CCode::Esc,
    WCode::Backspace => CCode::Backspace,
    WCode::Tab => CCode::Tab,
    WCode::Delete => CCode::Delete,
    WCode::Insert => CCode::Insert,
    WCode::Home => CCode::Home,
    WCode::End => CCode::End,
    WCode::PageUp => CCode::PageUp,
    WCode::PageDown => CCode::PageDown,

    // --- Arrows ---
    WCode::ArrowUp => CCode::Up,
    WCode::ArrowDown => CCode::Down,
    WCode::ArrowLeft => CCode::Left,
    WCode::ArrowRight => CCode::Right,

    // --- Modifiers ---
    WCode::ShiftLeft => CCode::Modifier(ModifierKeyCode::LeftShift),
    WCode::ShiftRight => CCode::Modifier(ModifierKeyCode::RightShift),
    WCode::ControlLeft => CCode::Modifier(ModifierKeyCode::LeftControl),
    WCode::ControlRight => CCode::Modifier(ModifierKeyCode::RightControl),
    WCode::AltLeft => CCode::Modifier(ModifierKeyCode::LeftAlt),
    WCode::AltRight => CCode::Modifier(ModifierKeyCode::RightAlt),

    // --- Function Keys ---
    WCode::F1 => CCode::F(1),
    WCode::F2 => CCode::F(2),
    WCode::F3 => CCode::F(3),
    WCode::F4 => CCode::F(4),
    WCode::F5 => CCode::F(5),
    WCode::F6 => CCode::F(6),
    WCode::F7 => CCode::F(7),
    WCode::F8 => CCode::F(8),
    WCode::F9 => CCode::F(9),
    WCode::F10 => CCode::F(10),
    WCode::F11 => CCode::F(11),
    WCode::F12 => CCode::F(12),

    // Fallback for weird keys
    _ => CCode::Null,
  }
}

use crossterm::event::{KeyEvent, KeyModifiers};

pub fn normalize_crossterm_key(event: &KeyEvent) -> CCode {
  // If Shift is NOT held, just return the code as-is
  if !event.modifiers.contains(KeyModifiers::SHIFT) {
    return event.code;
  }

  match event.code {
    // 1. Handle letters (A -> a)
    CCode::Char(c) if c.is_ascii_uppercase() => {
      CCode::Char(c.to_ascii_lowercase())
    },

    // 2. Handle top row numbers (! -> 1)
    CCode::Char('!') => CCode::Char('1'),
    CCode::Char('@') => CCode::Char('2'),
    CCode::Char('#') => CCode::Char('3'),
    CCode::Char('$') => CCode::Char('4'),
    CCode::Char('%') => CCode::Char('5'),
    CCode::Char('^') => CCode::Char('6'),
    CCode::Char('&') => CCode::Char('7'),
    CCode::Char('*') => CCode::Char('8'),
    CCode::Char('(') => CCode::Char('9'),
    CCode::Char(')') => CCode::Char('0'),

    // 3. Handle common symbols
    CCode::Char('_') => CCode::Char('-'),
    CCode::Char('+') => CCode::Char('='),
    CCode::Char('{') => CCode::Char('['),
    CCode::Char('}') => CCode::Char(']'),
    CCode::Char('|') => CCode::Char('\\'),
    CCode::Char(':') => CCode::Char(';'),
    CCode::Char('"') => CCode::Char('\''),
    CCode::Char('<') => CCode::Char(','),
    CCode::Char('>') => CCode::Char('.'),
    CCode::Char('?') => CCode::Char('/'),
    
    // 4. Other codes
    CCode::BackTab => CCode::Tab,

    // If it's not one of these, just return the original (e.g. Backspace + Shift)
    _ => event.code,
  }
}
