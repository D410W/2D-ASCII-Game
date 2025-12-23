#[allow(dead_code)]

use crate::{Character};

use anyhow::Result;

pub struct DrawBuffer {
  pub width: u32,
  pub height: u32,
  pub characters: Vec<Vec<Character>>,
  // writing_handle: W,
}

impl DrawBuffer {
  pub fn new(p_width: u32, p_height: u32) -> Self {
    let mut wb = DrawBuffer {
      width: p_width,
      height: p_height,
      characters: Vec::new(),
      // writing_handle: p_writing_handle,
    };
    
    let w_us : usize = p_width as usize;
    let h_us : usize = p_height as usize;
    
    // reserving the used screen space
    wb.characters = vec![vec![Character::default(); w_us]; h_us];
    
    return wb;
  }
  
  pub fn set_char(self: &mut Self, row: u32, col: u32, character: Character) -> () {
    self.characters[row as usize][col as usize] = character;
  }
  
  pub fn clear(self: &mut Self) -> () {
    for i in 0..(self.height as usize) {
      for j in 0..(self.width as usize) {
        self.characters[i][j] = Default::default();
      }
    }
  }
  
  pub fn fill_char(self: &mut Self, character: Character) -> () {
    for i in 0..(self.height as usize) {
      for j in 0..(self.width as usize) {
        self.characters[i][j] = character.clone();
      }
    }
  }
  
  // pub fn flush(self: &mut Self) -> Result<()> {
  //   match self.writing_handle.flush() {
  //     Ok(_) => Ok(()),
  //     Err(e) => Err(e.into()),
  //   }
  // }
  
}
