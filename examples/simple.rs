extern crate gg;
extern crate glium;

use glium::DisplayBuild;
use gg::event::step::fixed_60;

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    // fixed_60(|_dt| {
    //     use gg::event::StepResult;
    //     for ev in display.poll_events() {
    //         match ev {
    //             glium::glutin::Event::Closed => return StepResult::Stop,
    //             _ => ()
    //         }
    //     }
    //     return StepResult::Continue;
    // });

}
