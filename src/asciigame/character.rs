use crossterm::{
  style::{Color}, //, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

#[derive(Clone)]
pub struct Character {
  symbol: char,
  color: Color,
  color_back: Color,
}

impl Default for Character {
  fn default() -> Self {
    Character { symbol: ' ', color: Color::White, color_back: Color::Black } // custom default value
  }
}
