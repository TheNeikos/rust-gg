use glium::backend::glutin_backend::GlutinFacade;

/// The game object, you give it your initial State and start it off
pub struct Game<T> {
    /// Your own state
    state: T,
    /// The display handle
    display: GlutinFacade
}
