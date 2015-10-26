use vec_map::VecMap;
use glium::glutin::ElementState;

/// Re-Export Glutin VirtualKeyCodes
pub use glium::glutin::VirtualKeyCode as KeyCode;

/// An enum allowing us to communicate what state a given Key is at. The `f64`
/// in each variant tells you since when the key was last pressed. A value of 0
/// indicating that it has never been pressed.
#[derive(Copy, Clone, Debug)]
pub enum KeyState {
    /// The key has been pressed _this_ tick
    Pressed(f64),
    /// A key that has been held down for some time
    Held(f64),
    /// The key has just been released
    Released(f64),
    /// The Key is currently *not* pressed
    NotPressed(f64)
}

/// Holds state about the currently pressed buttons as well as buttons that just
/// have been pressed and those that are released.
/// Sequence is as follows:
///     1. Released
///     2. Press
///     3. Pressed
///     4. Release
///     5. goto: 1
///
/// Per default all keys are 'Released'
pub struct Keys {
    keys: VecMap<KeyState>
}

impl Keys {
    /// Gives you the KeyState of a given Key
    pub fn status(&self, key: KeyCode) -> KeyState {
        *self.keys.get(&(key as usize)).unwrap_or(&KeyState::NotPressed(0.0))
    }

    /// A quick way to check if a given key is pressed or held
    pub fn held(&self, key: KeyCode) -> bool {
        match self.status(key) {
            KeyState::Pressed(_)  | KeyState::Held(_) => { true },
            KeyState::Released(_) | KeyState::NotPressed(_) => { false }
        }
    }

    /// A quick way to check if a given key has just been pressed
    pub fn pressed(&self, key: KeyCode) -> bool {
        match self.status(key) {
            KeyState::Pressed(_) => { true },
            KeyState::Released(_) | KeyState::NotPressed(_)
                | KeyState::Held(_) => { false }
        }
    }

    /// A quick way to check if a given key is released
    pub fn not_pressed(&self, key: KeyCode) -> bool {
        !self.pressed(key)
    }

    /// Creates a new keys struct
    pub fn new() -> Keys {
        Keys {
            keys: VecMap::new(),
        }
    }

    /// Update a given key with a given state
    pub fn update_key(&mut self, key: KeyCode, state: ElementState, time: f64) {
        println!("{:?} {:?} {:?}", key, state, time);
        if let Some(keystate) = self.keys.get_mut(&(key as usize)) {
            match (state, *keystate) {
                (ElementState::Pressed, KeyState::NotPressed(_)) => {
                    *keystate = KeyState::Pressed(time);
                },
                (ElementState::Released, KeyState::Held(_)) => {
                    *keystate = KeyState::Released(time);
                },
                (ElementState::Pressed, KeyState::Held(_)) => {
                    // We do nothing in this case because it is still being held
                },
                (es, state) => {
                    panic!("Received a ({:?}, {:?}) pair! That shouldn't happen!",
                        es, state);
                }
            }
        }
        if !self.keys.contains_key(&(key as usize)) {
            self.keys.insert(key as usize, KeyState::Pressed(time));
        }
    }

    /// Update all the keys and advance them to the next step
    pub fn update(&mut self, time: f64) {
        for (_, value) in self.keys.iter_mut() {
            match *value {
                KeyState::Pressed(time) => {
                    *value = KeyState::Held(time);
                },
                KeyState::Released(_) => {
                    *value = KeyState::NotPressed(time);
                },
                _ => {}
            }
        }
    }
}

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

#[allow(unused_imports)]
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
