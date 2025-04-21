use std::time::Duration;

pub struct Timer {
    // start: [Option<Instant>; 64],
    elapsed: [Duration; 64],
}

impl Timer {
    pub const fn new() -> Self {
        Timer {
            // start: [None; 64],
            elapsed: [Duration::from_secs(0); 64],
        }
    }
    pub fn clear(&mut self, n: usize) {
        self.elapsed[n] = Duration::from_secs(0);
    }
    pub fn start(&mut self, _n: usize) {
        // self.start[n] = Some(Instant::now());
    }
    pub fn stop(&mut self, _n: usize) {
        // if let Some(start_time) = self.start[n] {
        //     let elapsed_time = start_time.elapsed();
        //     self.elapsed[n] += elapsed_time;
        // }
    }
    pub fn read(&self, n: usize) -> Duration {
        self.elapsed[n]
    }
}
