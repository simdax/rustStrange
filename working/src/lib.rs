#[repr(C)]
#[derive(Default)]
pub struct State {
    phase: f64,
    freq: f64,
    inc: f64,
    pub result: f32,
}

impl State {
    fn calculate(&mut self) -> f32 {
        let res = self.phase.sin();
        self.phase += self.inc;
        res as f32 * 0.3
    }

    fn set_freq(&mut self, freq: f64, sr: u32) {
        self.freq = freq;
        self.inc = (2.0f64 * std::f64::consts::PI) / (sr as f64) * freq;
    }
}

#[no_mangle]
pub extern "C" fn init(sr: u32) -> State {
    let mut s = State::default();
    s.set_freq(200.0, sr);
    s
}

#[no_mangle]
pub extern "C" fn generate(mut state: State) -> State {
    state.result = state.calculate();
    state
}

#[no_mangle]
pub extern "C" fn set(mut state: State, sr: u32, param: &str, value: f64) -> State {
    if let "freq" = param {
        state.set_freq(value, sr);
    };
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = init(48000);
        for i in 0..32 {
            s.calculate();
        }
    }
}
