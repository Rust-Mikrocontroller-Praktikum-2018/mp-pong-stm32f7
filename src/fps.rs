use stm32f7::system_clock;

pub struct FpsCounter {
    last_print: usize,
    frames: usize,
    pub last_fps: usize,
    pub output_enabled: bool,
}

pub fn init() -> FpsCounter {
    let current_time = system_clock::ticks();
    FpsCounter {
        last_print: current_time,
        frames: 0,
        last_fps: 0,
        output_enabled: false,
    }
}

impl FpsCounter {
    pub fn count_frame(&mut self) {
        let current_ticks = system_clock::ticks();
        self.frames += 1;
        let diff_since_last_print = current_ticks - self.last_print;

        if diff_since_last_print > 1000 {
            self.last_fps = self.frames;
            self.last_print = current_ticks;
            if self.output_enabled {
                hprintln!("FPS: {}", self.frames);
            }
            self.frames = 0;
        }
    }
}
