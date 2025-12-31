use crate::{Character};

use anyhow::Result;

pub struct DrawBuffer {
  pub width: u32,
  pub height: u32,
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
  pub fn new(p_width: u32, p_height: u32) -> Self {
    let w_us = p_width as usize;
    let h_us = p_height as usize;
    
    DrawBuffer {
      width: p_width,
      height: p_height,
      characters: vec![Character::default(); w_us * h_us], // reserving the used screen space
      
      text_changed: true,
    }
  }
  
  pub fn get_size_usize(&mut self) -> (usize, usize) {
    (self.width as usize, self.height as usize)
  }
  
  pub fn resize(&mut self, p_width: u32, p_height: u32) -> &mut Self {
    let w_us = p_width as usize;
    let h_us = p_height as usize;
    
    self.characters.resize(w_us * h_us, Default::default());
    
    self.width = p_width;
    self.height = p_height;
    
    self
  }
  
  pub fn set_char(&mut self, col: u32, row: u32, character: Character) -> &mut Self {
    let char_ref = &mut self.characters[row as usize * self.width as usize + col as usize];
    if *char_ref != character {
      *char_ref = character;
      self.text_changed = true;
    }
    
    self
  }
  
  pub fn clear(&mut self) -> &mut Self {
    for i in 0..(self.height as usize) {
      for j in 0..(self.width as usize) {
        let char_ref = &mut self.characters[i * self.width as usize + j];
        if *char_ref != Default::default() {
          *char_ref = Default::default();
          self.text_changed = true;        
        }
      }
    }
    
    self
  }
  
  pub fn fill_char(&mut self, character: Character) -> &mut Self {
    for i in 0..(self.height as usize) {
      for j in 0..(self.width as usize) {
        let char_ref = &mut self.characters[i * self.width as usize + j];
        if *char_ref != character {
          *char_ref = character;
          self.text_changed = true;        
        }
      }
    }
    
    self
  }
  
}
