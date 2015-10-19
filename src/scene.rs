use glium::backend::glutin_backend::GlutinFacade;
use event::Keys;

/// Signalling Enum, meant to tell the SceneManager what should happen next.
pub enum SceneTransition<T : Sized> {
    /// `Nothing` will leave the current Scene on the Stack.
    Nothing,
    /// `Push` will leave the current scene (but not destroy it) and put the new
    /// scene on the stack.
    Push(Box<Scene<State=T>>),
    /// `Pop` will remove the current Scene from the stack returning to the previous
    /// one.
    Pop,
    /// `PopUntil` will remove scenes until the given scene is found, this is useful
    /// to get back to a parent menu for example.
    /// **This panics if the menu does not exist!**
    PopUntil(usize)
}

/// One of the most important traits for a game, the scene is what tells the
/// display what to draw as well as what should happen with the given input.
pub trait Scene {
    /// What kind of state is carried around?
    type State : Sized;
    /// Called everytime this scene becomes the top of the stack
    fn enter(&mut self, _state: &mut Self::State) {}
    /// Called everytime this scene stops being the top of the stack (also
    /// before a drop)
    fn leave(&mut self, _state: &mut Self::State) {}
    /// Convenience method where you can handle keyboard input specifically.
    /// This is called _before_ `tick`.
    fn keypress(&mut self, _state: &mut Self::State, _keys: &Keys) {}
    /// Called with a display to draw into something
    fn display(&mut self, _state: &mut Self::State, _display: &GlutinFacade) {}
    /// Called to update the state so as to reflect one advancement in time.
    fn tick(&mut self, _state: &mut Self::State) -> SceneTransition<Self::State>
    {
        SceneTransition::Pop
    }
    /// The id of a scene to identify it
    fn get_id(&self) -> usize;
}

/// This trait has to be implemented by the SceneManager that will run your game.
/// A sample implementation is `StackSceneManager`
pub trait SceneManager<T : Sized> {
    /// The Associated Scene
    type Scene : ?Sized = Scene<State=T>;
    /// The Associated SceneTransition
    type SceneTransition = SceneTransition<T>;

    /// Return the scenes as non-mut references
    fn get_scenes(&self) -> &Vec<Box<Self::Scene>>;
    /// Return the scenes as mut references
    fn get_scenes_mut(&mut self) -> &mut Vec<Box<Self::Scene>>;
    /// Make the manager handle a given transition.
    fn handle_transition(&mut self, Self::SceneTransition);
}

/// A sample implementation of `SceneManager` can be used as is for a stack
/// based scene system. The type parameter is the state of the game.
pub struct StackSceneManager<T : Sized> {
    /// The scenes inside the manager.
    scenes: Vec<Box<Scene<State=T>>>,
    state: T
}

impl<T> SceneManager<T> for StackSceneManager<T> where T: Sized {
    fn get_scenes(&self) -> &Vec<Box<Self::Scene>> {
        return &self.scenes;
    }

    fn get_scenes_mut(&mut self) -> &mut Vec<Box<Self::Scene>> {
        return &mut self.scenes;
    }

    fn handle_transition(&mut self, trans: Self::SceneTransition) {
        use scene::SceneTransition::*;
        match trans {
            Nothing => {},
            Push(boxed_scene) => {
                if let Some(s) = self.scenes.first_mut() {
                    s.leave(&mut self.state);
                }
                self.scenes.push(boxed_scene);
                if let Some(s) = self.scenes.first_mut() {
                    s.enter(&mut self.state);
                }
            },
            Pop => {
                self.scenes.pop();
            },
            PopUntil(id) => {
                // If we have just one or zero scenes we can simply panic.
                // If not then we just call leave once and iterate through
                // If we have not panicked at the end we then enter that scene
                let mut length = self.scenes.len();

                if length == 0 {
                    // This should never happen !?
                    panic!("Tried to pop until on an empty stack.");
                }

                if length == 1 {
                    panic!("Tried to pop until a nonexistant stack with 1 element.");
                }

                if let Some(s) = self.scenes.first_mut() {
                    s.leave(&mut self.state);
                }

                while length > 0 {
                    if let Some(k) = self.scenes.first().map(|s| s.get_id()) {
                        if k == id {
                            break;
                        } else {
                            self.scenes.pop();
                        }
                    }

                    length = self.scenes.len();
                }

                if length == 0 {
                    panic!("Emptied the stack in a PopUntil, use Quit instead if this is wanted.");
                } else {
                    if let Some(s) = self.scenes.first_mut() {
                        s.enter(&mut self.state);
                    }
                }
            }
        }
    }
}
