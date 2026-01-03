use asciigame::{*};
use crossterm::event::{KeyCode};

use crate::common_structs::{*};
// use crate::gs_funcs::{*};

pub struct Walker {
  pub screen_dims: (usize, usize),
  pub map: Vec<Cell>,
  pub map_seen: Vec<bool>,
  
  pub player_pos: (i32, i32),
  pub player_char: Character,
  
  pub should_run: bool,
}

impl GameState for Walker {
  fn new(ctx: &mut Engine<Self>) -> Self {
    use rand::Rng;
  
    ctx.set_framerate(10);
    
    let (swidth, sheight) = (60, 30);
    ctx.db.resize(swidth, sheight);
  
    let mut walker = Walker{
      screen_dims: (swidth, sheight),
      map: vec![Cell::Void; swidth * sheight],
      map_seen: vec![false; swidth * sheight],
      
      player_pos: (10, 10),
      player_char: Character{ symbol: '@', ..Default::default() },
      
      should_run: true,
    };
    
    let mut rooms: Vec::<(usize, usize, usize, usize)> = vec![]; // room = (pos_x, pos_y, size_x, size_y)
    let num_rooms = 5;
    
    // generating map
    let mut rng = rand::rng();
    for room_index in 0..num_rooms {
    
      let max_room_size = 20;
      let min_room_size = 5;
      
      let mut tries: i32 = 0;
      let mut failed = true;
      
      while failed {
        if tries > 100 {
          println!("failed");
          break;
        }
      
        failed = false;
        tries += 1;
        
        let size = [rng.random_range(min_room_size..max_room_size); 2];
        let pos = (rng.random_range(0..walker.screen_dims.0 - size[0]), rng.random_range(0..walker.screen_dims.1 - size[1]) );
        
        for room in rooms.iter() {
          let new_range_x = (pos.0, pos.0 + size[0]-1);
          let new_range_y = (pos.1, pos.1 + size[1]-1);
          
          let old_range_x = (room.0, room.0 + room.2-1);
          let old_range_y = (room.1, room.1 + room.3-1);
          
          if !((new_range_x.1 < old_range_x.0 ||
               old_range_x.1 < new_range_x.0) ||
               (new_range_y.1 < old_range_y.0 ||
               old_range_y.1 < new_range_y.0) ) {
            failed = true;
            break;
          }
        }
        
        if failed { continue }
        rooms.push( (pos.0, pos.1, size[0], size[1]) );
        
        if room_index == 0 { walker.player_pos = (pos.0 as i32 + 2, pos.1 as i32 + 2); }
      }
      
      
      println!("{} tries", tries);
    }
    
    
    for room in &rooms {
      for x in 0..room.2 as usize {
        walker.map[room.0 + x + room.1 as usize * walker.screen_dims.0] = Cell::Wall;
        walker.map[room.0 + x + (room.1 + room.3-1) as usize * walker.screen_dims.0] = Cell::Wall;
      }
      
      for y in 0..room.3 as usize {
        walker.map[room.0 + (room.1 + y) as usize * walker.screen_dims.0] = Cell::Wall;
        walker.map[room.0 + room.2-1 + (room.1 + y) as usize * walker.screen_dims.0] = Cell::Wall;
      }
      
      for y in 1..room.3-1 as usize {
        for x in 1..room.2-1 as usize {
          walker.map[room.0 + x + (room.1 + y) * walker.screen_dims.0] = Cell::Floor;
        }
      }
    
    }
        
    for iteration in 0..num_rooms {
      let mut tries = 0;
      
      while tries < 20 {
        let room1 = &rooms[iteration];
        let room2 = &rooms[(iteration+1) % num_rooms];
        
        let possible_doors1 = [(room1.0, room1.1 + room1.3/2),
                      (room1.0 + room1.2-1, room1.1 + room1.3/2),
                      (room1.0 + room1.2/2, room1.1),
                      (room1.0 + room1.2/2, room1.1 + room1.3-1)];
        let possible_doors2 = [(room2.0, room2.1 + room2.3/2),
                      (room2.0 + room2.2-1, room2.1 + room2.3/2),
                      (room2.0 + room2.2/2, room2.1),
                      (room2.0 + room2.2/2, room2.1 + room2.3-1)];
        
        let door1 = possible_doors1[rng.random_range(0..4)];
        let door2 = possible_doors2[rng.random_range(0..4)];
        
        let corridor = walker.bfs_to_pos(&mut rng, door1, door2);
        
        if let Some(corridor) = corridor {
          // println!("{:?}", corridor);
        
          for cell in corridor {
            walker.map[cell.0 + cell.1 * walker.screen_dims.0] = Cell::Corridor;
          }
        } else {
          tries += 1;
          continue;
        }
        break;
      }
    }
    
    // binding keys
    ctx.bind(KeyCode::Esc, KeyState::Pressed, |gs| { gs.should_run = false; } );
    
    ctx.bind(KeyCode::Char('w'), KeyState::Down, move |gs| { if gs.is_position_walkable((gs.player_pos.0, gs.player_pos.1 - 1)) { gs.player_pos.1 -= 1; } } );
    ctx.bind(KeyCode::Char('s'), KeyState::Down, move |gs| { if gs.is_position_walkable((gs.player_pos.0, gs.player_pos.1 + 1)) { gs.player_pos.1 += 1; } } );
    ctx.bind(KeyCode::Char('d'), KeyState::Down, move |gs| { if gs.is_position_walkable((gs.player_pos.0 + 1, gs.player_pos.1)) { gs.player_pos.0 += 1; } } );
    ctx.bind(KeyCode::Char('a'), KeyState::Down, move |gs| { if gs.is_position_walkable((gs.player_pos.0 - 1, gs.player_pos.1)) { gs.player_pos.0 -= 1; } } );
    
    walker
  }
  
  fn update(&mut self, ctx: &mut Engine<Walker>) {
    
    if ctx.frame_counter > 20 { /* self.should_run = false; */ }
    
  }
  
  fn draw(&mut self, ctx: &mut Engine<Walker>) {

    let (width, height) = ctx.db.get_size_usize();
    
    for y in 0..=height - 1 {
      for x in 0..=width - 1 {
        let cell_char;
        if self.has_lineofsight( (self.player_pos.0 as usize, self.player_pos.1 as usize), (x, y), 10, false) {
          cell_char = self.get_cell_char(self.map[y * width + x]);
          self.map_seen[y * width + x] = true;
        } else {
          if self.map_seen[y * width + x] {
            cell_char = self.get_cell_char(self.map[y * width + x]).dim_background_safe(20).dim_color_safe(30);
          } else {
            cell_char = Default::default();
          }
        };
        
        ctx.db.set_char(x, y, cell_char);
      }
    }
    
    ctx.db.set_char(self.player_pos.0 as usize, self.player_pos.1 as usize, self.player_char);
    
  }
  
  fn should_run(&mut self) -> bool {
    self.should_run
  }
  
}
