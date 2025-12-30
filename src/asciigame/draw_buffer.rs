#[allow(dead_code)]

use crate::{Character};

use anyhow::Result;

pub struct DrawBuffer {
  pub width: u32,
  pub height: u32,
  pub characters: Vec<Vec<Character>>,
  
  pub text_changed: bool,
}

impl DrawBuffer {
  pub fn new(p_width: u32, p_height: u32) -> Self {
    let mut wb = DrawBuffer {
      width: p_width,
      height: p_height,
      characters: Vec::new(),
      
      text_changed: true,
    };
    
    let w_us = p_width as usize;
    let h_us = p_height as usize;
    
    // reserving the used screen space
    wb.characters = vec![vec![Character::default(); w_us]; h_us];
    
    return wb;
  }
  
  pub fn resize(&mut self, p_width: u32, p_height: u32) -> &mut Self {
    let w_us = p_width as usize;
    let h_us = p_height as usize;
    
    for line in &mut self.characters {
      line.resize(w_us, Default::default());
    }
    
    self.characters.resize(h_us, vec![Character::default(); w_us]);
    
    self.width = p_width;
    self.height = p_height;
    
    self
  }
  
  pub fn set_char(&mut self, row: u32, col: u32, character: Character) -> &mut Self {
    self.characters[row as usize][col as usize] = character;
    self.text_changed = true;
    
    self
  }
  
  pub fn clear(&mut self) -> &mut Self {
    for i in 0..(self.height as usize) {
      for j in 0..(self.width as usize) {
        self.characters[i][j] = Default::default();
      }
    }
    self.text_changed = true;
    
    self
  }
  
  pub fn fill_char(&mut self, character: Character) -> &mut Self {
    for i in 0..(self.height as usize) {
      for j in 0..(self.width as usize) {
        self.characters[i][j] = character.clone();
      }
    }
    self.text_changed = true;
    
    self
  }
  
  // pub fn flush(self: &mut Self) -> Result<()> {
  //   match self.writing_handle.flush() {
  //     Ok(_) => Ok(()),
  //     Err(e) => Err(e.into()),
  //   }
  // }
  
}
