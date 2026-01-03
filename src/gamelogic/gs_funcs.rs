use asciigame::{*};

use crate::common_structs::{*};
use crate::core::{*};

use std::collections::{HashMap, VecDeque};

impl Walker {
  pub fn is_position_walkable(&mut self, position: (i32, i32)) -> bool {
    if position.0 < 0 || position.1 < 0 {
      false
    } else if position.0 >= self.screen_dims.0 as i32 || position.1 >= self.screen_dims.1 as i32 {
      false
    } else {
      let pos = &self.map[position.1 as usize * self.screen_dims.0 + position.0 as usize];
      
      *pos == Cell::Floor || *pos == Cell::Corridor
    }
  }
  
  pub fn get_cell(&self, x: usize, y: usize) -> Cell {
    self.map[x + y * self.screen_dims.0]
  }
  
  pub fn get_cell_char(&mut self, cell_type: Cell) -> Character {
    match cell_type {
      Cell::Void => Default::default(),
      Cell::Wall => Character{
        symbol: '#',
        color: Color{r: 100, g: 100, b: 100},
        color_back: Color{r: 20, g: 20, b: 20}
      },
      Cell::Floor => Character{
        symbol: ',',
        color: Color{r: 50, g: 50, b: 50},
        color_back: Color{r: 1, g: 1, b: 1}
      },
      Cell::Corridor => Character{
        symbol: '.',
        color: Color{r: 50, g: 50, b: 50},
        color_back: Color{r: 1, g: 1, b: 1}
      },
    }
  }
  
  pub fn dfs_to_pos<T: rand::Rng>(&mut self, rng: &mut T, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    self.dfs_or_bfs(rng, start, end, true)
  }

  pub fn bfs_to_pos<T: rand::Rng>(&mut self, rng: &mut T, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    self.dfs_or_bfs(rng, start, end, false)
  }
  
  pub fn dfs_or_bfs<T: rand::Rng>(&mut self, rng: &mut T, start: (usize, usize), end: (usize, usize), is_dfs: bool) -> Option<Vec<(usize, usize)>> {
    use rand::prelude::{*};
    
    let mut dirs = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    
    let mut came_from = HashMap::<(usize, usize), (usize, usize)>::new();
    let mut to_look = VecDeque::new();
    let mut visited = vec![false; self.screen_dims.0 * self.screen_dims.1];
    to_look.push_back( start );
    
    while let Some(current) = if is_dfs { to_look.pop_back() } else { to_look.pop_front() } {
      
      if current == end {
        // println!("did it");
        let mut path = Vec::new();
        let mut x = current;
        
        while x != start {
          // println!("{:?}", x);
          path.push(x);
          x = *came_from.get(&x).unwrap();
        }
        path.push(start);
        
        // println!("finished it");
        
        return Some(path);
      }
      
      if current.0 >= self.screen_dims.0 ||
         current.1 >= self.screen_dims.1 { continue; }
      
      if self.get_cell(current.0, current.1) != Cell::Void &&
         self.get_cell(current.0, current.1) != Cell::Corridor &&
         current != start { continue; }
      
      let visit_bool = &mut visited[current.0 + current.1 * self.screen_dims.0];
      if *visit_bool {
        continue;
      } else {
        *visit_bool = true;
      }
      
      // println!("{:?}", current);
      
      dirs.shuffle(rng);
      
      for dir in dirs {
        let pos_x = current.0 as i32 + dir.0;
        let pos_y = current.1 as i32 + dir.1;
        
        if pos_x >= 0 && pos_y >= 0 &&
           pos_x < self.screen_dims.0 as i32 && pos_y < self.screen_dims.0 as i32 {
          let new_pos = (pos_x as usize, pos_y as usize);
          
          if !came_from.contains_key(&new_pos) {
            came_from.insert(new_pos, current);
            to_look.push_back( new_pos );
          }
        }
      }
      
    }
    
    None
  }
  
  pub fn has_lineofsight(
      &self,
      uorigin: (usize, usize),
      utarget: (usize, usize),
      max_dist: usize,
      check_corners: bool,
  ) -> bool {
    let origin = (uorigin.0 as i32, uorigin.1 as i32);
    let target = (utarget.0 as i32, utarget.1 as i32);
    
    let dx = target.0 - origin.0;
    let dy = target.1 - origin.1;
    
    let dmax = dx.abs().max(dy.abs());
    let dmin = dx.abs().min(dy.abs());
    
    let sx = if dx > 0 { 1 } else { -1 };
    let sy = if dy > 0 { 1 } else { -1 };
    
    let (mut oldx, mut oldy) = (0, 0);
    
    let rounding_error = dmax / 2;
    
    for i in 1..=dmax {
    
      let (x, y);
      
      if dx.abs() > dy.abs() {
        x = i * sx;
        y = (i * sy * dmin + rounding_error*sy) / dmax;
      } else {
        y = i * sy;
        x = (i * sx * dmin + rounding_error*sx) / dmax;
      }
      
      let physical_x = x + origin.0;
      let physical_y = y + origin.1;
      
      if physical_x < 0 || physical_y < 0 {
        return false;
      }
      
      if (x,y) != (oldx, oldy) && check_corners && (x,y) != target {
        
        return_if!(
          self.get_cell((oldx + origin.0) as usize, physical_y as usize) == Cell::Wall ||
          self.get_cell((oldx + origin.0) as usize, physical_y as usize) == Cell::Void,
          false);
        return_if!(
          self.get_cell(physical_x as usize, (oldy + origin.1) as usize) == Cell::Wall ||
          self.get_cell(physical_x as usize, (oldy + origin.1) as usize) == Cell::Void,
          false);
        
      }
      
      (oldx, oldy) = (x, y);
            
      return_if!(
        (self.get_cell(physical_x as usize, physical_y as usize) == Cell::Wall ||
        self.get_cell(physical_x as usize, physical_y as usize) == Cell::Void) &&
        (physical_x, physical_y) != target,
        false);
    }
    
    true
  }
  
}
