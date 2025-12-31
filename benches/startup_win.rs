use asciigame::{*};

use winit::event_loop::EventLoop;
use anyhow::Result;

struct Walker {
  should_run: bool,
}

impl Walker {
  
}

impl GameState for Walker {
  fn new(ctx: &mut Engine<Self>) -> Self {
    ctx.set_framerate(1000);
    
    let (swidth, sheight) = (30, 15);
    
    ctx.db.resize(swidth, sheight);
  
    Walker{
      should_run: true,
    }
  }
  
  fn update(&mut self, ctx: &mut Engine<Walker>) {
    
    if ctx.framerate > 1 { self.should_run = false; }
    
  }
  
  fn draw(&mut self, _ctx: &mut Engine<Walker>) {
    
  }
  
  fn should_run(&mut self) -> bool {
    self.should_run
  }
}

fn startup_win(c: &mut criterion::Criterion) {
  c.bench_function("window_game_startup", |b| {
      b.iter(|| {
      std::hint::black_box({
        let mut game = WindowGame::<Walker>::new();
        if let Ok(mut g) = game {
          let event_loop = EventLoop::new()?;
          if let Err(e) = event_loop.run_app(&mut g) {
            game = Err(e.into());
          } else {
            game = Ok(g);
          }
        }
        
        // 4. Report
        if let Err(e) = game {
          eprintln!("Game Error: {:?}", e);
          return Err(e);
        }
        
        Ok(())
      })
    })
  });
}

fn startup_term(c: &mut criterion::Criterion) {
  c.bench_function("terminal_game_startup", |b| {
      b.iter({
      std::hint::black_box(|| {
        let mut game = TerminalGame::<Walker>::new();
        if let Ok(mut g) = game {
          if let Err(e) = g.run() {
            game = Err(e);
          } else {
            game = Ok(g);
          }
        }
        
        // 4. Report
        if let Err(e) = game {
          eprintln!("Game Error: {:?}", e);
          return Err(e);
        }
        
        Ok(())
      })
    })
  });
}

criterion::criterion_group!(benches, startup_win, startup_term);
criterion::criterion_main!(benches);
