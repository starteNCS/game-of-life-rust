extern crate sdl2;

mod gol;
mod renderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::gol::GameOfLife;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

pub fn main() {
    let mut game_of_life: GameOfLife;
    match gol::GameOfLife::new(40, 40) {
        Err(_x) => unimplemented!(),
        Ok(x) => game_of_life = x
    }
    let mut renderer = renderer::Renderer::new(WIDTH, HEIGHT);

    let mut is_running = false;
    let mut event_pump = renderer.sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    is_running = !is_running;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    if !is_running {
                        game_of_life.draw(x as usize, y as usize, WIDTH, HEIGHT);
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        if is_running {
            game_of_life.run();
        }


        renderer.draw_grid(&game_of_life);
        ::std::thread::sleep(Duration::new(0, 100000000));
    }
}