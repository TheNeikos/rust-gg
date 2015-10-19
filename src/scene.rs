use glium::backend::glutin_backend::GlutinFacade;
use event::Keys;

/// Signalling Enum, meant to tell the SceneManager what should happen next.
pub enum SceneTransition<T> {
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
    type State;
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

