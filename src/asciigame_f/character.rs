#[derive(Clone, Copy, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8
}

impl From<crossterm::style::Color> for Color {
  fn from(cross_color: crossterm::style::Color) -> Self {
    if let crossterm::style::Color::Rgb{r, g, b} = cross_color {
      Color{r, g, b}
    } else {
      Color{r: 0, g: 255, b: 0}
    }
  }
}

impl From<Color> for crossterm::style::Color {
  fn from(val: Color) -> Self {
    crossterm::style::Color::Rgb {
      r: val.r,
      g: val.g,
      b: val.b
    }
  }
}

/// Colored 'character' class. Can be seen as a "pixel" to the WindowBuffer.
#[derive(Clone, Copy, PartialEq)]
pub struct Character {
  pub symbol: char,
  pub color: Color,
  pub color_back: Color,
}

impl Default for Character {
  fn default() -> Self {
    Character { symbol: ' ', color: Color{ r: 255, g: 255, b: 255 }, color_back: Color{ r: 0, g: 0, b: 0 } } // custom default value
  }
}
