

extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
//use std::time::Duration;
//use rand::Rng;

pub mod spiral;
use crate::spiral::Spiral;

pub mod mandelbrot;
mod complex;

use crate::mandelbrot::is_mandelbrot;

const SCALE: f64 = 0.008;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot set", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    //let i = 0;
    //let mut rng = rand::thread_rng();
    //let mut points = vec!(Point::new(0, 0));
    let mut s = Spiral::new();
    let in_color =  Color::RGB(240, 40, 40);
    let out_color =  Color::RGB(20, 240, 240);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { .. } => break 'running,
                _ => {}
            }
        }
        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();
        //i = (i + 1) % 255;
        let (w, h) = canvas.output_size().unwrap();
        //let mut points = [Point::new(0, 0); 256];
        //points.fill_with(|| Point::new(rng.gen_range(0..w as i32), rng.gen_range(0..h as i32)));
        // For performance, it's probably better to draw a whole bunch of points at once
        //canvas.draw_points(points.as_slice()).unwrap();
        let (x, y) = s.next().unwrap();
        let mut color = out_color.clone();
        if is_mandelbrot(x as f64 * SCALE, y as f64 * SCALE) {
            color = in_color.clone()
        }

        canvas.set_draw_color(color);
        let points = [Point::new(x  + (w / 2) as i32, y + (h / 2) as i32)];
        canvas.draw_points(points.as_slice()).unwrap();

        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120)); // sloppy FPS limit
    }
}
