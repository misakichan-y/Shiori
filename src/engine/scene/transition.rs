pub struct Transition {
    pub active: bool,
    pub time: f32,
    pub duration: f32,
    pub alpha: f32,
    pub mode: TransitionMode,
}

pub enum TransitionMode {
    FadeIn,
    FadeOut,
}

impl Transition {
    pub fn new(duration: f32, mode: TransitionMode) -> Self {
        Self {
            active: true,
            time: 0.0,
            duration,
            alpha: if matches!(mode, TransitionMode::FadeIn) { 1.0 } else { 0.0 },
            mode,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.active {
            return;
        }

        self.time += delta;

        let mut t = self.time / self.duration;
        if t > 1.0 {
            t = 1.0;
            self.active = false;
        }

        match self.mode {
            TransitionMode::FadeIn => {
                self.alpha = 1.0 - t;
            }
            TransitionMode::FadeOut => {
                self.alpha = t;
            }
        }
    }
}