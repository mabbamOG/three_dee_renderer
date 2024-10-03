use macroquad::texture::{draw_texture, Texture2D};
use macroquad::input::{is_key_down, is_key_released, is_mouse_button_down};
use macroquad::miniquad::{KeyCode, MouseButton};
use macroquad::prelude::Conf;

const NAME: &str = "PoopyPoop";
const WIDTH: usize = 600;
const HEIGHT: usize = 400;
const WHITE: u32 = 0xff_ff_ff;
const BLACK: u32 = 0x00_00_00;
const GREY: u32 = 0x88_88_88;
const RED: u32 = 0xff_00_00;
const LIGHT_GREY: u32 = 0xcc_cc_cc;
const CHARWIDTH: usize = 3;
const A: &[bool] = &trans([
    true, true, true,
    true, true, true,
    true, false, true
]);
const B: &[bool] = &trans([
    true, false, false,
    true, true, true,
    true, true, true
]);
const C: &[bool] = &trans([
    true, true, true,
    true, false, false,
    true, true, true
]);
const E: &[bool] = &trans([
    true, true, true,
    true, true, false,
    true, true, true
]);
const H: &[bool] = &trans([
    true, false, true,
    true, true, true,
    true, false, true
]);
const L: &[bool] = &trans([
    true, false, false,
    true, false, false,
    true, true, true
]);
const O: &[bool] = &trans([
    true, true, true,
    true, false, true,
    true, true, true
]);
const S: &[bool] = &trans([
    false, true, true,
    false, true, false,
    true, true, false
]);
const X: &[bool] = &trans([
    true, false, true,
    false, true, false,
    true, false, true
]);
const Y: &[bool] = &trans([
    true, false, true,
    false, true, false,
    false, true, false
]);
const SPACE: &[bool] = &trans([
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

fn fill(pixels: &mut[u32], colour: u32) {
    for i in 0..WIDTH*HEIGHT {
        pixels[i] = colour;
    }
}

fn draw_grid(pixels: &mut[u32]) {
    for x in (0..WIDTH).step_by(10) {
        for y in 0..HEIGHT {
            pixels[index(x,y)] = BLACK;
        }
    }
    for x in 0..WIDTH {
        for y in (0..HEIGHT).step_by(10) {
            pixels[index(x,y)] = BLACK;
        }
    }
}

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

fn draw_pixel(pixels: &mut[u32], x: usize, y: usize, colour: u32) {
    pixels[index(x,y)] = colour;
}

fn draw_region(pixels: &mut[u32], startx: usize, starty: usize, xlen: usize, ylen: usize, reg: &[u32]) {
    let mut i = 0;
    for x in startx..startx+xlen {
        for y in starty..starty+ylen {
            pixels[index(x,y)] = reg[i];
            i += 1;
        }
    }
}

// fn draw_region_part(pixels: &mut[u32], startx: usize, starty: usize, xlen: usize, ylen: usize, reg: &[u32], reg_width: usize) {
//     for x in startx..startx+xlen {
//         for y in starty..starty+ylen {
//             pixels[index(x,y)] = reg[y*reg_width + x];
//         }
//     }
// }

fn dump_region(pixels: &[u32], startx: usize, starty: usize, xlen: usize, ylen: usize) -> Vec<u32> {
    let mut out = vec![0; xlen*ylen];
    let mut i = 0;
    for x in startx..startx+xlen {
        for y in starty..starty+ylen {
            out[i] = pixels[index(x,y)];
            i += 1;
        }
    }
    out
}

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
    draw_texture(&texture, 0.0, 0.0, macroquad::color::WHITE); // IMPORTANT: KEEP THIS TO WHITE!!!
    macroquad::window::next_frame().await
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

fn is_overlapping(x1: usize, y1: usize, w1: usize, h1:usize, x2: usize, y2: usize, w2: usize, h2: usize) -> bool {
    // let (left1, right1) = (x1, x1 + w1);
    // let (left2, right2) = (x2, x2+w2);
    // let (up1, down1) = (y1, y1+h1);
    // let (up2, down2) = (y2, y2+h2);
    // let overlappingx = (left2 >= left1 && left2 <= right1) || (right2 >= left1 && right2 <= right1);
    // let overlappingy = (up2 >= up1 && up2 <= down1) || (down2 >= up1 && down2 <= down1);
    // overlappingx && overlappingy

    // Check if one rectangle is to the left of the other
    if x1 + w1 < x2 {
        return false;
    }
    // Check if one rectangle is to the right of the other
    if x2 + w2 < x1 {
        return false;
    }
    // Check if one rectangle is above the other
    if y1 + h1 < y2 {
        return false;
    }
    // Check if one rectangle is below the other
    if y2 + h2 < y1 {
        return false;
    }
    
    // If none of the above are true, the rectangles must overlap
    true
}

// fn get_overlapping(x1: usize, y1: usize, w1: usize, h1:usize, x2: usize, y2: usize, w2: usize, h2: usize) -> (usize, usize, usize, usize) {
//     let left = x1.max(x2);
//     let top = y1.max(y2);
//     let right = x1.min(x2 + w2);
//     let bottom = y1.min(y2 + h2);

//     let width = right - left;
//     let height = bottom - top;
//     (left, top, width, height)
// }

#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, world!");
    let mut pixels = vec![WHITE; WIDTH*HEIGHT];
    draw_dotgrid(&mut pixels);
    fill(&mut pixels, WHITE);
    draw_grid(&mut pixels);
    draw_rect(&mut pixels, 10, 30, 50, 5, BLACK);
    draw_rect(&mut pixels, 400, 50, 50, 100, GREY);
    draw_rect(&mut pixels, WIDTH*50/100, HEIGHT*50/100, 100, 80, LIGHT_GREY);
    draw_pixel(&mut pixels, 200, 200, BLACK);
    draw_char(&mut pixels, 30, 30, 10, C, BLACK);
    draw_char(&mut pixels, 70, 30, 10, A, BLACK);
    draw_char(&mut pixels, 110, 30, 10, O, BLACK);
    draw_char(&mut pixels, 150, 30, 10, S, BLACK);
    draw_text(&mut pixels, 30, 80, 15, 15, &[C,A,O,S], BLACK);
    draw_text(&mut pixels, 50, 150, 20, 15, &[S,E,X,Y], BLACK);
    draw_num(&mut pixels, WIDTH*70/100, HEIGHT*70/100, 10, 10, 302, BLACK);
    draw_text(&mut pixels, WIDTH*5/100, HEIGHT*60/100, 10, 10, &[H,E,L,L,O,SPACE], GREY);
    draw_heart(&mut pixels, WIDTH*50/100 + 10, HEIGHT*55/100, 80, 50, WHITE);

    let (mut reg, mut startx, mut starty, mut xlen, mut ylen) = (vec![], 0,0,0,0);
    let (mut fps, fps_startxy, fps_thickness) = (0, 5, 10);
    let (fps_xlen, fps_ylen) = (fps_thickness*CHARWIDTH*3, fps_thickness*CHARWIDTH);
    let fps_reg = dump_region(&pixels, fps_startxy, fps_startxy, fps_xlen, fps_ylen);
    let mut time = instant::Instant::now();
    let bob_xlen = 40;
    let bob_ylen = 4*bob_xlen;
    let (mut bob_x, bob_y) = (0, HEIGHT - bob_ylen);
    let mut bob_reg = dump_region(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen);
    let mut bob_colour = BLACK;
    let mut heart_colour = WHITE;
    let mut last_updated = "BOB";
    loop {
        // QUIT
        if is_key_down(KeyCode::Escape) {
            macroquad::window::miniquad::window::quit();
            break;
        } 

        // RANDOM SQUARE
        if is_key_down(KeyCode::Space) || is_key_released(KeyCode::Enter) || is_mouse_button_down(MouseButton::Left) {
            if last_updated == "BOB" && is_overlapping(startx, starty, xlen, ylen, bob_x, bob_y, bob_xlen, bob_ylen) {
                draw_region(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen, &bob_reg);
            }
            draw_region(&mut pixels, startx, starty, xlen, ylen, &reg);
            let rect = rands();
            let colour = rect[0]^rect[1]^rect[2]^rect[3];
            startx = rect[0] as usize % WIDTH;
            starty = rect[1] as usize % HEIGHT;
            xlen = rect[2] as usize % (WIDTH - startx);
            ylen = rect[3] as usize % (HEIGHT - starty);
            reg = dump_region(&pixels, startx, starty, xlen, ylen);
            draw_rect(&mut pixels, startx, starty, xlen, ylen, colour);
            last_updated = "RECT";
        }

        // CHANGE BACKGROUND
        if is_key_released(KeyCode::Backspace) {
            fill(&mut pixels, WHITE);
            draw_dotgrid(&mut pixels);
            draw_rect(&mut pixels, WIDTH*70/100, HEIGHT*50/100, WIDTH*10/100, HEIGHT*30/100, GREY);
            // background = pixels.clone();
            reg = dump_region(&pixels, startx, starty, xlen, ylen);
        }

        // FPS COUNTER
        fps += 1;
        const IS_WASM32: bool = cfg!(target_arch = "wasm32");
        let elapsed = time.elapsed();
        let elapsed_secs = if IS_WASM32 {elapsed.as_millis() as usize} else {elapsed.as_secs() as usize}; // bugged library...
        let elapsed_millis = if IS_WASM32 {elapsed.as_micros() as usize} else {elapsed.as_millis() as usize};
        if elapsed_secs > 0 {
            fps %= 1000;
            draw_region(&mut pixels, fps_startxy, fps_startxy, fps_xlen, fps_ylen, &fps_reg);
            draw_num(&mut pixels, 5, 5, 5, 10, fps, GREY);
            fps = 0;
            time += elapsed;
        }

        // BOB
        if elapsed_millis > 500 {
            if last_updated == "RECT" && is_overlapping(startx, starty, xlen, ylen, bob_x, bob_y, bob_xlen, bob_ylen) {
                draw_region(&mut pixels, startx, starty, xlen, ylen, &reg);
            }
            draw_region(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen, &bob_reg);
            bob_x += 5;
            bob_x %= WIDTH - bob_xlen;
            bob_reg = dump_region(&pixels, bob_x, bob_y, bob_xlen, bob_ylen);
            draw_bob(&mut pixels, bob_x, bob_y, bob_xlen, bob_ylen, LIGHT_GREY, BLACK);
            draw_text(&mut pixels, WIDTH*45/100, 20, 30, 20, &[B,O,B], bob_colour);
            draw_heart(&mut pixels, WIDTH*50/100 + 10, HEIGHT*55/100, 80, 50, heart_colour);
            heart_colour = if heart_colour==WHITE {RED} else {WHITE};
            bob_colour = if bob_colour==BLACK {LIGHT_GREY} else {BLACK};
            last_updated = "BOB";
        }


        window_update_with_buffer(&pixels).await;
    }
}