// #![allow(unused)]
use macroquad::texture::{draw_texture, Texture2D};
use macroquad::input::{is_key_down, is_mouse_button_down};
use macroquad::miniquad::{KeyCode, MouseButton};
use macroquad::prelude::Conf;
use macroquad::{color, window};
use instant::{Duration, Instant};
const NAME: &str = "PoopyPoop";
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const WHITE: u32 = 0xff_ff_ff;
const BLACK: u32 = 0x00_00_00;
const _GREY: u32 = 0x88_88_88;
const RED: u32 = 0xff_00_00;
const LIGHT_GREY: u32 = 0xcc_cc_cc;
const CHARWIDTH: usize = 3;
const _A: &[bool] = &trans([
    true, true, true,
    true, true, true,
    true, false, true
]);
const B: &[bool] = &trans([
    true, false, false,
    true, true, true,
    true, true, true
]);
const _C: &[bool] = &trans([
    true, true, true,
    true, false, false,
    true, true, true
]);
const _E: &[bool] = &trans([
    true, true, true,
    true, true, false,
    true, true, true
]);
const _H: &[bool] = &trans([
    true, false, true,
    true, true, true,
    true, false, true
]);
const _L: &[bool] = &trans([
    true, false, false,
    true, false, false,
    true, true, true
]);
const O: &[bool] = &trans([
    true, true, true,
    true, false, true,
    true, true, true
]);
const _S: &[bool] = &trans([
    false, true, true,
    false, true, false,
    true, true, false
]);
const _X: &[bool] = &trans([
    true, false, true,
    false, true, false,
    true, false, true
]);
const _Y: &[bool] = &trans([
    true, false, true,
    false, true, false,
    false, true, false
]);
const _SPACE: &[bool] = &trans([
    false, false, false,
    false, false, false,
    false, false, false
]);
const _LT: &[bool] = &trans([
    false, true, false,
    true, false, false,
    false, true, false
]);
const ZERO: &[bool] = &trans([
    true, true, true,
    true, false, true,
    true, true, true
]);
const ONE: &[bool] = &trans([
    true, true, false,
    false, true, false,
    true, true, true
]);
const TWO: &[bool] = &trans([
    true, true, false,
    false, true, false,
    false, true, true
]);
const THREE: &[bool] = &trans([
    true, true, true,
    false, true, true,
    true, true, true
]);
const FOUR: &[bool] = &trans([
    true, false, true,
    true, true, true,
    false, false, true
]);
const FIVE: &[bool] = &trans([
    true, true, true,
    false, true, false,
    true, true, false
]);
const SIX: &[bool] = &trans([
    true, false, false,
    true, true, false,
    true, true, false
]);
const SEVEN: &[bool] = &trans([
    true, true, true,
    false, false, true,
    false, false, true
]);
const EIGHT: &[bool] = &trans([
    true, true, false,
    true, true, true,
    false, true, true
]);
const NINE: &[bool] = &trans([
    false, true, true,
    false, true, true,
    false, false, true
]);
const NUMBERS: [&[bool]; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

const fn trans(input: [bool; CHARWIDTH*CHARWIDTH]) -> [bool; CHARWIDTH*CHARWIDTH] {
    let t = [0,3,6,1,4,7,2,5,8];
    let mut out = [false; CHARWIDTH*CHARWIDTH];
    let mut i = 0;
    while i<out.len() {
        out[i] = input[t[i]];
        i += 1;
    }
    out
}

fn window_conf() -> Conf {
    Conf{
        window_title: NAME.to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[inline] 
fn index(x: usize, y: usize) -> usize {
    y*WIDTH + x
}

// fn fill(pixels: &mut[u32], colour: u32) {
//     for i in 0..WIDTH*HEIGHT {
//         pixels[i] = colour;
//     }
// }

// fn draw_grid(pixels: &mut[u32]) {
//     for x in (0..WIDTH).step_by(10) {
//         for y in 0..HEIGHT {
//             pixels[index(x,y)] = BLACK;
//         }
//     }
//     for x in 0..WIDTH {
//         for y in (0..HEIGHT).step_by(10) {
//             pixels[index(x,y)] = BLACK;
//         }
//     }
// }

fn draw_dotgrid(pixels: &mut[u32]) {
    for x in (0..WIDTH).step_by(10) {
        for y in (0..HEIGHT).step_by(10) {
            pixels[index(x,y)] = BLACK;
        }
    }
}

fn draw_rect(pixels: &mut[u32], startx: usize, starty: usize, xlen: usize, ylen: usize, colour: u32) {
    for x in startx..startx+xlen {
        for y in starty..starty+ylen {
            pixels[index(x,y)] = colour;
        }
    }
}

#[inline]
fn draw_pixel(pixels: &mut[u32], x: usize, y: usize, colour: u32) {
    pixels[index(x,y)] = colour;
}

// fn _draw_region(pixels: &mut[u32], startx: usize, starty: usize, xlen: usize, ylen: usize, reg: &[u32]) {
//     let mut i = 0;
//     for x in startx..startx+xlen {
//         for y in starty..starty+ylen {
//             pixels[index(x,y)] = reg[i];
//             i += 1;
//         }
//     }
// }

// fn draw_region_part(pixels: &mut[u32], startx: usize, starty: usize, xlen: usize, ylen: usize, reg: &[u32], reg_width: usize) {
//     for x in startx..startx+xlen {
//         for y in starty..starty+ylen {
//             pixels[index(x,y)] = reg[y*reg_width + x];
//         }
//     }
// }

// fn dump_region(pixels: &[u32], startx: usize, starty: usize, xlen: usize, ylen: usize) -> Vec<u32> {
//     let mut out = vec![0; xlen*ylen];
//     let mut i = 0;
//     for x in startx..startx+xlen {
//         for y in starty..starty+ylen {
//             out[i] = pixels[index(x,y)];
//             i += 1;
//         }
//     }
//     out
// }

fn draw_char(pixels: &mut[u32], startx: usize, starty: usize, thickness: usize, c: &[bool], colour: u32) {
    let mut i = 0;
    for x in (startx..startx+3*thickness).step_by(thickness) {
        for y in (starty..starty+3*thickness).step_by(thickness) {
            if c[i] {
                draw_rect(pixels, x, y, thickness, thickness, colour);
            }
            i += 1;
        }
    }
}

fn draw_text(pixels: &mut[u32], startx: usize, starty: usize, thickness: usize, spacing: usize, cs: &[&[bool]], colour: u32) {
    for i in 0..cs.len() {
        draw_char(pixels, startx+i*(thickness*CHARWIDTH+spacing), starty, thickness, cs[i], colour);
    }
}

fn draw_num(pixels: &mut[u32], startx: usize, starty: usize, thickness: usize, spacing: usize, mut num: u32, colour: u32) {
    let len = if num==0 {1} else {num.ilog10() as usize + 1};
    let mut cs = vec![ZERO; len];
    for i in (0..cs.len()).rev() {
        let digit = (num % 10) as usize;
        cs[i] = NUMBERS[digit];
        num /= 10;
    }
    draw_text(pixels, startx, starty, thickness, spacing, &cs, colour);
}

fn draw_bob(pixels: &mut[u32], startx: usize, starty: usize, xlen: usize, ylen: usize, colour_body: u32, colour_detail: u32) {
    draw_rect(pixels, startx, starty, xlen, ylen, colour_body); // body
    draw_rect(pixels, startx + xlen*0/100, starty + ylen*18/100, xlen*40/100, ylen*10/100, colour_detail); // left eye
    draw_rect(pixels, startx + xlen*80/100, starty + ylen*20/100, xlen*20/100, ylen*5/100, colour_detail); // right eye
    draw_rect(pixels, startx, starty + ylen*50/100, xlen, ylen*5/100, colour_detail); // mouth

}

fn draw_heart(pixels: &mut[u32], startx: usize, starty: usize, width: usize, height: usize, colour: u32) {
    draw_rect(pixels, startx + width*1/7, starty + height*1/8, width*5/7, height*5/8, colour); // body
    draw_rect(pixels, startx + width*6/70 , starty + height*2/8, width*5/70, height*2/8, colour); // left flap
    draw_rect(pixels, startx + width*6/7, starty + height*2/8, width*5/70, height*2/8, colour); // right flap
    draw_rect(pixels, startx + width*2/7, starty + height*5/80, width*1/7, height*5/80, colour); // left top
    draw_rect(pixels, startx + width*4/7, starty + height*5/80, width*1/7, height*5/80, colour); // right top
    draw_rect(pixels, startx + width*2/7, starty + height*6/8, width*3/7, height*1/8, colour); // bottom flap
    draw_rect(pixels, startx + width*3/7, starty + height*7/8, width*1/7, height*1/8, colour); // bottom
}

async fn window_update_with_buffer(pixels: &[u32]) {
    let bytes = pixels.iter().flat_map(|pixel| {
        let [_, red, green, blue] = pixel.to_be_bytes();
        [red, green, blue, 255]
    }).collect::<Vec<u8>>();
    let texture = Texture2D::from_rgba8(WIDTH as u16, HEIGHT as u16, &bytes);
    draw_texture(&texture, 0.0, 0.0, color::WHITE); // IMPORTANT: KEEP THIS TO WHITE!!!
    window::next_frame().await
}

static mut SEED: [u32;4] = [1,3,3,7];
fn rands() -> [u32;4] {
    let [mut a,mut b,mut c,mut d] = unsafe {SEED};
    a = a.wrapping_add(b);
    d = (a^d).rotate_right(16);
    c = c.wrapping_add(d);
    b = (b^c).rotate_right(12);
    a = a.wrapping_add(b);
    d = (a^d).rotate_right(8);
    c = c.wrapping_add(d);
    b = (b^c).rotate_right(7);
    unsafe {SEED = [a,b,c,d]};
    [a,b,c,d]
}

// fn is_overlapping(x1: usize, y1: usize, w1: usize, h1:usize, x2: usize, y2: usize, w2: usize, h2: usize) -> bool {
//     // Check if one rectangle is to the left of the other
//     if x1 + w1 < x2 {
//         return false;
//     }
//     // Check if one rectangle is to the right of the other
//     if x2 + w2 < x1 {
//         return false;
//     }
//     // Check if one rectangle is above the other
//     if y1 + h1 < y2 {
//         return false;
//     }
//     // Check if one rectangle is below the other
//     if y2 + h2 < y1 {
//         return false;
//     }
    
//     // If none of the above are true, the rectangles must overlap
//     true
// }

fn elapsed_secs(elapsed: &Duration) -> usize {
    const IS_WASM32: bool = cfg!(target_arch = "wasm32");
    if IS_WASM32 {elapsed.as_millis() as usize} else {elapsed.as_secs() as usize}
}

fn elapsed_millis(elapsed: &Duration) -> usize {
    const IS_WASM32: bool = cfg!(target_arch = "wasm32");
    if IS_WASM32 {elapsed.as_micros() as usize} else {elapsed.as_millis() as usize}
}

#[derive(Clone, Copy)]
struct Vector2D {
    x: f32,
    y: f32
}

#[derive(Clone, Copy)]
struct Vector3D {
    x: f32,
    y: f32,
    z: f32
}

struct _Camera {
    pos: Vector3D, // where is the camera positioned
    rot: Vector3D, // where is the camera looking at
    fov: f32 // field of view angle of the camera
}

fn _project_orthographic(points3d: &[Vector3D], fov_scale: f32, translation_width: f32, translation_height: f32) -> Vec<Vector2D> {
    let mut points2d = vec![Vector2D{x: 0.0, y: 0.0}; points3d.len()];
    for i in 0..points3d.len() {
        points2d[i].x = points3d[i].x * fov_scale + translation_width;
        points2d[i].y = points3d[i].y * fov_scale + translation_height;
    }
    points2d
}

fn _draw_projection(pixels: &mut[u32], points: &[Vector2D], colour: u32) {
    for i in 0..points.len() {
        draw_rect(pixels, points[i].x as usize, points[i].y as usize, 3, 3, colour);
    }
}

fn _project_isometric(points3d: &[Vector3D], fov_scale: f32, translation_width: f32, translation_height: f32) -> Vec<Vector2D> {
    let mut points2d = vec![Vector2D{x: 0.0, y: 0.0}; points3d.len()];
    for i in 0..points3d.len() {
        points2d[i].x = (points3d[i].x - points3d[i].z) * fov_scale + translation_width;
        points2d[i].y = (points3d[i].x / 2.0 + points3d[i].y + points3d[i].z / 2.0) * fov_scale + translation_height;
    }
    points2d
}

fn rotate2d(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let (sina, cosa) = (angle.sin(), angle.cos());
    (x*cosa - y*sina, x*sina + y*cosa)
}

fn rotate3d(mut x: f32, mut y:f32, mut z: f32, xangle: f32, yangle: f32, zangle: f32) -> (f32, f32, f32) {
    (y,z) = rotate2d(y, z, xangle);
    (x,z) = rotate2d(x, z, yangle);
    (x,y) = rotate2d(x, y, zangle);
    (x, y, z)
}

fn project_perspective(points: &[Vector3D], fov_scale: f32, translation_width: f32, translation_height: f32, translation_depth: f32, xrotation: f32, yrotation: f32, zrotation: f32) -> Vec<Vector2D> {
    let mut projection = vec![Vector2D{x: 0.0, y: 0.0}; points.len()];
    for i in 0..points.len() {
        let (x, y, z) = rotate3d(points[i].x, points[i].y, points[i].z, xrotation, yrotation, zrotation);
        projection[i].x = (x / (z + translation_depth)) * fov_scale + translation_width; // scaledown assuming fov-z is 1.0
        projection[i].y = (y / (z + translation_depth)) * fov_scale + translation_height; // scaledown assuming fov-z is 1.0
    }
    projection
}

fn oscillate(x: f32, max: f32) -> f32 {
    (( x % (max*2.0) - max).abs() - max).abs()
}

fn _model_cube_dotted() -> Vec<Vector3D> {
    let mut cube = vec![Vector3D{x: 0.0,y: 0.0,z: 0.0}; 9*9*9];
    let mut i = 0;
    for x in 0..9 {
        for y in 0..9 {
            for z in 0..9 {
                cube[i].x = -1.0 + x as f32 * 0.25;
                cube[i].y = -1.0 + y as f32 * 0.25;
                cube[i].z = -1.0 + z as f32 * 0.25;
                i += 1;
            }
        }
    }
    cube
}

fn model_cube_mesh() -> (Vec<Vector3D>, Vec<Triangle>) {
    let mut cube = vec![Vector3D{x: 0.0, y: 0.0, z: 0.0}; 8];
    let mut i = 0;
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                cube[i].x = -1.0 + x as f32 * 2.0;
                cube[i].y = -1.0 + y as f32 * 2.0;
                cube[i].z = -1.0 + z as f32 * 2.0;
                println!("{i} -> {} {} {}", cube[i].x, cube[i].y, cube[i].z);
                i += 1;
            }
        }
    }
    // WARNING: the triangles have not been adjusted for rotation order!
    let faces = vec![
        // LEFT RIGHT
        Triangle{a: 0, b: 1, c: 3},
        Triangle{a: 0, b: 2, c: 3},
        Triangle{a: 4, b: 5, c: 7},
        Triangle{a: 4, b: 6, c: 7},

        // TOP BOTTOM
        Triangle{a: 0, b: 1, c: 5},
        Triangle{a: 0, b: 4, c: 5},
        Triangle{a: 2, b: 3, c: 7},
        Triangle{a: 2, b: 6, c: 7},

        // FRONT BACK
        Triangle{a: 0, b: 2, c: 4},
        Triangle{a: 2, b: 4, c: 6},
        Triangle{a: 1, b: 3, c: 5},
        Triangle{a: 3, b: 5, c: 7},
    ];
    (cube, faces)
}

