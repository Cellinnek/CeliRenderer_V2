use std::fs;

use crate::WIDTH;
use crate::HEIGHT;
use std::mem::swap;
use std::f64::consts::PI;

pub fn line(buffer: &mut [u32], [argx1,argy1]: [i32; 2], [argx2,argy2]: [i32; 2], color: u32) {
    let mut x = argx1;
    let mut y = argy1;

    let dx = if argx1 > argx2 {
        argx1 - argx2
    } else {
        argx2 - argx1
    };
    let dy = if argy1 > argy2 {
        argy1 - argy2
    } else {
        argy2 - argy1
    };

    let sx = if argx1 < argx2 { 1 } else { -1 };
    let sy = if argy1 < argy2 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err_tolerance;

    loop {
        if (x as usize)<WIDTH && (y as usize)<HEIGHT{
            buffer[(y*(WIDTH as i32) + x) as usize] = color;
        };

        if x == argx2 && y == argy2 {
            break;
        };

        err_tolerance = err;

        if err_tolerance > -dx {
            err -= dy;
            x += sx;
        }
        if err_tolerance < dy {
            err += dx;
            y += sy;
        }
    }
}

pub fn draw_triangle_faces(buffer: &mut [u32], [mut x1,mut y1]: [i32; 2], [mut x2,mut y2]: [i32; 2], [mut x3,mut y3]: [i32; 2], color:u32){
    let height = HEIGHT as i32;
    let width = WIDTH as i32;

    if y2 > y3
    {
        swap(&mut x2,&mut x3);
        swap(&mut y2,&mut y3);
    }
    if y1 > y2
    {
        swap(&mut x1,&mut x2);
        swap(&mut y1,&mut y2);
    }
    if y2 > y3
    {
        swap(&mut x2,&mut x3);
        swap(&mut y2,&mut y3);
    }

    let dx_far = (x3 - x1) as f32/ (y3 - y1 + 1) as f32;
    let dx_upper = (x2 - x1) as f32 / (y2 - y1 + 1) as f32;
    let dx_low = (x3 - x2) as f32 / (y3 - y2 + 1) as f32;
    let mut xf = x1 as f32;
    let mut xt = x1 as f32 + dx_upper;
    for y in y1..(if y3<height-1{y3} else{height-1}) {
        if y >= 0 {
            for x in (if xf>0.0{xf as i32} else{0})..(if xt < (width-1) as f32{xt as i32} else{width-1}){
                buffer[(x+y*width) as usize] = color;
            }
            for x in (if xt > 0.0{xt as i32} else{0})..(if xf<width as f32{xf as i32} else{width-1}){
                buffer[(x+y*width) as usize] = color;
            }
        }
        xf += dx_far;
        if y < y2{xt += dx_upper;}
        else{ xt += dx_low;}
    }
}

pub fn draw_triangle_edges(buffer: &mut [u32], [x1,y1]: [i32; 2], [x2,y2]: [i32; 2], [x3,y3]: [i32; 2], color:u32){
    line(buffer, [x1,y1],[x2,y2], color);
    line(buffer, [x2,y2],[x3,y3], color);
    line(buffer, [x3,y3],[x1,y1], color);
}

#[derive(Clone,Copy)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

#[derive(Clone,Copy)]
pub struct Triangle {
    pub a: Vec3d,
    pub b: Vec3d,
    pub c: Vec3d,
    pub col: u32,
}


pub struct Mesh {
    pub tris: Vec<Triangle>
}
impl Mesh {
    pub fn load_from_object_file(&mut self, path: &str){
        let file = fs::read_to_string(path).unwrap();
        let split = file.split('\n');
        let mut verts:Vec<Vec3d> = vec![];

        for s in split{
            if s.split_whitespace().next().unwrap() == "v"{
                verts.push(Vec3d {
                    x: s.split_whitespace().nth(1).unwrap().parse::<f64>().unwrap(),
                    y: s.split_whitespace().nth(2).unwrap().parse::<f64>().unwrap(),
                    z: s.split_whitespace().nth(3).unwrap().parse::<f64>().unwrap(),
                    w: 0.0
                });
            }

            if s.split_whitespace().next().unwrap() == "f"{
                let f = [
                    s.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap(),
                    s.split_whitespace().nth(2).unwrap().parse::<usize>().unwrap(),
                    s.split_whitespace().nth(3).unwrap().parse::<usize>().unwrap(),
                    ];
                self.tris.push(Triangle {
                    a: verts[f[0] - 1],
                    b: verts[f[1] - 1],
                    c: verts[f[2] - 1],
                    col: 0,
                });
            }
        }
    }
}

pub struct Mat4x4(pub [[f64; 4]; 4]);

pub fn matrix_multiply_vector(m: &Mat4x4, i: &Vec3d) -> Vec3d{
    Vec3d{
        x: i.x * m.0[0][0] + i.y * m.0[1][0] + i.z * m.0[2][0] + i.w * m.0[3][0],
        y: i.x * m.0[0][1] + i.y * m.0[1][1] + i.z * m.0[2][1] + i.w * m.0[3][1],
        z: i.x * m.0[0][2] + i.y * m.0[1][2] + i.z * m.0[2][2] + i.w * m.0[3][2],
        w: i.x * m.0[0][3] + i.y * m.0[1][3] + i.z * m.0[2][3] + i.w * m.0[3][3]
    }
}

