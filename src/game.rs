use glium::backend::glutin_backend::GlutinFacade;
use scene::SceneManager;
use event::step::fixed_60;
use scene::Scene;
use event::Keys;
use time;

/// The game object, you give it your initial State and start it off
pub struct Game<T, M> where M: SceneManager<T> {
    /// Your own state
    state: T,
    /// The Scene Manager
    scene_mgr: M,
    /// The display handle
    display: GlutinFacade,
    /// KeyStates
    keys: Keys,
    /// Time started
    time_started: f64,
    /// Time now
    time_now: f64,
}

impl<T, M> Game<T, M> where M: SceneManager<T> {
    /// Creates a new game you can start!
    pub fn new(state: T, mgr: M, disp: GlutinFacade) -> Game<T, M> {
        Game {
            state: state,
            scene_mgr: mgr,
            display: disp,
            keys: Keys::new(),
            time_started: 0.0,
            time_now: 0.0
        }
    }

    /// Consumes the game and starts the display loop, once there are no
    /// more scenes or the window is closed this method returns.
    pub fn kickoff(mut self) {
        self.time_started = time::precise_time_ns() as f64 / 1000_000_000. as f64;
        fixed_60(|dt| {
            use glium::glutin::Event;
            use event::StepResult;
            self.time_now = time::precise_time_ns() as f64 / 1000_000_000. as f64;

            self.keys.update(self.time_now);

            for ev in self.display.poll_events() {
                match ev {
                    Event::Closed => return StepResult::Stop,
                    Event::KeyboardInput(state, _, Some(key)) => {
                        self.keys.update_key(key, state, self.time_now);
                    }
                    _ => ()
                }
            }

            self.scene_mgr.update(dt, &self.keys);
            if self.scene_mgr.get_scenes().len() == 0 {
                return StepResult::Stop;
            }

            self.scene_mgr.display(&self.display);
            return StepResult::Continue;
        });
    }
}
