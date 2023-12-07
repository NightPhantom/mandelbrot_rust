use sdl2::pixels::Color;
use std::time::Instant;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    // SDL boilerplate
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem= sdl_context.video().unwrap();
    let window = video_subsystem.window("Fractal Box", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().build().unwrap();
    let mut sdl_canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        let start_time = Instant::now();

        // TODO: Process events

        sdl_canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl_canvas.clear();

        // TODO: Calculate mandelbrot
        std::thread::sleep(std::time::Duration::from_millis(1000));

        // Display render time on title
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        let title = format!("{}{}", "Fractal Box - Render time: ".to_string(), elapsed_time.as_secs_f64().to_string());
        let _ = sdl_canvas.window_mut().set_title(&title);
    }
}
