use crate::{Character};

use anyhow::Result;

pub struct DrawBuffer {
  pub width: usize,
  pub height: usize,
  pub characters: Vec<Character>,
  
  pub text_changed: bool,
}

impl std::ops::Index<(usize, usize)> for DrawBuffer {
  type Output = Character;
  fn index(&self, i: (usize, usize)) -> &Character {
    &self.characters[i.1 * self.width as usize + i.0]
  }
}

impl DrawBuffer {
  pub fn new(p_width: usize, p_height: usize) -> Self {
    DrawBuffer {
      width: p_width,
      height: p_height,
      characters: vec![Character::default(); p_width * p_height], // reserving the used screen space
      
      text_changed: true,
    }
  }
  
  pub fn get_size_usize(&mut self) -> (usize, usize) {
    (self.width, self.height)
  }
  
  pub fn resize(&mut self, p_width: usize, p_height: usize) -> &mut Self {
    self.characters.resize(p_width * p_height, Default::default());
    
    self.width = p_width;
    self.height = p_height;
    
    self
  }
  
  pub fn set_char(&mut self, col: usize, row: usize, character: Character) -> &mut Self {
    let char_ref = &mut self.characters[row * self.width + col];
    if *char_ref != character {
      *char_ref = character;
      self.text_changed = true;
    }
    
    self
  }
  
  pub fn clear(&mut self) -> &mut Self {
    for i in 0..(self.height) {
      for j in 0..(self.width) {
        let char_ref = &mut self.characters[i * self.width + j];
        if *char_ref != Default::default() {
          *char_ref = Default::default();
          self.text_changed = true;        
        }
      }
    }
    
    self
  }
  
  pub fn fill_char(&mut self, character: Character) -> &mut Self {
    for i in 0..(self.height) {
      for j in 0..(self.width) {
        let char_ref = &mut self.characters[i * self.width + j];
        if *char_ref != character {
          *char_ref = character;
          self.text_changed = true;        
        }
      }
    }
    
    self
  }
  
}
