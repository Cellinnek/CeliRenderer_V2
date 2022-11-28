use std::alloc::System;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

#[global_allocator]
static A: System = System;

extern crate core;

use minifb::{Scale, Window, WindowOptions};
use minifb::Key::{Escape};
use minifb::ScaleMode::AspectRatioStretch;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

mod functions;
use functions::*;

fn main() {
    let fNear: f64 = 0.1;
    let fFar: f64 = 1000.0;
    let fFov: f64 = 90.0;
    let fAspectRatio: f64 = HEIGHT as f64/WIDTH as f64;
    let fFovRad = 1.0 / (fFov * 0.5 / 180.0 * PI).tan();

    let matProj = mat4x4 {
        m: [[fAspectRatio * fFovRad,0.0,0.0,0.0],
            [0.0,fFovRad,0.0,0.0],
            [0.0,0.0,fFar / (fFar - fNear),1.0],
            [0.0,0.0,(-fFar * fNear) / (fFar - fNear),0.0]
        ]
    };

    let meshCube = mesh {
        tris: vec![

            // SOUTH
            triangle(vec3d{ x: 0.0, y: 0.0, z: 0.0},vec3d{ x: 0.0, y: 1.0, z: 0.0},vec3d{ x: 1.0, y: 1.0, z: 0.0}),
            triangle(vec3d{ x: 0.0, y: 0.0, z: 0.0},vec3d{ x: 1.0, y: 1.0, z: 0.0},vec3d{ x: 1.0, y: 0.0, z: 0.0}),

            // EAST
            triangle(vec3d{ x: 1.0, y: 0.0, z: 0.0},vec3d{ x: 1.0, y: 1.0, z: 0.0},vec3d{ x: 1.0, y: 1.0, z: 1.0}),
            triangle(vec3d{ x: 1.0, y: 0.0, z: 0.0},vec3d{ x: 1.0, y: 1.0, z: 1.0},vec3d{ x: 1.0, y: 0.0, z: 1.0}),

            //NORTH
            triangle(vec3d{ x: 1.0, y: 0.0, z: 1.0},vec3d{ x: 1.0, y: 1.0, z: 1.0},vec3d{ x: 0.0, y: 1.0, z: 1.0}),
            triangle(vec3d{ x: 1.0, y: 0.0, z: 1.0},vec3d{ x: 0.0, y: 1.0, z: 1.0},vec3d{ x: 0.0, y: 0.0, z: 1.0}),

            // WEST
            triangle(vec3d{ x: 0.0, y: 0.0, z: 1.0},vec3d{ x: 0.0, y: 1.0, z: 1.0},vec3d{ x: 0.0, y: 1.0, z: 0.0}),
            triangle(vec3d{ x: 0.0, y: 0.0, z: 1.0},vec3d{ x: 0.0, y: 1.0, z: 0.0},vec3d{ x: 0.0, y: 0.0, z: 0.0}),

            // TOP
            triangle(vec3d{ x: 0.0, y: 1.0, z: 0.0},vec3d{ x: 0.0, y: 1.0, z: 1.0},vec3d{ x: 1.0, y: 1.0, z: 1.0}),
            triangle(vec3d{ x: 0.0, y: 1.0, z: 0.0},vec3d{ x: 1.0, y: 1.0, z: 1.0},vec3d{ x: 1.0, y: 1.0, z: 0.0}),

            // BOTTOM
            triangle(vec3d{ x: 1.0, y: 0.0, z: 1.0},vec3d{ x: 0.0, y: 0.0, z: 1.0},vec3d{ x: 0.0, y: 0.0, z: 0.0}),
            triangle(vec3d{ x: 1.0, y: 0.0, z: 1.0},vec3d{ x: 0.0, y: 0.0, z: 0.0},vec3d{ x: 1.0, y: 0.0, z: 0.0}),
        ],
    };

    let mut buffer:Vec<u32>= vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ).unwrap();

    window.set_position(500, 175);
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/

    let mut start = Instant::now();

    let mut matRotZ:mat4x4;
    let mut matRotX:mat4x4;
    let mut fTheta:f64 = 50.0;

    while window.is_open() && !window.is_key_down(Escape) {
        fTheta = start.elapsed().as_secs_f64();

        // Rotation Z
        matRotZ = mat4x4{
            m: [
                [fTheta.cos(),(fTheta).sin(),0.0,0.0],
                [-(fTheta).sin(),(fTheta).cos(),0.0,0.0],
                [0.0,0.0,1.0,0.0],
                [0.0,0.0,0.0,1.0],
            ]
        };

        // Rotation X
        matRotX = mat4x4{
          m: [
              [1.0,0.0,0.0,0.0],
              [0.0,(fTheta * 0.5).cos(),(fTheta * 0.5).sin(),0.0],
              [0.0,-(fTheta * 0.5).sin(),(fTheta * 0.5).cos(),0.0],
              [0.0,0.0,0.0,1.0],
          ]
        };

        for tri in &meshCube.tris{
            let mut triProjected = triangle(vec3d{ x: 0.0, y: 0.0, z: 0.0},vec3d{ x: 0.0, y: 0.0, z: 0.0},vec3d{ x: 0.0, y: 0.0, z: 0.0});
            let mut triTranslated:triangle;
            let mut triRotatedZX:triangle;
            let mut triRotatedZ:triangle;
            triRotatedZ = triProjected.clone();
            triRotatedZX = triProjected.clone();

            MultiplyMatricVector(&tri.0, &mut triRotatedZ.0, &matRotZ);
            MultiplyMatricVector(&tri.1, &mut triRotatedZ.1, &matRotZ);
            MultiplyMatricVector(&tri.2, &mut triRotatedZ.2, &matRotZ);

            MultiplyMatricVector(&triRotatedZ.0, &mut triRotatedZX.0, &matRotX);
            MultiplyMatricVector(&triRotatedZ.1, &mut triRotatedZX.1, &matRotX);
            MultiplyMatricVector(&triRotatedZ.2, &mut triRotatedZX.2, &matRotX);

            triTranslated = triRotatedZX.clone();
            triTranslated.0.z = triRotatedZX.0.z + 3.0;
            triTranslated.1.z = triRotatedZX.1.z + 3.0;
            triTranslated.2.z = triRotatedZX.2.z + 3.0;

            MultiplyMatricVector(&triTranslated.0, &mut triProjected.0, &matProj);
            MultiplyMatricVector(&triTranslated.1, &mut triProjected.1, &matProj);
            MultiplyMatricVector(&triTranslated.2, &mut triProjected.2, &matProj);

            triProjected.0.x += 1.0; triProjected.0.y += 1.0;
            triProjected.1.x += 1.0; triProjected.1.y += 1.0;
            triProjected.2.x += 1.0; triProjected.2.y += 1.0;

            triProjected.0.x *= 0.5*WIDTH as f64; triProjected.0.y *= 0.5*HEIGHT as f64;
            triProjected.1.x *= 0.5*WIDTH as f64; triProjected.1.y *= 0.5*HEIGHT as f64;
            triProjected.2.x *= 0.5*WIDTH as f64; triProjected.2.y *= 0.5*HEIGHT as f64;

            draw_triangle_edges(&mut buffer,
                          [triProjected.0.x as i32,triProjected.0.y as i32],
                          [triProjected.1.x as i32,triProjected.1.y as i32],
                          [triProjected.2.x as i32,triProjected.2.y as i32],
                            0x00ffffff)
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Oops!");
        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);
    }
}
