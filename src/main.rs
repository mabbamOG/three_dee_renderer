use minifb::{Window, WindowOptions, Key};

fn main() {
    println!("Hello, world!");
    const NAME: &str = "PoopyPoop";
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;
    let mut window = Window::new(NAME, WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut pixels = [0xff_ff_ff; WIDTH*HEIGHT];
    for _ in  0..5 {
        window.update_with_buffer(&pixels, WIDTH, HEIGHT).unwrap();
    }

    let a1 = (HEIGHT/10, WIDTH/10);
    let a2 = (HEIGHT*7/10, WIDTH/10);
    let a3 = (HEIGHT/10, WIDTH*7/10);
    let a4 = (HEIGHT*7/10, WIDTH*7/10);
    let rectangles = [a1, a2, a3, a4];
    let mut count = 0;
    let time = std::time::Instant::now();

    loop {
        if window.is_key_down(Key::Space) || window.is_key_released(Key::Enter) {
            let draw = rectangles[count%4];
            let clear = rectangles[(count+3) % 4];
            let colour = time.elapsed().as_nanos() as u32;
            for x in draw.0..draw.0+100 {
                for y in draw.1..draw.1+100 {
                    pixels[x*HEIGHT + y] = colour;
                }
            }
            for x in clear.0..clear.0+100 {
                for y in clear.1..clear.1+100 {
                    pixels[x*HEIGHT + y] = 0xff_ff_ff;
                }
            }
            window.update_with_buffer(&pixels, WIDTH, HEIGHT).unwrap();
            count += 1;
        }
        if window.is_key_down(Key::Escape) {
            break;
        }
        window.update();
    }
}