#[allow(dead_code)]
use crate::game::{*};

/// A Generic struct that implements the basic 'update' and 'draw' game-logic methods.
/// It requires the most basic methods so that the Game struct can utilize it. 
pub trait GameState<W>: Sized {
  fn update(&mut self, ctx: &mut Game<Self, W>) -> ();
  
  fn draw(&mut self, ctx: &mut Game<Self, W>) -> ();
  
  fn should_run(&mut self) -> bool;
}
