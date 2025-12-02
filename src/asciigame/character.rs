#[allow(dead_code)]

use crossterm::{
  style::{Color}, //, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

/// Colored 'character' class. Can be seen as a "pixel" to the WindowBuffer.
#[derive(Clone)]
pub struct Character {
  pub symbol: char,
  pub color: Color,
  pub color_back: Color,
}

impl Default for Character {
  fn default() -> Self {
    Character { symbol: ' ', color: Color::White, color_back: Color::Black } // custom default value
  }
}