#[derive(Clone, Copy)]
struct Triangle { // structure that encodes triangle face indexes in a mesh
    a: usize,
    b: usize,
    c: usize
}

fn draw_line(pixels: &mut[u32], mut x1: f32, mut y1: f32, x2: f32, y2: f32, colour: u32) {
    let xdelta = x2 - x1;
    let ydelta = y2 - y1;
    let walk = if xdelta.abs() >= ydelta.abs() { xdelta.abs() } else {ydelta.abs()};
    let xincrement = xdelta / walk;
    let yincrement = ydelta / walk;
    for _ in 0..walk as usize {
        draw_pixel(pixels, x1.round() as usize, y1.round() as usize, colour);
        x1 += xincrement;
        y1 += yincrement;
    }
}

fn draw_projection_with_mesh(pixels: &mut[u32], points: &[Vector2D], triangles: &[Triangle], colour: u32) {
    for i in 0..points.len() {
        draw_rect(pixels, points[i].x as usize, points[i].y as usize, 3, 3, colour);
    }

    for i in 0..triangles.len() {
        let point1 = points[triangles[i].a];
        let point2 = points[triangles[i].b];
        let point3 = points[triangles[i].c];
        draw_line(pixels, point1.x, point1.y, point2.x, point2.y, colour);
        draw_line(pixels, point2.x, point2.y, point3.x, point3.y, colour);
        draw_line(pixels, point3.x, point3.y, point1.x, point1.y, colour);
    }
}

