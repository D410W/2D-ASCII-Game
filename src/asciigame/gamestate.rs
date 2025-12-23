#[allow(dead_code)]
use crate::{Engine};

/// A Generic struct that implements the basic 'update' and 'draw' game-logic methods.
/// It requires the most basic methods so that the Game struct can utilize it. 
pub trait GameState: Sized {
  fn new(ctx: &mut Engine<Self>) -> Self;

  fn update(&mut self, ctx: &mut Engine<Self>) -> ();
  
  fn draw(&mut self, ctx: &mut Engine<Self>) -> ();
  
  fn should_run(&mut self) -> bool;
}
