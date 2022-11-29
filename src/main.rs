use std::alloc::System;
use std::f64::consts::PI;
use std::time::{Instant};


#[global_allocator]
static A: System = System;

extern crate core;

use minifb::{Window, WindowOptions};
use minifb::Key::{Escape};


const WIDTH: usize = 512;
const HEIGHT: usize = 512;

mod functions;
use functions::*;

fn main() {
    let f_near: f64 = 0.1;
    let f_far: f64 = 1000.0;
    let f_fov: f64 = 90.0;
    let f_aspect_ratio: f64 = HEIGHT as f64/WIDTH as f64;
    let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * PI).tan();

    let mat_proj = mat4x4([[f_aspect_ratio * f_fov_rad, 0.0, 0.0, 0.0],
        [0.0, f_fov_rad, 0.0, 0.0],
        [0.0, 0.0, f_far / (f_far - f_near), 1.0],
        [0.0, 0.0, (-f_far * f_near) / (f_far - f_near), 0.0]
    ]);

    /*let mut mesh_cube = mesh {
        tris: vec![

            // SOUTH
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 0.0 },
            },
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 0.0, z: 0.0 },
            },

            // EAST
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 1.0 },
            },
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 1.0, y: 0.0, z: 1.0 },
            },

            //NORTH
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 1.0, z: 1.0 },
            },
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 1.0 },
            },

            // WEST
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 1.0, z: 0.0 },
            },
            triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 0.0 },
            },

            // TOP
            triangle {
                a: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                b: vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 1.0 },
            },
            triangle {
                a: vec3d { x: 0.0, y: 1.0, z: 0.0 },
                b: vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: vec3d { x: 1.0, y: 1.0, z: 0.0 },
            },

            // BOTTOM
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 0.0, z: 1.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 0.0 },
            },
            triangle {
                a: vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                c: vec3d { x: 1.0, y: 0.0, z: 0.0 },
            },
        ],
    };*/

    let mut mesh_cube: mesh = mesh{ tris: vec![] };
    mesh_cube.load_from_object_file("C:/Users/Cysie/CLionProjects/Renderer_V2/src/VideoShip.obj");

    let mut mat_rot_z:mat4x4;
    let mut mat_rot_x:mat4x4;
    let mut f_theta:f64;
    let v_camera = vec3d{
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
        f_theta = start.elapsed().as_secs_f64();

        // Rotation Z
        mat_rot_z = mat4x4([
            [f_theta.cos(), (f_theta).sin(), 0.0, 0.0],
            [-(f_theta).sin(), (f_theta).cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Rotation X
        mat_rot_x = mat4x4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, (f_theta * 0.5).cos(), (f_theta * 0.5).sin(), 0.0],
            [0.0, -(f_theta * 0.5).sin(), (f_theta * 0.5).cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        for tri in &mesh_cube.tris{
            let mut tri_projected = triangle {
                a: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: vec3d { x: 0.0, y: 0.0, z: 0.0 },
                c: vec3d { x: 0.0, y: 0.0, z: 0.0 },
            };
            let mut tri_translated:triangle;
            let mut tri_rotated_zx:triangle;
            let mut tri_rotated_z:triangle;
            tri_rotated_z = tri_projected.clone();
            tri_rotated_zx = tri_projected.clone();

            MultiplyMatricVector(&tri.a, &mut tri_rotated_z.a, &mat_rot_z);
            MultiplyMatricVector(&tri.b, &mut tri_rotated_z.b, &mat_rot_z);
            MultiplyMatricVector(&tri.c, &mut tri_rotated_z.c, &mat_rot_z);

            MultiplyMatricVector(&tri_rotated_z.a, &mut tri_rotated_zx.a, &mat_rot_x);
            MultiplyMatricVector(&tri_rotated_z.b, &mut tri_rotated_zx.b, &mat_rot_x);
            MultiplyMatricVector(&tri_rotated_z.c, &mut tri_rotated_zx.c, &mat_rot_x);

            tri_translated = tri_rotated_zx.clone();
            tri_translated.a.z = tri_rotated_zx.a.z + 3.0;
            tri_translated.b.z = tri_rotated_zx.b.z + 3.0;
            tri_translated.c.z = tri_rotated_zx.c.z + 3.0;

            let line1 = vec3d{
                x: tri_translated.b.x - tri_translated.a.x,
                y: tri_translated.b.y - tri_translated.a.y,
                z: tri_translated.b.z - tri_translated.a.z
            };

            let line2 = vec3d{
                x: tri_translated.c.x - tri_translated.a.x,
                y: tri_translated.c.y - tri_translated.a.y,
                z: tri_translated.c.z - tri_translated.a.z
            };

            let mut normal = vec3d{
                x: line1.y * line2.z - line1.z * line2.y,
                y: line1.z * line2.x - line1.x * line2.z,
                z: line1.x * line2.y - line1.y * line2.x
            };

            let l = (normal.x*normal.x + normal.y*normal.y + normal.z*normal.z).sqrt();
            normal.x /= l; normal.y /= l; normal.z /= l;

            if (normal.x * (tri_translated.a.x - v_camera.x) +
                normal.y * (tri_translated.a.y - v_camera.y) +
                normal.z * (tri_translated.a.z - v_camera.z)) < 0.0 {
                let mut light_direction = vec3d{ x: 0.0, y: 0.0, z: -1.0 };
                let l = (light_direction.x*light_direction.x + light_direction.y*light_direction.y + light_direction.z*light_direction.z).sqrt();
                light_direction.x /= l; light_direction.y /= l; light_direction.z /= l;

                let dp = normal.x * light_direction.x + normal.y * light_direction.y + normal.z * light_direction.z;
                let col = (255.0*dp) as u32 * 0x10101;

                MultiplyMatricVector(&tri_translated.a, &mut tri_projected.a, &mat_proj);
                MultiplyMatricVector(&tri_translated.b, &mut tri_projected.b, &mat_proj);
                MultiplyMatricVector(&tri_translated.c, &mut tri_projected.c, &mat_proj);

                tri_projected.a.x += 1.0;
                tri_projected.a.y += 1.0;
                tri_projected.b.x += 1.0;
                tri_projected.b.y += 1.0;
                tri_projected.c.x += 1.0;
                tri_projected.c.y += 1.0;

                tri_projected.a.x *= 0.5 * WIDTH as f64;
                tri_projected.a.y *= 0.5 * HEIGHT as f64;
                tri_projected.b.x *= 0.5 * WIDTH as f64;
                tri_projected.b.y *= 0.5 * HEIGHT as f64;
                tri_projected.c.x *= 0.5 * WIDTH as f64;
                tri_projected.c.y *= 0.5 * HEIGHT as f64;

                draw_triangle_faces(&mut buffer,
                                    [tri_projected.a.x as i32, tri_projected.a.y as i32],
                                    [tri_projected.b.x as i32, tri_projected.b.y as i32],
                                    [tri_projected.c.x as i32, tri_projected.c.y as i32],
                                    col)
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Oops!");
        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);
    }
}
