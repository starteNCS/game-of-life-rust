extern crate sdl2;

mod gol;
mod renderer;
mod debug;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::gol::GameOfLife;
use stopwatch::Stopwatch;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();


    let mut game_of_life: GameOfLife;
    match gol::GameOfLife::new(40, 40) {
        Err(_x) => unimplemented!(),
        Ok(x) => game_of_life = x
    }
    let mut renderer = renderer::Renderer::new(&video_subsystem, WIDTH, HEIGHT);
    let mut debug = debug::Debug::open_debug_window(&video_subsystem);
    debug.display_stats();

    let mut is_simulation_running = false;
    let mut event_pump = sdl.event_pump().unwrap();
    let mut stopwatch = Stopwatch::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    is_simulation_running = !is_simulation_running;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    if !is_simulation_running {
                        game_of_life.draw(x as usize, y as usize, WIDTH, HEIGHT);
                    }
                }
                _ => {}
            }
        }

        if is_simulation_running {
            stopwatch.restart();
            game_of_life.run();
            stopwatch.stop();
            debug.stats.latest_calculation_time = stopwatch.elapsed_ms();
        }


        stopwatch.restart();
        renderer.draw_grid(&game_of_life);
        stopwatch.stop();
        debug.stats.latest_draw_time = stopwatch.elapsed_ms();

        debug.display_stats();
        // ::std::thread::sleep(Duration::new(0, 100000000));
    }
}