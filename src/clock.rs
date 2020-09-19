use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Clock {
  duration: Duration,
  instant: Option<Instant>,
}

impl Clock {
  pub const fn new(hz: u64) -> Self {
    Clock {
      duration: Duration::from_micros(1000000 / hz),
      instant: Option::None,
    }
  }

  pub fn start_clock(&mut self) {
    self.instant = Option::Some(Instant::now());
  }

  pub fn get_instant(&mut self) -> Instant {
    self.instant.unwrap()
  }

  pub fn set_instant(&mut self, instant: Instant) {
    println!("Reset instant");
    self.instant = Option::Some(instant);
  }

  pub fn run<F>(&mut self, mut run_fn: F) where F: FnMut() {
    // if self.tick() {
      let elapsed = self.get_instant().elapsed();
      
      let num_times_to_run = elapsed.as_micros() / self.duration.as_micros();
      println!("Running {} ticks - {} elapsed, {} tick rate", num_times_to_run, elapsed.as_micros(), self.duration.as_micros());
      for _ in 0..num_times_to_run {
        run_fn();
      }

      if num_times_to_run > 0 {
        self.set_instant(Instant::now());
      }

    // }
  }

  // fn time_to_next_tick(&mut self) -> Duration {
  //   let elapsed = self.get_instant().elapsed();

  //   if elapsed > self.duration {
  //     Duration::from_micros(0)
  //   } else {
  //     self.duration - elapsed
  //   }
  // }

  // pub fn duration_until_next_tick(clocks: Vec<&mut Clock>) -> Duration {
  //   match clocks.iter().map(|c| c.time_to_next_tick()).min() {
  //     Some(duration) => {
  //       if cfg!(windows) {
  //         if duration.as_millis() > 1 {
  //           Duration::from_millis(duration.as_millis() as u64 - 1)
  //         } else {
  //           Duration::from_micros(0)
  //         }
  //       } else {
  //         duration
  //       }
  //     }
  //     _ => Duration::from_micros(0)
  //   }
  // }

  // pub fn sleep_until_next_tick(clocks: Vec<&Clock>) {
  //   match clocks.iter().map(|c| c.time_to_next_tick()).min() {
  //     Some(duration) => {
  //       if cfg!(windows) {
  //         if duration.as_millis() > 1 {
  //           std::thread::sleep(Duration::from_millis(duration.as_millis() as u64 - 1));
  //         }
  //       } else {
  //         std::thread::sleep(duration);
  //       }
  //     }
  //     _ => {}
  //   };
  // }
}

