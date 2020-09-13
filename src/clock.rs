use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Clock {
  duration: Duration,
  instant: Instant,
}

impl Clock {
  pub fn new(hz: u64) -> Self {
    Clock {
      duration: Duration::from_micros(1000000 / hz),
      instant: Instant::now(),
    }
  }

  pub fn tick(&mut self) -> bool {
    let elapsed = self.instant.elapsed();
    if elapsed >= self.duration {
      self.instant = Instant::now();
      return true;
    }
    return false;
  }

  fn time_to_next_tick(&self) -> Duration {
    let elapsed = self.instant.elapsed();

    if elapsed > self.duration {
      Duration::from_micros(0)
    } else {
      self.duration - elapsed
    }
  }

  pub fn sleep_until_next_tick(clocks: Vec<&Clock>) {
    match clocks.iter().map(|c| c.time_to_next_tick()).min() {
      Some(duration) => {
        if cfg!(windows) {
          if duration.as_millis() > 1 {
            std::thread::sleep(Duration::from_millis(duration.as_millis() as u64 - 1));  
          }
        } else {
          std::thread::sleep(duration);
        }
      },
      _ => {}
    };
  }
}
