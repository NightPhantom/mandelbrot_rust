use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::Canvas, video::Window, rect::Point};
use std::time::Instant;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Settings {
    max_iterations: u32,
    escape_target: f64
}

struct Pixel {
    x: u32,
    y: u32
}
struct Complex {
    real: f64,
    imaginary: f64
}

struct ComplexSpace {
    real_minimun: f64,
    real_maximun: f64,
    imaginary_minimun: f64,
    imaginary_maximun: f64,
    zoom_factor: f64
}

impl ComplexSpace {
    fn zoomed_real_minimum(&self) -> f64 { self.zoom_factor * self.real_minimun }
    fn zoomed_real_maximun(&self) -> f64 { self.zoom_factor * self.real_maximun }
    fn zoomed_imaginary_minimun(&self) -> f64 { self.zoom_factor * self.imaginary_minimun }
    fn zoomed_imaginary_maximun(&self) -> f64 { self.zoom_factor * self.imaginary_maximun }
}

fn main() {
    // SDL boilerplate
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem= sdl_context.video().unwrap();
    let window = video_subsystem.window("Fractal Box", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().build().unwrap();
    let mut sdl_canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Define coordinates
    let mut complex_space: ComplexSpace = ComplexSpace {
        real_minimun: -2.0,
        real_maximun: 1.0,
        imaginary_minimun: 1.2,
        imaginary_maximun: -(((3.0) * (f64::from(SCREEN_HEIGHT) / f64::from(SCREEN_WIDTH))) - 1.2),
        zoom_factor: 1.0
    };

    'gameloop: loop {
        let start_time = Instant::now();

        // Process keyboard events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Q), .. } => break 'gameloop, // Q - Quit
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'gameloop, // Esc - Quit
                Event::KeyDown { keycode: Some(Keycode::R), .. } => { // R - Reset coordinate space to default
                    complex_space.zoom_factor = 1.0;
                    complex_space.real_minimun = -2.0;
                    complex_space.real_maximun = 1.0;
                    complex_space.imaginary_minimun = 1.2;
                    complex_space.imaginary_maximun = -(((1.0 + 2.0) * (600.0 / 800.0)) - 1.2);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => { // ◄ - Scroll to the left
                    complex_space.real_minimun = complex_space.real_minimun - 0.5;
                    complex_space.real_maximun = complex_space.real_maximun - 0.5;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { // ► - Scroll to the right
                    complex_space.real_minimun = complex_space.real_minimun + 0.5;
                    complex_space.real_maximun = complex_space.real_maximun + 0.5;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => { // ▲ - Scroll up
                    complex_space.imaginary_minimun = complex_space.imaginary_minimun + 0.5;
                    complex_space.imaginary_maximun = complex_space.imaginary_maximun + 0.5;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => { // ▼ - Scroll down
                    complex_space.imaginary_minimun = complex_space.imaginary_minimun - 0.5;
                    complex_space.imaginary_maximun = complex_space.imaginary_maximun - 0.5;
                },
                Event::KeyDown { keycode: Some(Keycode::PageDown), .. } => { // PgUp - Zoom out
                    complex_space.zoom_factor = complex_space.zoom_factor * 0.5;
                },
                Event::KeyDown { keycode: Some(Keycode::PageUp), .. } => { // PgDn - Zoom in
                    complex_space.zoom_factor = complex_space.zoom_factor * 1.5;
                },
                _ => {}
            }
        }

        sdl_canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl_canvas.clear();

        // Calculate and show the mandelbrot set
        present_mandelbrot(&mut sdl_canvas, &complex_space, Settings { max_iterations: 2000, escape_target: 4.0 });
        
        // Display render time on title
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        let title = format!("{}{}", "Fractal Box - Render time: ".to_string(), elapsed_time.as_secs_f64().to_string());
        let _ = sdl_canvas.window_mut().set_title(&title);
    }
}

fn present_mandelbrot(canvas: &mut Canvas<Window>, complex_space: &ComplexSpace, settings: Settings) {
    let screen_width = canvas.window().size().0;
    let screen_height = canvas.window().size().1;
    
    for y in 0..screen_height {
        for x in 0.. screen_width {
            // Convert pixel coordinates to complex number
            let c: Complex = convert_pixel_to_complex(
                Pixel { x: x, y: y },
                screen_width,
                screen_height,
                complex_space
            );

            // Compute the number of iterations
            let iterations: u32 = is_in_mandelbrot_set(&c, settings.escape_target, settings.max_iterations);

            // Plot the point
            if iterations == settings.max_iterations {
                // No escape, paint it black
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            else {
                // Escaped, color it something based on number of iterations
                let bright: f64 = map_color(f64::from(iterations), 0.0, f64::from(settings.max_iterations), 0.0, 255.0);
                let red: f64 = map_color(bright * bright, 0.0, 6502.0, 0.0, 255.0);
                let green: f64 = bright;
                let blue: f64 = map_color(bright.sqrt(), 0.0, 255f64.sqrt(), 0.0, 255.0);
                
                canvas.set_draw_color(Color::RGB(red as u8, green as u8, blue as u8));
            }
            let _ = canvas.draw_point(Point::new(x as i32, y as i32));
        }
    }

    canvas.present();
}

// Return max_iterations if the argument is in the mandelbrot set,
// otherwise return the number of iterations to reach escape_target
fn is_in_mandelbrot_set(c: &Complex, escape_target: f64, max_iterations: u32) -> u32 {
    let mut iterations = 0;
    let mut z: Complex = Complex { real: 0.0, imaginary: 0.0 };

    while iterations < max_iterations && f64::abs((z.real * z.real) + (z.imaginary * z.imaginary)) <= escape_target {
        let temp_real = (z.real * z.real) - (z.imaginary * z.imaginary) + c.real;
        z.imaginary = 2.0 * z.real * z.imaginary + c.imaginary;
        z.real = temp_real;
        iterations = iterations + 1;
    }

    return iterations;
}

fn convert_pixel_to_complex(pixel: Pixel, pix_width:u32, pix_height:u32, complex_space: &ComplexSpace) -> Complex {
    let c: Complex = Complex {
        real: convert_component_to_complex(
            pixel.x as i32,
            pix_width,
            complex_space.zoomed_real_minimum(),
            complex_space.zoomed_real_maximun()
        ),
        imaginary: convert_component_to_complex(
            pixel.y as i32,
            pix_height,
            complex_space.zoomed_imaginary_minimun(),
            complex_space.zoomed_imaginary_maximun()
        )
    };

    return c;
}

fn convert_component_to_complex(pixel: i32, pix_max: u32, complex_min: f64, complex_max: f64) -> f64 {
    let pix_ratio = f64::from(pixel) / f64::from(pix_max);
    return complex_min + (complex_max - complex_min) * pix_ratio;
}

fn map_color(val: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    return (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}
