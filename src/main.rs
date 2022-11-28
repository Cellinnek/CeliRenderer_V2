use std::alloc::System;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

#[global_allocator]
static A: System = System;

extern crate core;

use minifb::{Scale, Window, WindowOptions};
use minifb::Key::{Escape};
use minifb::ScaleMode::AspectRatioStretch;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

mod functions;
use functions::*;

fn main() {
    let fNear: f64 = 0.1;
    let fFar: f64 = 1000.0;
    let fFov: f64 = 90.0;
    let fAspectRatio: f64 = HEIGHT as f64/WIDTH as f64;
    let fFovRad = 1.0 / (fFov * 0.5 / 180.0 * PI).tan();

    let matProj = mat4x4([[fAspectRatio * fFovRad, 0.0, 0.0, 0.0],
        [0.0, fFovRad, 0.0, 0.0],
        [0.0, 0.0, fFar / (fFar - fNear), 1.0],
        [0.0, 0.0, (-fFar * fNear) / (fFar - fNear), 0.0]
    ]);

    let meshCube = mesh {
        tris: vec![

            // SOUTH
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 0.0 }
            },
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 0.0, z: 0.0 }
            },

            // EAST
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 1.0 }
            },
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 1.0, y: 0.0, z: 1.0 }
            },

            //NORTH
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 1.0, z: 1.0 }
            },
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 1.0 }
            },

            // WEST
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 1.0, z: 0.0 }
            },
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 0.0 }
            },

            // TOP
            triangle {
                a: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 1.0 }
            },
            triangle {
                a: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 0.0 }
            },

            // BOTTOM
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 0.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 0.0 }
            },
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 0.0, z: 0.0 }
            },
        ],
    };

    let mut matRotZ:mat4x4;
    let mut matRotX:mat4x4;
    let mut fTheta:f64;
    let vCamera = vec3d{
        x: 0.0,
        y: 0.0,
        z: 0.0
    };

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
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/

    let start = Instant::now();

    while window.is_open() && !window.is_key_down(Escape) {
        fTheta = start.elapsed().as_secs_f64();

        // Rotation Z
        matRotZ = mat4x4([
            [fTheta.cos(), (fTheta).sin(), 0.0, 0.0],
            [-(fTheta).sin(), (fTheta).cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Rotation X
        matRotX = mat4x4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, (fTheta * 0.5).cos(), (fTheta * 0.5).sin(), 0.0],
            [0.0, -(fTheta * 0.5).sin(), (fTheta * 0.5).cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        for tri in &meshCube.tris{
            let mut triProjected = triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 0.0 }
            };
            let mut triTranslated:triangle;
            let mut triRotatedZX:triangle;
            let mut triRotatedZ:triangle;
            triRotatedZ = triProjected.clone();
            triRotatedZX = triProjected.clone();

            MultiplyMatricVector(&tri.a, &mut triRotatedZ.a, &matRotZ);
            MultiplyMatricVector(&tri.b, &mut triRotatedZ.b, &matRotZ);
            MultiplyMatricVector(&tri.c, &mut triRotatedZ.c, &matRotZ);

            MultiplyMatricVector(&triRotatedZ.a, &mut triRotatedZX.a, &matRotX);
            MultiplyMatricVector(&triRotatedZ.b, &mut triRotatedZX.b, &matRotX);
            MultiplyMatricVector(&triRotatedZ.c, &mut triRotatedZX.c, &matRotX);

            triTranslated = triRotatedZX.clone();
            triTranslated.a.z = triRotatedZX.a.z + 3.0;
            triTranslated.b.z = triRotatedZX.b.z + 3.0;
            triTranslated.c.z = triRotatedZX.c.z + 3.0;

            let line1 = vec3d{
                x: triTranslated.b.x - triTranslated.a.x,
                y: triTranslated.b.y - triTranslated.a.y,
                z: triTranslated.b.z - triTranslated.a.z
            };

            let line2 = vec3d{
                x: triTranslated.c.x - triTranslated.a.x,
                y: triTranslated.c.y - triTranslated.a.y,
                z: triTranslated.c.z - triTranslated.a.z
            };

            let mut normal = vec3d{
                x: line1.y * line2.z - line1.z * line2.y,
                y: line1.z * line2.x - line1.x * line2.z,
                z: line1.x * line2.y - line1.y * line2.x
            };

            let l = (normal.x*normal.x + normal.y*normal.y + normal.z*normal.z).sqrt();
            normal.x /= l; normal.y /= l; normal.z /= l;

            if (normal.x * (triTranslated.a.x - vCamera.x) +
                normal.y * (triTranslated.a.y - vCamera.y) +
                normal.z * (triTranslated.a.z - vCamera.z)) < 0.0 {
                let mut light_direction = vec3d{ x: 0.0, y: 0.0, z: -1.0 };
                let l = (light_direction.x*light_direction.x + light_direction.y*light_direction.y + light_direction.z*light_direction.z).sqrt();
                light_direction.x /= l; light_direction.y /= l; light_direction.z /= l;

                let dp = normal.x * light_direction.x + normal.y * light_direction.y + normal.z * light_direction.z;

                MultiplyMatricVector(&triTranslated.a, &mut triProjected.a, &matProj);
                MultiplyMatricVector(&triTranslated.b, &mut triProjected.b, &matProj);
                MultiplyMatricVector(&triTranslated.c, &mut triProjected.c, &matProj);

                triProjected.a.x += 1.0;
                triProjected.a.y += 1.0;
                triProjected.b.x += 1.0;
                triProjected.b.y += 1.0;
                triProjected.c.x += 1.0;
                triProjected.c.y += 1.0;

                triProjected.a.x *= 0.5 * WIDTH as f64;
                triProjected.a.y *= 0.5 * HEIGHT as f64;
                triProjected.b.x *= 0.5 * WIDTH as f64;
                triProjected.b.y *= 0.5 * HEIGHT as f64;
                triProjected.c.x *= 0.5 * WIDTH as f64;
                triProjected.c.y *= 0.5 * HEIGHT as f64;

                draw_triangle_faces(&mut buffer,
                                    [triProjected.a.x as i32, triProjected.a.y as i32],
                                    [triProjected.b.x as i32, triProjected.b.y as i32],
                                    [triProjected.c.x as i32, triProjected.c.y as i32],
                                    0x00ffffff)
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Oops!");
        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);
    }
}
