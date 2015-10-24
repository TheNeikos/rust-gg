/// Allows us to verify that there is a way to get an id for this
pub trait HasId {
    /// The id of a scene to identify it
    fn get_id(&self) -> usize;
}
