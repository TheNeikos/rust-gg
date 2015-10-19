//!
//! # GG
//!
//! A simple and straight to the point Game Developement Library
//!

#![deny(missing_docs)]

#![feature(associated_type_defaults)]

extern crate glium;
extern crate time;

use glium::backend::glutin_backend::GlutinFacade;

/// The event module
/// TODO: Expand
pub mod event;

/// The Scenes module
pub mod scene;

/// The game object, you give it your initial State and start it off
pub struct Game<T> {
    /// Your own state
    state: T,
    /// The display handle
    display: GlutinFacade
}
