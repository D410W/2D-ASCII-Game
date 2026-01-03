#[derive(Clone, Copy, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8
}

impl Color {
  fn min(&mut self) -> u8 {
    self.r.min(self.g.min(self.b))
  }
  
  fn sub_safe(self, value: u8) -> Self {
    Color {
      r: if value >= self.r { 0 } else { self.r - value },
      g: if value >= self.g { 0 } else { self.g - value },
      b: if value >= self.b { 0 } else { self.b - value },
    }
  }

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

impl std::ops::Sub<u8> for Color {
  type Output = Self;
  fn sub(self, value: u8) -> Self {
    Color {
      r: self.r - value,
      g: self.g - value,
      b: self.b - value
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

impl Character {
  pub fn dim_background(&mut self, value: u8) -> Option<Self> {
    if value > self.color_back.min() { return None; }
    
    Some(Character {
      color_back: self.color_back - value,
      ..*self
    })
  }
  
  pub fn dim_background_safe(&mut self, value: u8) -> Self {
    Character {
      color_back: self.color_back.sub_safe(value),
      ..*self
    }
  }
  
  pub fn dim_color(&mut self, value: u8) -> Option<Self> {
    if value > self.color_back.min() { return None; }
    
    Some(Character {
      color: self.color - value,
      ..*self
    })
  }
  
  pub fn dim_color_safe(&mut self, value: u8) -> Self {
    Character {
      color: self.color.sub_safe(value),
      ..*self
    }
  }
  
}

impl Default for Character {
  fn default() -> Self {
    Character { symbol: ' ', color: Color{ r: 255, g: 255, b: 255 }, color_back: Color{ r: 0, g: 0, b: 0 } } // custom default value
  }
}
