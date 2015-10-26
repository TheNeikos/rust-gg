//!
//! # GG
//!
//! A simple and straight to the point Game Developement Library
//!

#![deny(missing_docs)]

#![feature(associated_type_defaults)]

extern crate glium;
extern crate time;
extern crate vec_map;

/// The event module
/// TODO: Expand
pub mod event;

/// The Scenes module
/// TODO: Expand
pub mod scene;

/// Commonly used traits
/// TODO: Expand
pub mod traits;

/// Global game related
/// TODO: Expand
pub mod game;

pub use game::Game;