#[macroquad::main(window_conf)]
#[warn(unused)]
async fn main() {
    let mut background = vec![WHITE; WIDTH*HEIGHT];
    draw_dotgrid(&mut background);
    draw_text(&mut background, WIDTH*70/100, HEIGHT*90/100, 10, 10, &[B,O,B], LIGHT_GREY);

    // CUBE mesh
    let (cube, triangles) = model_cube_mesh();

    let (mut fps,mut fps_timer, mut fps_counter) = (0, Instant::now(), 0);
    let (mut bobx, boby, bobwidth, bobheight, mut bob_timer) = (0, HEIGHT - 50, 20, 50, Instant::now());
    let (mut heartx, hearty, heartwidth, heartheight, mut heartcolour) = (WIDTH*20/100, HEIGHT - 50, 50, 50, RED); 
    let (mut cube_timer, mut cube_rot, mut cube_scale_i, mut cube_scale) = (Instant::now(), 0.0, 0.0, 0.0);
    // let mut global_timer = Instant::now();
    loop {
        let mut pixels = background.clone();

        // QUIT
        if is_key_down(KeyCode::Escape) {
            window::miniquad::window::quit();
            break;
        } 

        // MOUSE
        if is_mouse_button_down(MouseButton::Left) {
            heartcolour = rands().into_iter().fold(0, |x, y| x ^ y);
        }

        // FPS COUNTER
        fps_counter += 1;
        let elapsed = fps_timer.elapsed();
        if elapsed_secs(&elapsed) > 0 {
            fps = fps_counter;
            fps_counter = 0;
            fps_timer += elapsed;
        }
        draw_num(&mut pixels, 0, 0, 5, 3, fps, LIGHT_GREY);

        // BOB
        let elapsed = bob_timer.elapsed();
        if elapsed_millis(&elapsed) > 100 {
            bobx += bobwidth;
            if bobx + bobwidth > WIDTH {
                bobx = 0;
            }
            bob_timer += elapsed;
        }
        draw_bob(&mut pixels, bobx, boby, bobwidth, bobheight, BLACK, WHITE);

        // HEART
        if heartx >= bobx && heartx < bobx + bobwidth {
            heartx += 20;
            if heartx + heartwidth > WIDTH {
                heartx = WIDTH*30/100;
            }
        }
        draw_heart(&mut pixels, heartx, hearty, heartwidth, heartheight, heartcolour);

        // CUBE
        let elapsed = cube_timer.elapsed();
        if elapsed_millis(&elapsed) > 1000 / 30 * 0{
            cube_timer += elapsed;
            cube_rot += 0.01;
            cube_scale_i += 1.0;
            cube_scale = oscillate(cube_scale_i, (WIDTH+HEIGHT) as f32 / 3.0);
        }
        let projection = project_perspective(&cube,  cube_scale + 0.0 * (WIDTH+HEIGHT) as f32 / 2.0, WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, -5.0, cube_rot, cube_rot, cube_rot);
        draw_projection_with_mesh(&mut pixels, &projection, &triangles, BLACK);

        // GLOBAL FPS LOCK WITH SLEEP -----> DOES NOT WORK OVER WASM!
        // let elapsed = global_timer.elapsed();
        // if elapsed < Duration::from_millis(1000/10) {
        //     std::thread::sleep(Duration::from_millis(1000/10) - elapsed);
        // }
        // global_timer += elapsed;


        window_update_with_buffer(&pixels).await;
    }
}

