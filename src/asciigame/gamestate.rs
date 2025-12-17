#[allow(dead_code)]
use crate::runtime_error::{*};
use crate::game::{*};

/// A Generic struct that implements the basic 'update' and 'draw' game-logic methods.
/// It requires the most basic methods so that the Game struct can utilize it. 
pub trait GameState: Sized {
  fn update(&mut self, ctx: &mut Game<Self>) -> Result<(), RuntimeError>;
  
  fn draw(&mut self, ctx: &mut Game<Self>) -> Result<(), RuntimeError>;
  
  fn should_run(&mut self) -> bool;
}
