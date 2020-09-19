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

  pub fn consume_ticks(&mut self) -> u128 {
    let elapsed = self.instant.elapsed();
      
    let num_times_to_run = elapsed.as_micros() / self.duration.as_micros();

    if num_times_to_run > 0 {
      self.instant = Instant::now() - Duration::from_micros((elapsed.as_micros() % self.duration.as_micros()) as u64)
    }

    num_times_to_run
  }
}