// #[macroquad::main(window_conf)]
// async fn main() {
//     println!("Hello, world!");
//     let mut pixels = vec![WHITE; WIDTH*HEIGHT];
//     draw_dotgrid(&mut pixels);
//     fill(&mut pixels, WHITE);
//     draw_grid(&mut pixels);
//     draw_rect(&mut pixels, 10, 30, 50, 5, BLACK);
//     draw_rect(&mut pixels, 400, 50, 50, 100, GREY);
//     draw_rect(&mut pixels, WIDTH*50/100, HEIGHT*50/100, 100, 80, LIGHT_GREY);
//     draw_pixel(&mut pixels, 200, 200, BLACK);
//     draw_char(&mut pixels, 30, 30, 10, C, BLACK);
//     draw_char(&mut pixels, 70, 30, 10, A, BLACK);
//     draw_char(&mut pixels, 110, 30, 10, O, BLACK);
//     draw_char(&mut pixels, 150, 30, 10, S, BLACK);
//     draw_text(&mut pixels, 30, 80, 15, 15, &[C,A,O,S], BLACK);
//     draw_text(&mut pixels, 50, 150, 20, 15, &[S,E,X,Y], BLACK);
//     draw_num(&mut pixels, WIDTH*70/100, HEIGHT*70/100, 10, 10, 302, BLACK);
//     draw_text(&mut pixels, WIDTH*5/100, HEIGHT*60/100, 10, 10, &[H,E,L,L,O,SPACE], GREY);
//     draw_heart(&mut pixels, WIDTH*50/100 + 10, HEIGHT*55/100, 80, 50, WHITE);

