#[allow(dead_code)]

use crate::character::Character;
use crate::runtime_error::RuntimeError;

use crossterm::{
  queue,
  // QueueableCommand,
  style::{SetBackgroundColor, SetForegroundColor, Print}, // Color, ResetColor, 
  cursor,
};

pub struct WindowBuffer<W> {
  width: u16,
  height: u16,
  characters: Vec<Vec<Character>>,
  writing_handle: W,
}

impl<W: std::io::Write + crossterm::QueueableCommand> WindowBuffer<W> {
  pub fn new(p_width: u16, p_height: u16, p_writing_handle: W) -> Result<Self, RuntimeError> {
    let mut wb = WindowBuffer {
      width: p_width,
      height: p_height,
      characters: Vec::new(),
      writing_handle: p_writing_handle,
    };
    
    let w_us : usize = p_width.into();
    let h_us : usize = p_height.into();
    
    // reserving the used screen space
    wb.characters = vec![vec![Character::default(); w_us]; h_us];
    
    return Ok(wb);
  }
  
  pub fn draw(self: &mut Self) -> Result<(), RuntimeError> {
  
    for i in 0usize..self.characters.len() {
      for j in 0usize..self.characters[0].len() {

        let i_u16 : u16 = match u16::try_from(i) {
          Ok(val) => val,
          Err(e) => return Err(RuntimeError::TryFromIntError(e, "Height of WindowBuffer doesn't fit in u16 for terminal output.")),
        };  
        let j_u16 : u16 = match u16::try_from(j) {
          Ok(val) => val,
          Err(e) => return Err(RuntimeError::TryFromIntError(e, "Width of WindowBuffer doesn't fit in u16 for terminal output.")),
        };
        
        let c : &Character = &self.characters[i][j];
        
        queue!(
          self.writing_handle,
          cursor::MoveTo(j_u16, i_u16),
          SetForegroundColor(c.color),
          SetBackgroundColor(c.color_back),
          Print(c.symbol),
        )?;
        
      }
        
    }
    
    self.flush()?;
    self.clear();
    
    return Ok(());
  }
  
  pub fn set_char(self: &mut Self, row: u16, col: u16, character: Character) -> () {
    self.characters[usize::from(row)][usize::from(col)] = character;
  }
  
  pub fn clear(self: &mut Self) -> () {
    for i in 0..usize::from(self.height) {
      for j in 0..usize::from(self.width) {
        self.characters[i][j] = Default::default();
      }
    }
  }
  
  pub fn fill_char(self: &mut Self, character: Character) -> () {
    for i in 0..usize::from(self.height) {
      for j in 0..usize::from(self.width) {
        self.characters[i][j] = character.clone();
      }
    }
  }
  
  pub fn flush(self: &mut Self) -> Result<(), RuntimeError> {
    match self.writing_handle.flush() {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }
  
}
