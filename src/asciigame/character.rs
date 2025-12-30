#[allow(dead_code)]

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8
}

impl From<crossterm::style::Color> for Color {
  fn from(cross_color: crossterm::style::Color) -> Self {
    if let crossterm::style::Color::Rgb{r, g, b} = cross_color {
      return Color{r, g, b};
    } else {
      return Color{r: 0, g: 255, b: 0};
    }
  }
}

impl Into<crossterm::style::Color> for Color {
  fn into(self) -> crossterm::style::Color {
    return crossterm::style::Color::Rgb {
      r: self.r,
      g: self.g,
      b: self.b
    };
  }
}

/// Colored 'character' class. Can be seen as a "pixel" to the WindowBuffer.
#[derive(Clone, Copy)]
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