//     let (mut reg, mut startx, mut starty, mut xlen, mut ylen) = (vec![], 0,0,0,0);
//     let (mut fps, fps_startxy, fps_thickness) = (0, 5, 10);
//     let (fps_xlen, fps_ylen) = (fps_thickness*CHARWIDTH*3, fps_thickness*CHARWIDTH);
//     let fps_reg = dump_region(&pixels, fps_startxy, fps_startxy, fps_xlen, fps_ylen);
//     let mut time = instant::Instant::now();
//     let bob_xlen = 40;
//     let bob_ylen = 4*bob_xlen;
//     let (mut bob_x, bob_y) = (0, HEIGHT - bob_ylen);
//     let mut bob_reg = dump_region(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen);
//     let mut bob_colour = BLACK;
//     let mut heart_colour = WHITE;
//     let mut last_updated = "BOB";
//     loop {
//         // QUIT
//         if is_key_down(KeyCode::Escape) {
//             macroquad::window::miniquad::window::quit();
//             break;
//         } 

//         // RANDOM SQUARE
//         if is_key_down(KeyCode::Space) || is_key_released(KeyCode::Enter) || is_mouse_button_down(MouseButton::Left) {
//             if last_updated == "BOB" && is_overlapping(startx, starty, xlen, ylen, bob_x, bob_y, bob_xlen, bob_ylen) {
//                 draw_region(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen, &bob_reg);
//             }
//             draw_region(&mut pixels, startx, starty, xlen, ylen, &reg);
//             let rect = rands();
//             let colour = rect[0]^rect[1]^rect[2]^rect[3];
//             startx = rect[0] as usize % WIDTH;
//             starty = rect[1] as usize % HEIGHT;
//             xlen = rect[2] as usize % (WIDTH - startx);
//             ylen = rect[3] as usize % (HEIGHT - starty);
//             reg = dump_region(&pixels, startx, starty, xlen, ylen);
//             draw_rect(&mut pixels, startx, starty, xlen, ylen, colour);
//             last_updated = "RECT";
//         }

