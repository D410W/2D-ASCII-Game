use asciigame::{*};
use crossterm::event::{KeyCode};

use crate::common_structs::{*};
// use crate::gs_funcs::{*};

pub struct Walker {
  pub screen_dims: (usize, usize),
  pub map_dims: (usize, usize),
  pub map: Vec<Cell>,
  pub map_seen: Vec<bool>,
  
  pub player_pos: (i32, i32),
  pub player_char: Character,
  
  pub should_run: bool,
}

impl GameState for Walker {
  fn new(ctx: &mut Engine<Self>) -> Self {
    use rand::prelude::{*};
  
    ctx.set_framerate(10);
    
    let (screen_width, screen_height) = (60, 30);
    let (map_width, map_height) = (100, 100);
    ctx.db.resize(screen_width, screen_height);
  
    let mut walker = Walker{
      screen_dims: (screen_width, screen_height),
      map_dims: (map_width, map_height),
      map: vec![Cell::Floor; map_width * map_height],
      map_seen: vec![false; map_width * map_height],
      
      player_pos: (5, 5),
      player_char: Character{ symbol: '@', color: Color{r: 100, g: 100, b: 255}, ..Default::default() },
      
      should_run: true,
    };
    
    // generating map
    let rooms_area = (20usize, 20usize, 60usize, 60usize);
    
    for y in rooms_area.1..(rooms_area.3 + rooms_area.1) {
      for x in rooms_area.0..(rooms_area.2 + rooms_area.0) {
        walker.map[x + y * walker.map_dims.0] = Cell::Void;
      }
    }
    
    let mut rooms: Vec::<(usize, usize, usize, usize)> = vec![]; // room = (pos_x, pos_y, size_x, size_y)
    let num_rooms = 5;
    
    let mut rng = rand::rng();
    // making rooms
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
        let pos = (rng.random_range(0..(rooms_area.2 - size[0])), rng.random_range(0..(rooms_area.3 - size[1])) );
        
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
        
        println!("room size: {:?}; room pos: {:?}", size, pos);
        
        // if room_index == 0 { walker.player_pos = (pos.0 as i32 + 2, pos.1 as i32 + 2); }
      }
      
      
      println!("{} tries", tries);
    }
    
    
    for room in &rooms {
      for x in 0..room.2 as usize {
        walker.map[rooms_area.0 + room.0 + x + (rooms_area.1 + room.1 as usize) * walker.map_dims.0] = Cell::Wall;
        walker.map[rooms_area.0 + room.0 + x + (rooms_area.1 + (room.1 + room.3-1) as usize) * walker.map_dims.0] = Cell::Wall;
      }
      
      for y in 0..room.3 as usize {
        walker.map[rooms_area.0 + room.0 + (rooms_area.1 + (room.1 + y) as usize) * walker.map_dims.0] = Cell::Wall;
        walker.map[rooms_area.0 + room.0 + room.2-1 + (rooms_area.1 + (room.1 + y) as usize) * walker.map_dims.0] = Cell::Wall;
      }
      
      for y in 1..room.3-1 as usize {
        for x in 1..room.2-1 as usize {
          walker.map[rooms_area.0 + room.0 + x + (rooms_area.1 + room.1 + y) * walker.map_dims.0] = Cell::Floor;
        }
      }
    
    }
    
    let mut last_door = (0, 0);
    
    // corridors between rooms
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
        
        // println!("begin: {:?}, end: {:?}", door1, door2);
        
        let corridor = walker.bfs_to_pos(&mut rng, (rooms_area.0 + door1.0, rooms_area.1 + door1.1),
                                                   (rooms_area.0 + door2.0, rooms_area.1 + door2.1));
        
        if let Some(corridor) = corridor {
          // println!("{:?}", corridor);
        
          for cell in corridor {
            walker.map[cell.0 + cell.1 * walker.map_dims.0] = Cell::Corridor;
          }
          
          last_door = door2;
          break;
        } else {
          tries += 1;
          continue;
        }
      }
    }
    
    // entrnace
    let mut tries = 0;
    
    while tries < 20 {
      
      let door = (rooms_area.0 + last_door.0, rooms_area.1 + last_door.1);
      let dungeon_entrance = (rooms_area.0, rooms_area.1 + rooms_area.3/2);
      
      println!("entrance: {:?}, door: {:?}", dungeon_entrance, door);
      
      let corridor = walker.bfs_to_pos(&mut rng, door, dungeon_entrance);
      
      if let Some(corridor) = corridor {
        for cell in corridor {
          walker.map[cell.0 + cell.1 * walker.map_dims.0] = Cell::Corridor;
        }
      } else {
        tries += 1;
        continue;
      }
      break;
    }
    
    println!("tries to make entrance: {}", tries);
    
    // binding keys
    let noclip = false;
    
    ctx.bind(KeyCode::Esc, KeyState::Pressed, |gs| { gs.should_run = false; } );
    
    ctx.bind(KeyCode::Char('w'), KeyState::Down, move |gs| { if noclip || gs.is_position_walkable((gs.player_pos.0, gs.player_pos.1 - 1)) { gs.player_pos.1 -= 1; } } );
    ctx.bind(KeyCode::Char('s'), KeyState::Down, move |gs| { if noclip || gs.is_position_walkable((gs.player_pos.0, gs.player_pos.1 + 1)) { gs.player_pos.1 += 1; } } );
    ctx.bind(KeyCode::Char('d'), KeyState::Down, move |gs| { if noclip || gs.is_position_walkable((gs.player_pos.0 + 1, gs.player_pos.1)) { gs.player_pos.0 += 1; } } );
    ctx.bind(KeyCode::Char('a'), KeyState::Down, move |gs| { if noclip || gs.is_position_walkable((gs.player_pos.0 - 1, gs.player_pos.1)) { gs.player_pos.0 -= 1; } } );
    
    walker
  }
  
  fn update(&mut self, ctx: &mut Engine<Walker>) {
    
    if ctx.frame_counter > 20 { /* self.should_run = false; */ }
    
  }
  
  fn draw(&mut self, ctx: &mut Engine<Walker>) {
    let corner_x = self.player_pos.0 - (self.screen_dims.0/2) as i32;
    let corner_y = self.player_pos.1 - (self.screen_dims.1/2) as i32;
    let (width, height) = self.screen_dims;
    
    for y in 0..=height - 1 {
      for x in 0..=width - 1 {
        let (phys_x, phys_y) = (corner_x + x as i32, corner_y + y as i32);
      
        let cell = self.get_cell_ref(phys_x, phys_y);
        let cell_char: Character;
        
        match cell {
          None => { cell_char = Default::default(); }
          Some(cref) => {
            let char_pos = (phys_x as usize, phys_y as usize);
            if self.has_lineofsight( (self.player_pos.0 as usize, self.player_pos.1 as usize), (char_pos.0, char_pos.1), 10, false) {
              cell_char = self.get_cell_char(*cref);
              self.map_seen[char_pos.0 + char_pos.1 * self.map_dims.0] = true;
            } else {
              if self.map_seen[char_pos.0 + char_pos.1 * self.map_dims.0] {
                cell_char = self.get_cell_char(*cref).dim_background_safe(20).dim_color_safe(30);
              } else {
                cell_char = Default::default();
              }
            };
          },
        }
        
        ctx.db.set_char(x, y, cell_char);
      }
    }
    
    ctx.db.set_char(self.screen_dims.0/2, self.screen_dims.1/2, self.player_char);
    
  }
  
  fn should_run(&mut self) -> bool {
    self.should_run
  }
  
}
