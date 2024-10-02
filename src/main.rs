use macroquad::texture::{draw_texture, Texture2D};
use macroquad::input::{is_key_down, is_key_released, is_mouse_button_down};
use macroquad::miniquad::{KeyCode, MouseButton};
use macroquad::prelude::Conf;

const NAME: &str = "PoopyPoop";
const WIDTH: usize = 600;
const HEIGHT: usize = 400;
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


const WHITE: u32 = 0xff_ff_ff;
const BLACK: u32 = 0x00_00_00;
const GREY: u32 = 0x88_88_88;
const LIGHT_GREY: u32 = 0xcc_cc_cc;

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

// #[macroquad::main("HELLO")]
#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, world!");
    let mut pixels = vec![WHITE; WIDTH*HEIGHT];
    draw_dotgrid(&mut pixels);
    fill(&mut pixels, WHITE);
    draw_grid(&mut pixels);
    draw_rect(&mut pixels, 10, 30, 50, 5, BLACK);
    draw_rect(&mut pixels, 400, 50, 50, 100, GREY);
    draw_rect(&mut pixels, WIDTH/2, HEIGHT/2, 100, 80, LIGHT_GREY);
    draw_pixel(&mut pixels, 200, 200, BLACK);
    let (mut reg, mut startx, mut starty, mut xlen, mut ylen) = (vec![], 0,0,0,0);
    loop {
        if is_key_down(KeyCode::Escape) {
            macroquad::window::miniquad::window::quit();
            break;
        } 

        if is_key_down(KeyCode::Space) || is_key_released(KeyCode::Enter) || is_mouse_button_down(MouseButton::Left) {
            draw_region(&mut pixels, startx, starty, xlen, ylen, &reg);
            let rect = rands();
            let colour = rect[0]^rect[1]^rect[2]^rect[3];
            startx = rect[0] as usize % WIDTH;
            starty = rect[1] as usize % HEIGHT;
            xlen = rect[2] as usize % (WIDTH - startx);
            ylen = rect[3] as usize % (HEIGHT - starty);
            reg = dump_region(&pixels, startx, starty, xlen, ylen);
            draw_rect(&mut pixels, startx, starty, xlen, ylen, colour);
        }

        if is_key_released(KeyCode::Backspace) {
            fill(&mut pixels, WHITE);
            draw_dotgrid(&mut pixels);
            draw_rect(&mut pixels, WIDTH*70/100, HEIGHT*50/100, WIDTH*10/100, HEIGHT*30/100, GREY);
            reg = dump_region(&pixels, startx, starty, xlen, ylen);
        }
        
        window_update_with_buffer(&pixels).await;
    }
}