//         // CHANGE BACKGROUND
//         if is_key_released(KeyCode::Backspace) {
//             fill(&mut pixels, WHITE);
//             draw_dotgrid(&mut pixels);
//             draw_rect(&mut pixels, WIDTH*70/100, HEIGHT*50/100, WIDTH*10/100, HEIGHT*30/100, GREY);
//             // background = pixels.clone();
//             reg = dump_region(&pixels, startx, starty, xlen, ylen);
//         }

//         // FPS COUNTER
//         fps += 1;
//         const IS_WASM32: bool = cfg!(target_arch = "wasm32");
//         let elapsed = time.elapsed();
//         let elapsed_secs = if IS_WASM32 {elapsed.as_millis() as usize} else {elapsed.as_secs() as usize}; // bugged library...
//         let elapsed_millis = if IS_WASM32 {elapsed.as_micros() as usize} else {elapsed.as_millis() as usize};
//         if elapsed_secs > 0 {
//             fps %= 1000;
//             draw_region(&mut pixels, fps_startxy, fps_startxy, fps_xlen, fps_ylen, &fps_reg);
//             draw_num(&mut pixels, 5, 5, 5, 10, fps, GREY);
//             fps = 0;
//             time += elapsed;
//         }

//         // BOB
//         if elapsed_millis > 500 {
//             if last_updated == "RECT" && is_overlapping(startx, starty, xlen, ylen, bob_x, bob_y, bob_xlen, bob_ylen) {
//                 draw_region(&mut pixels, startx, starty, xlen, ylen, &reg);
//             }
//             draw_region(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen, &bob_reg);
//             bob_x += 5;
//             bob_x %= WIDTH - bob_xlen;
//             bob_reg = dump_region(&pixels, bob_x, bob_y, bob_xlen, bob_ylen);
//             draw_bob(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen, LIGHT_GREY, BLACK);
//             draw_text(&mut pixels, WIDTH*45/100, 20, 30, 20, &[B,O,B], bob_colour);
//             draw_heart(&mut pixels, WIDTH*50/100 + 10, HEIGHT*55/100, 80, 50, heart_colour);
//             heart_colour = if heart_colour==WHITE {RED} else {WHITE};
//             bob_colour = if bob_colour==BLACK {LIGHT_GREY} else {BLACK};
//             last_updated = "BOB";
//         }


//         window_update_with_buffer(&pixels).await;
//     }
// }