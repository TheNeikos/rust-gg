
/// A StepResult should be returned by the closure given to one of the step
/// functions.
pub enum StepResult {
    /// Signal that we should continue the event loop
    Continue,
    /// Signal that we should stop the event loop
    Stop
}

/// A collection of methods meant to make it easy to get a quick and dirty
/// event loop going.
pub mod step {
    use std::thread;
    use super::StepResult;
    use time;

    /// A simple and stupid loop that tries to call `cb` after `step` time has
    /// elapsed.
    ///
    /// `step` is in nanoseconds.
    /// `cb` is a closure meant to return a `StepResult`
    pub fn fixed<T>(step: u64, mut cb: T) where T: FnMut(f64) -> StepResult
    {
        let mut now = time::precise_time_ns();
        loop {
            if time::precise_time_ns() - now < step {
                thread::sleep_ms((step - (time::precise_time_ns() - now)) as u32 / 1000);
            }
            let dt = time::precise_time_ns() - now;
            if let StepResult::Stop = cb(dt as f64 / 1000_000_000. as f64) {
                break;
            }
            now = time::precise_time_ns();
        }
    }

    /// A curried version of `step::fixed`, trying to average 60 updates a
    /// second.
    /// See `step::fixed`
    pub fn fixed_60<T>(cb: T) where T: FnMut(f64) -> StepResult {
        fixed(16666, cb)
    }
}

mod test {
    use super::{step, StepResult};
    use time;

    #[test]
    fn test_fixed_step() {
        let mut t = 0;
        step::fixed(1000, |_dt| {
            t = 1;
            StepResult::Stop
        });

        assert_eq!(1, t);
    }

    #[test]
    fn test_fixed_60_step() {
        let mut t = 0;
        step::fixed_60(|_dt| {
            t = 1;
            StepResult::Stop
        });

        assert_eq!(1, t);
    }
}
