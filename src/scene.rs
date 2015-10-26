use glium::backend::glutin_backend::GlutinFacade;
use event::Keys;
use traits::HasId;

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
pub trait Scene : HasId {
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
}

/// This trait has to be implemented by the SceneManager that will run your game.
/// A sample implementation is `StackSceneManager`
pub trait SceneManager<T : Sized> {
    /// The Associated Scene
    type Scene : ?Sized + HasId = Scene<State=T>;
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

impl<T> StackSceneManager<T> {
    /// Creates a new StackSceneManager. It has nothing in it,
    /// you probably want to use `with_scene`
    pub fn new(state: T) -> StackSceneManager<T> {
        StackSceneManager {
            scenes: Vec::new(),
            state: state
        }
    }

    /// Creates a StackSceneManager with
    pub fn with_scene(state: T, scene: Box<Scene<State=T>>) -> StackSceneManager<T>
    {
        let mut m = StackSceneManager::new(state);
        m.handle_transition(SceneTransition::Push(scene));
        m
    }
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
                if let Some(s) = self.scenes.last_mut() {
                    s.leave(&mut self.state);
                }
                self.scenes.push(boxed_scene);
                if let Some(s) = self.scenes.last_mut() {
                    s.enter(&mut self.state);
                }
            },
            Pop => {
                if let Some(mut s) = self.scenes.pop() {
                    s.leave(&mut self.state);
                }
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

                if let Some(s) = self.scenes.last_mut() {
                    s.leave(&mut self.state);
                }

                while length > 0 {
                    if let Some(k) = self.scenes.last().map(|s| s.get_id()) {
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
                    if let Some(s) = self.scenes.last_mut() {
                        s.enter(&mut self.state);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use glium::backend::glutin_backend::GlutinFacade;
    use glium::glutin::HeadlessRendererBuilder;
    use glium::DisplayBuild;

    use traits::HasId;

    struct TestData {
        has_been_modified: usize,
        has_entered:       usize,
        has_left:          usize,
    }

    type State = Rc<RefCell<TestData>>;

    fn create_state() -> State {
        Rc::new(RefCell::new(TestData {
            has_been_modified: 0,
            has_entered:       0,
            has_left:          0,
        }))
    }

    fn create_scene_manager(state: State) -> StackSceneManager<State> {
        StackSceneManager {
            scenes: Vec::new(),
            state: state
        }
    }

    fn create_display() -> GlutinFacade {
        HeadlessRendererBuilder::new(1024, 768).build_glium().unwrap()
    }

    #[test]
    fn enter_leave_scene_manager() {
        struct TestScene;

        impl HasId for TestScene {
            fn get_id(&self) -> usize {
                0
            }
        }

        impl Scene for TestScene {
            type State = State;
            fn enter(&mut self, data: &mut State) {
                data.borrow_mut().has_entered += 1;
            }
            fn leave(&mut self, data: &mut State) {
                data.borrow_mut().has_left += 1;
            }
            fn tick(&mut self, data: &mut State) -> SceneTransition<State>
            {
                if data.borrow().has_been_modified > 0 {
                    return SceneTransition::Pop
                }

                data.borrow_mut().has_been_modified = 1;
                SceneTransition::Nothing
            }
        }

        let mut state = create_state();
        let mut mgr = create_scene_manager(state.clone());

        mgr.handle_transition(SceneTransition::Push(Box::new(TestScene)));

        assert_eq!(mgr.get_scenes().len(), 1);

        let answer = mgr.get_scenes_mut().last_mut().unwrap().tick(&mut state);
        mgr.handle_transition(answer);

        assert_eq!(state.borrow().has_been_modified, 1);

        let answer = mgr.get_scenes_mut().last_mut().unwrap().tick(&mut state);
        mgr.handle_transition(answer);

        assert_eq!(mgr.get_scenes().len(), 0);
        assert_eq!(state.borrow().has_entered, 1);
        assert_eq!(state.borrow().has_left, 1);
    }

    #[test]
    fn fake_display() {
        struct TestScene;

        impl HasId for TestScene {
            fn get_id(&self) -> usize {
                0
            }
        }

        impl Scene for TestScene {
            type State = State;
            fn display(&mut self, data: &mut Self::State, display: &GlutinFacade) {
                use glium::Surface;
                let mut frame = display.draw();
                frame.clear_color(0.,1.,0.,1.0);
                data.borrow_mut().has_been_modified = 1;
                frame.finish().unwrap();
            }
        }
        let mut state = create_state();
        let display = create_display();

        let mut scene = TestScene;

        scene.display(&mut state, &display);

        assert_eq!(state.borrow().has_been_modified, 1);
    }

    #[test]
    fn popuntil_manager() {
        struct TestScene;

        impl HasId for TestScene {
            fn get_id(&self) -> usize {
                0
            }
        }

        impl Scene for TestScene {
            type State = State;
            fn enter(&mut self, data: &mut State) {
                data.borrow_mut().has_entered += 1;
            }
            fn leave(&mut self, data: &mut State) {
                data.borrow_mut().has_left += 1;
            }
            fn tick(&mut self, _data: &mut State) -> SceneTransition<State>
            {
                SceneTransition::Push(Box::new(TestSceneMenu))
            }
        }

        struct TestSceneMenu;

        impl HasId for TestSceneMenu {
            fn get_id(&self) -> usize {
                1
            }
        }

        impl Scene for TestSceneMenu {
            type State = State;
            fn enter(&mut self, data: &mut State) {
                data.borrow_mut().has_entered += 1;
            }
            fn leave(&mut self, data: &mut State) {
                data.borrow_mut().has_left += 1;
            }
            fn tick(&mut self, _data: &mut State) -> SceneTransition<State>
            {
                SceneTransition::Push(Box::new(TestSceneSubMenu))
            }
        }

        struct TestSceneSubMenu;

        impl HasId for TestSceneSubMenu {
            fn get_id(&self) -> usize {
                2
            }
        }

        impl Scene for TestSceneSubMenu {
            type State = State;
            fn enter(&mut self, data: &mut State) {
                data.borrow_mut().has_entered += 1;
            }
            fn leave(&mut self, data: &mut State) {
                data.borrow_mut().has_left += 1;
            }
            fn tick(&mut self, _data: &mut State) -> SceneTransition<State>
            {
                SceneTransition::PopUntil(0)
            }
        }


        let mut state = create_state();
        let mut mgr = create_scene_manager(state.clone());

        mgr.handle_transition(SceneTransition::Push(Box::new(TestScene)));

        let answer = mgr.get_scenes_mut().last_mut().unwrap().tick(&mut state);
        mgr.handle_transition(answer);

        assert_eq!(mgr.get_scenes().len(), 2);

        let answer = mgr.get_scenes_mut().last_mut().unwrap().tick(&mut state);
        mgr.handle_transition(answer);

        assert_eq!(mgr.get_scenes().len(), 3);

        let answer = mgr.get_scenes_mut().last_mut().unwrap().tick(&mut state);
        mgr.handle_transition(answer);

        assert_eq!(mgr.get_scenes().len(), 1);

        // TestScene -> Menu -> SubMenu -> TestScene
        assert_eq!(state.borrow().has_entered, 4);

        // TestScene -> Menu -> SubMenu
        assert_eq!(state.borrow().has_left, 3);

    }

}