pub fn matrix_make_identity() -> Mat4x4 {
    let mut matrix = Mat4x4([[0.0;4];4]);
    matrix.0[0][0] = 1.0;
    matrix.0[1][1] = 1.0;
    matrix.0[2][2] = 1.0;
    matrix.0[3][3] = 1.0;
    return matrix;
}

pub fn matrix_make_rotation_x(f_angle_rad: f64) -> Mat4x4 {
    let mut matrix = Mat4x4([[0.0;4];4]);
    matrix.0[0][0] = 1.0;
    matrix.0[1][1] = f_angle_rad.cos();
    matrix.0[1][2] = f_angle_rad.sin();
    matrix.0[2][1] = -f_angle_rad.sin();
    matrix.0[2][2] = f_angle_rad.cos();
    matrix.0[3][3] = 1.0;
    return matrix;
}

pub fn matrix_make_rotation_y(f_angle_rad: f64) -> Mat4x4 {
    let mut matrix = Mat4x4([[0.0;4];4]);
    matrix.0[0][0] = f_angle_rad.cos();
    matrix.0[0][2] = f_angle_rad.sin();
    matrix.0[2][0] = -f_angle_rad.sin();
    matrix.0[1][1] = 1.0;
    matrix.0[2][2] = f_angle_rad.cos();
    matrix.0[3][3] = 1.0;
    return matrix;
}

pub fn matrix_make_rotation_z(f_angle_rad: f64) -> Mat4x4 {
    let mut matrix = Mat4x4([[0.0;4];4]);
    matrix.0[0][0] = f_angle_rad.cos();
    matrix.0[0][1] = f_angle_rad.sin();
    matrix.0[1][0] = -f_angle_rad.sin();
    matrix.0[1][1] = f_angle_rad.cos();
    matrix.0[2][2] = 1.0;
    matrix.0[3][3] = 1.0;
    return matrix;
}

pub fn matrix_make_translation(x:f64,y:f64,z:f64) -> Mat4x4{
    let mut matrix = Mat4x4([[0.0;4];4]);
    matrix.0[0][0] = 1.0;
    matrix.0[1][1] = 1.0;
    matrix.0[2][2] = 1.0;
    matrix.0[3][3] = 1.0;
    matrix.0[3][0] = x;
    matrix.0[3][1] = y;
    matrix.0[3][2] = z;
    return matrix;
}

pub fn matrix_make_projection(f_fov_degrees: f64, f_aspect_ratio: f64, f_near: f64, f_far: f64) -> Mat4x4 {
    let f_fov_rad = 1.0 / (f_fov_degrees * 0.5 / 180.0 * PI).tan();
    let mut matrix = Mat4x4([[0.0;4];4]);
    matrix.0[0][0] = f_aspect_ratio * f_fov_rad;
    matrix.0[1][1] = f_fov_rad;
    matrix.0[2][2] = f_far / (f_far - f_near);
    matrix.0[3][2] = (-f_far * f_near) / (f_far - f_near);
    matrix.0[2][3] = 1.0;
    matrix.0[3][3] = 0.0;
    return matrix;
}

pub fn matrix_multiply_matrix(m1: &Mat4x4, m2: &Mat4x4) -> Mat4x4 {
    let mut matrix = Mat4x4([[0.0;4];4]);
    for c in 0..4 {
        for r in 0..4 {
            matrix.0[r][c] = m1.0[r][0] * m2.0[0][c] + m1.0[r][1] * m2.0[1][c] + m1.0[r][2] * m2.0[2][c] + m1.0[r][3] * m2.0[3][c];
        }
    }
    return matrix;
}

pub fn vector_add(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    Vec3d{x: v1.x + v2.x,y: v1.y + v2.y ,z: v1.z + v2.z, w: 1.0 }
}

pub fn vector_sub(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    Vec3d{x: v1.x - v2.x,y: v1.y - v2.y ,z: v1.z - v2.z, w: 1.0 }
}

pub fn vector_mul(v1: &Vec3d, k: &f64) -> Vec3d {
    Vec3d{x: v1.x * k,y: v1.y * k ,z: v1.z * k, w: 1.0 }
}

pub fn vector_div(v1: &Vec3d, k: &f64) -> Vec3d {
    Vec3d{x: v1.x / k,y: v1.y / k ,z: v1.z / k, w: 1.0 }
}

pub fn vector_dot(v1: &Vec3d, v2: &Vec3d) -> f64 {
    (v1.x*v2.x+v1.y*v2.y+v1.z*v2.z)
}

pub fn vector_len(v: &Vec3d) -> f64 {
    (vector_dot(&v,&v)).sqrt()
}

pub fn vector_norm(v: &Vec3d) -> Vec3d {
    let l = vector_len(v);
    Vec3d{x: v.x/l, y: v.y/l, z: v.z/l, w: 1.0 }
}

pub fn vector_cross(v1: &Vec3d ,v2: &Vec3d) -> Vec3d {
    Vec3d{
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
        w: 1.0
    }
}