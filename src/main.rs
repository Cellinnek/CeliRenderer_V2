use std::alloc::System;

#[global_allocator]
static A: System = System;

extern crate core;

use minifb::{Scale, Window, WindowOptions};
use minifb::Key::{Escape};
use minifb::ScaleMode::AspectRatioStretch;

const WIDTH: usize = 1024;
const HEIGHT: usize = 512;

fn main() {
    let mut buffer:Vec<u32>= vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    ).unwrap();

    window.set_position(500, 175);

    while window.is_open() && !window.is_key_down(Escape) {


    buffer.clear();
    buffer.resize(WIDTH*HEIGHT,0);
    }
}
