use crate::character::Character;
use crate::runtime_error::RuntimeError;

use crossterm::{
  queue,
  QueueableCommand,
  style::{SetBackgroundColor, SetForegroundColor, Print}, // Color, ResetColor, 
  cursor,
};

pub struct WindowBuffer<W> {
  width: usize,
  height: usize,
  characters: Vec<Vec<Character>>,
  writing_handle: W,
}

impl<W> WindowBuffer<W> {
  pub fn new(p_width: u32, p_height: u32, p_writing_handle: W) -> Result<Self, RuntimeError> {
    let mut wb = WindowBuffer {
      width: 0usize,
      height: 0usize,
      characters: Vec::new(),
      writing_handle: p_writing_handle,
    };
    
    let w_us : usize = match usize::try_from(p_width) {
      Ok(val) => val,
      Err(e) => return Err(RuntimeError::new(e, "Screen width value out of range for usize" )),
    };
    let h_us : usize = match usize::try_from(p_width) {
      Ok(val) => val,
      Err(e) => return Err(RuntimeError::new(e, "Screen height value out of range for usize" )),
    };
    
    wb.width = w_us;
    wb.height = h_us;
    
    // reserving the used screen space
    wb.characters.push(Vec::<Character>::with_capacity(w_us)); //, Default::default())); // line 0 has size width.
    wb.characters.resize(h_us, wb.characters[0].clone()); // [height] lines equal to line 0.
    
    return Ok(wb);
  }
  
  pub fn draw(self: &Self) -> Result<(), RuntimeError> {

    for i in 0usize..self.height {
      for j in 0usize..self.width {

        let i_u16 : u16 = match u16::try_from(i) {
          Ok(val) => val,
          Err(e) => return Err(RuntimeError::new(e, "Height of WindowBuffer doesn't fit in u16 for terminal output.")),
        };  
        let j_u16 : u16 = match u16::try_from(j) {
          Ok(val) => val,
          Err(e) => return Err(RuntimeError::new(e, "Width of WindowBuffer doesn't fit in u16 for terminal output.")),
        };
        
        let c : &Character = &self.characters[i][j];
        
        queue!(
          self.writing_handle,
          cursor::MoveTo(i_u16, j_u16),
          SetForegroundColor(c.color),
          SetBackgroundColor(c.color_back),
          Print(c.symbol),
        )?;
        
      }
    }
    
    self.writing_handle.flush();
    
    Ok(());
  }
}
