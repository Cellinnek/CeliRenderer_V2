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
    let mut mat_proj = matrix_make_projection(90.0, HEIGHT as f64 / WIDTH as f64, 0.1, 1000.0);

    let mut mesh_cube = Mesh { tris: vec![] };

    mesh_cube.load_from_object_file("C:/Users/Cysie/CLionProjects/Renderer_V2/src/VideoShip.obj");
    /*mesh_cube.load_from_object_file("C:/Users/Cysie/CLionProjects/Renderer_V2/src/untitled.obj");*/

    let mut f_theta:f64;
    let v_camera = Vec3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0
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

        let mat_rot_z = matrix_make_rotation_z(f_theta * 0.5);
        let mat_rot_x = matrix_make_rotation_x(f_theta);

        let mat_trans = matrix_make_translation(0.0, 0.0, 16.0);

        let mut mat_world = matrix_make_identity();
        mat_world = matrix_multiply_matrix(&mat_rot_z,&mat_rot_x);
        mat_world = matrix_multiply_matrix(&mat_world, &mat_trans);

        let mut vec_triangles_to_raster:Vec<Triangle> = vec![];

        for tri in &mesh_cube.tris{
            let mut tri_projected = *tri;
            let mut tri_transformed = Triangle{
                a: matrix_multiply_vector(&mat_world, &tri.a),
                b: matrix_multiply_vector(&mat_world, &tri.b),
                c: matrix_multiply_vector(&mat_world, &tri.c),
                col: 0
            };

            let line1 = vector_sub(&tri_transformed.b, &tri_transformed.a);
            let line2 = vector_sub(&tri_transformed.c, &tri_transformed.a);

            let normal = vector_norm(&vector_cross(&line1,&line2));

            let v_camera_ray = vector_sub(&tri_transformed.a, &v_camera);

            if vector_dot(&normal, &v_camera_ray) < 0.0 {

                let light_direction = vector_norm(&Vec3d{ x: 0.0, y: 0.0, z: -1.0, w: 1.0});

                let dp = vector_dot(&light_direction, &normal).max(0.1);
                tri_projected.col = (255.0*dp) as u32 * 0x10101;

                tri_projected.a = matrix_multiply_vector(&mat_proj, &tri_transformed.a);
                tri_projected.b = matrix_multiply_vector(&mat_proj, &tri_transformed.b);
                tri_projected.c = matrix_multiply_vector(&mat_proj, &tri_transformed.c);

                tri_projected.a = vector_div(&tri_projected.a, &tri_projected.a.w);
                tri_projected.b = vector_div(&tri_projected.b, &tri_projected.b.w);
                tri_projected.c = vector_div(&tri_projected.c, &tri_projected.c.w);

                let offset = Vec3d{
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                    w: 1.0
                };
                tri_projected.a = vector_add(&tri_projected.a, &offset);
                tri_projected.b = vector_add(&tri_projected.b, &offset);
                tri_projected.c = vector_add(&tri_projected.c, &offset);

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
            draw_triangle_faces(&mut buffer,
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
