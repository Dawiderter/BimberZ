use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct FPSCounter {
    last_update_time: Instant,
    frames_elapsed: u32,
}

impl FPSCounter {
    pub fn new() -> Self {
        Self { last_update_time: Instant::now(), frames_elapsed: 0 }
    }

    pub fn fps(&self) -> f32 {
        self.frames_elapsed as f32 / (Instant::now() - self.last_update_time).as_secs_f32()
    }

    pub fn curr_duration(&self) -> Duration {
        Instant::now() - self.last_update_time
    }

    pub fn reset(&mut self) {
        self.last_update_time = Instant::now();
        self.frames_elapsed = 0;
    } 

    pub fn advance_frame(&mut self) {
        self.frames_elapsed += 1;
    }
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self::new()
    }
}