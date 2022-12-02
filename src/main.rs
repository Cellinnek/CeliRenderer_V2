use std::alloc::System;
use std::f64::consts::PI;
use std::time::{Instant};


#[global_allocator]
static A: System = System;

extern crate core;

use minifb::{Scale, Window, WindowOptions};
use minifb::Key::{Escape};


const WIDTH: usize = 800;
const HEIGHT: usize = 800;

mod functions;
use functions::*;

fn main() {
    let f_near: f64 = 0.1;
    let f_far: f64 = 1000.0;
    let f_fov: f64 = 90.0;
    let f_aspect_ratio: f64 = HEIGHT as f64/WIDTH as f64;
    let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * PI).tan();

    let mat_proj = Mat4x4([[f_aspect_ratio * f_fov_rad, 0.0, 0.0, 0.0],
        [0.0, f_fov_rad, 0.0, 0.0],
        [0.0, 0.0, f_far / (f_far - f_near), 1.0],
        [0.0, 0.0, (-f_far * f_near) / (f_far - f_near), 0.0]
    ]);

    /*let mut mesh_cube = Mesh {
        tris: vec![
            // SOUTH
            Triangle {
                a: Vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: Vec3d { x: 0.0, y: 1.0, z: 0.0 },
                c: Vec3d { x: 1.0, y: 1.0, z: 0.0 },
            },
            Triangle {
                a: Vec3d { x: 0.0, y: 0.0, z: 0.0 },
                b: Vec3d { x: 1.0, y: 1.0, z: 0.0 },
                c: Vec3d { x: 1.0, y: 0.0, z: 0.0 },
            },

            // EAST
            Triangle {
                a: Vec3d { x: 1.0, y: 0.0, z: 0.0 },
                b: Vec3d { x: 1.0, y: 1.0, z: 0.0 },
                c: Vec3d { x: 1.0, y: 1.0, z: 1.0 },
            },
            Triangle {
                a: Vec3d { x: 1.0, y: 0.0, z: 0.0 },
                b: Vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: Vec3d { x: 1.0, y: 0.0, z: 1.0 },
            },

            //NORTH
            Triangle {
                a: Vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: Vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: Vec3d { x: 0.0, y: 1.0, z: 1.0 },
            },
            Triangle {
                a: Vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: Vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: Vec3d { x: 0.0, y: 0.0, z: 1.0 },
            },

            // WEST
            Triangle {
                a: Vec3d { x: 0.0, y: 0.0, z: 1.0 },
                b: Vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: Vec3d { x: 0.0, y: 1.0, z: 0.0 },
            },
            Triangle {
                a: Vec3d { x: 0.0, y: 0.0, z: 1.0 },
                b: Vec3d { x: 0.0, y: 1.0, z: 0.0 },
                c: Vec3d { x: 0.0, y: 0.0, z: 0.0 },
            },

            // TOP
            Triangle {
                a: Vec3d { x: 0.0, y: 1.0, z: 0.0 },
                b: Vec3d { x: 0.0, y: 1.0, z: 1.0 },
                c: Vec3d { x: 1.0, y: 1.0, z: 1.0 },
            },
            Triangle {
                a: Vec3d { x: 0.0, y: 1.0, z: 0.0 },
                b: Vec3d { x: 1.0, y: 1.0, z: 1.0 },
                c: Vec3d { x: 1.0, y: 1.0, z: 0.0 },
            },

            // BOTTOM
            Triangle {
                a: Vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: Vec3d { x: 0.0, y: 0.0, z: 1.0 },
                c: Vec3d { x: 0.0, y: 0.0, z: 0.0 },
            },
            Triangle {
                a: Vec3d { x: 1.0, y: 0.0, z: 1.0 },
                b: Vec3d { x: 0.0, y: 0.0, z: 0.0 },
                c: Vec3d { x: 1.0, y: 0.0, z: 0.0 },
            },
        ],
    };*/

    let mut mesh_cube = Mesh { tris: vec![] };

    mesh_cube.load_from_object_file("C:/Users/Cysie/CLionProjects/Renderer_V2/src/monke.obj");
    /*mesh_cube.load_from_object_file("C:/Users/Cysie/CLionProjects/Renderer_V2/src/untitled.obj");*/

    let mut mat_rot_z: Mat4x4;
    let mut mat_rot_x: Mat4x4;
    let mut f_theta:f64;
    let v_camera = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0
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

    window.set_position(350, 20);
    /*window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));*/

    let start = Instant::now();

    while window.is_open() && !window.is_key_down(Escape) {
        f_theta = start.elapsed().as_secs_f64();

        // Rotation Z
        mat_rot_z = Mat4x4([
            [f_theta.cos(), (f_theta).sin(), 0.0, 0.0],
            [-(f_theta).sin(), (f_theta).cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // Rotation X
        mat_rot_x = Mat4x4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, (f_theta * 0.5).cos(), (f_theta * 0.5).sin(), 0.0],
            [0.0, -(f_theta * 0.5).sin(), (f_theta * 0.5).cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let mut vec_triangles_to_raster:Vec<Triangle> = vec![];

        for tri in &mesh_cube.tris{
            let mut tri_projected: Triangle;
            let mut tri_translated: Triangle;
            let mut tri_rotated_zx: Triangle;
            let mut tri_rotated_z: Triangle;
            tri_projected = *tri;
            tri_rotated_z = *tri;
            tri_rotated_zx = *tri;

            multiply_matric_vector(&tri.a, &mut tri_rotated_z.a, &mat_rot_z);
            multiply_matric_vector(&tri.b, &mut tri_rotated_z.b, &mat_rot_z);
            multiply_matric_vector(&tri.c, &mut tri_rotated_z.c, &mat_rot_z);

            multiply_matric_vector(&tri_rotated_z.a, &mut tri_rotated_zx.a, &mat_rot_x);
            multiply_matric_vector(&tri_rotated_z.b, &mut tri_rotated_zx.b, &mat_rot_x);
            multiply_matric_vector(&tri_rotated_z.c, &mut tri_rotated_zx.c, &mat_rot_x);

            tri_translated = tri_rotated_zx;
            tri_translated.a.z = tri_rotated_zx.a.z + 3.0;
            tri_translated.b.z = tri_rotated_zx.b.z + 3.0;
            tri_translated.c.z = tri_rotated_zx.c.z + 3.0;

            let line1 = Vec3d {
                x: tri_translated.b.x - tri_translated.a.x,
                y: tri_translated.b.y - tri_translated.a.y,
                z: tri_translated.b.z - tri_translated.a.z,
                w: 0.0
            };

            let line2 = Vec3d {
                x: tri_translated.c.x - tri_translated.a.x,
                y: tri_translated.c.y - tri_translated.a.y,
                z: tri_translated.c.z - tri_translated.a.z,
                w: 0.0
            };

            let mut normal = Vec3d {
                x: line1.y * line2.z - line1.z * line2.y,
                y: line1.z * line2.x - line1.x * line2.z,
                z: line1.x * line2.y - line1.y * line2.x,
                w: 0.0
            };

            let l = (normal.x*normal.x + normal.y*normal.y + normal.z*normal.z).sqrt();
            normal.x /= l; normal.y /= l; normal.z /= l;

            if (normal.x * (tri_translated.a.x - v_camera.x) +
                normal.y * (tri_translated.a.y - v_camera.y) +
                normal.z * (tri_translated.a.z - v_camera.z)) < 0.0 {
                let mut light_direction = Vec3d { x: 0.0, y: 0.0, z: -1.0, w: 0.0 };
                let l = (light_direction.x*light_direction.x + light_direction.y*light_direction.y + light_direction.z*light_direction.z).sqrt();
                light_direction.x /= l; light_direction.y /= l; light_direction.z /= l;

                let dp = normal.x * light_direction.x + normal.y * light_direction.y + normal.z * light_direction.z;
                tri_projected.col = (255.0*dp) as u32 * 0x10101;

                multiply_matric_vector(tri_translated.a, &mat_proj);
                multiply_matric_vector(tri_translated.b,  &mat_proj);
                multiply_matric_vector(tri_translated.c, &mat_proj);

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

                vec_triangles_to_raster.push(tri_projected);
            }
        }

        vec_triangles_to_raster.sort_by(|x,y| (-(x.a.z+x.b.z+x.c.z)/3.0).partial_cmp(&(-(y.a.z+y.b.z+y.c.z)/3.0)).unwrap());

        for tri in &vec_triangles_to_raster{

            draw_triangle_edges(&mut buffer,
                                [tri.a.x as i32, tri.a.y as i32],
                                [tri.b.x as i32, tri.b.y as i32],
                                [tri.c.x as i32, tri.c.y as i32],
                                0x00ff00);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).expect("Oops!");
        buffer.clear();
        buffer.resize(WIDTH*HEIGHT,0);
    }
}
