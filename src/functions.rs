use crate::WIDTH;
use crate::HEIGHT;
use std::mem::swap;

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

pub fn rotate(arr: &mut [[f64;3]], r: &[[f64; 3]], fi: f64, axis: u8) {
    let o = r[0];
    match axis%3{
        0 => for i in arr {
            let (y,z) = (i[1]-o[1],i[2]-o[2]);
            i[2] = z*fi.cos()-y*fi.sin()+o[2];
            i[1] = z*fi.sin()+y*fi.cos()+o[1];},
        1 => for i in arr {
            let (x,z) = (i[0]-o[0],i[2]-o[2]);
            i[0] = x*fi.cos()-z*fi.sin()+o[0];
            i[2] = x*fi.sin()+z*fi.cos()+o[2];},
        2 => for i in arr {
            let (x,y) = (i[0]-o[0],i[1]-o[1]);
            i[1] = y*fi.cos()-x*fi.sin()+o[1];
            i[0] = y*fi.sin()+x*fi.cos()+o[0];},
        _ => println!("Axis error!")
    }
}

#[derive(Clone)]
pub struct vec3d{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Clone)]
pub struct triangle(pub vec3d,pub vec3d,pub vec3d);

pub struct mesh {
    pub tris: Vec<triangle>
}

pub struct mat4x4{
    pub m: [[f64;4];4],
}

pub fn MultiplyMatricVector(i: &vec3d, o: &mut vec3d, m: &mat4x4){
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
    let w: f64 = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];

    if w != 0.0 {
        o.x /= w; o.y /= w; o.z /= w;
    }
}