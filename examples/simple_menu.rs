extern crate gg;
extern crate glium;

use glium::DisplayBuild;
use gg::scene::{Scene, StackSceneManager};
use gg::traits::HasId;
use std::rc::Rc;
use std::cell::RefCell;

struct GameState;

type State = Rc<RefCell<GameState>>;

struct MainMenu {
    quit: bool,
}

impl MainMenu {
    fn quit(&mut self) {
        self.quit = true;
    }
}

impl Scene for MainMenu {
    type State = State;
    fn enter(&mut self, _state: &mut Self::State) {
        println!("Enter State");
    }
    fn leave(&mut self, _state: &mut Self::State) {
        println!("Leave State");
    }
    fn keypress(&mut self, _state: &mut Self::State, keys: &gg::event::Keys) {
        use gg::event::KeyCode::*;

        if keys.pressed(Escape) {
            self.quit();
        }
    }

    fn display(&mut self, _state: &mut Self::State, display: &glium::backend::glutin_backend::GlutinFacade) {
        use glium::Surface;
        let mut target = display.draw();
        target.clear_color(0., 0., 1., 1.);
        target.finish().unwrap();
    }

    fn tick(&mut self, _state: &mut Self::State, _dt: f64) -> gg::scene::SceneTransition<Self::State> {
        if self.quit {
            gg::scene::SceneTransition::Pop
        } else {
            gg::scene::SceneTransition::Nothing
        }
    }

}

impl HasId for MainMenu {
    fn get_id(&self) -> usize {
        0
    }
}

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let state = Rc::new(RefCell::new(GameState));

    let game = gg::Game::new(
        state.clone(),
        StackSceneManager::with_scene(
            state.clone(),
            Box::new(MainMenu { quit: false })
        ),
        display
    );

    // Internally calls the draw/tick loop
    game.kickoff();

}
