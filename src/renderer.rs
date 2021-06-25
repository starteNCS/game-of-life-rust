use crate::gol::GameOfLife;
use sdl2::rect::Rect;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct Renderer {
    pub sdl: Sdl,
    video_subsystem: VideoSubsystem,
    pub canvas: WindowCanvas,
    width: usize,
    height: usize
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Renderer {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem.window("Rust Game of Life", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        return Renderer {
            sdl,
            video_subsystem,
            canvas,
            width,
            height
        }
    }

    pub fn draw_grid(&mut self, gol: &GameOfLife) -> () {
        let row_height = self.height / gol.rows;
        let col_width = self.width / gol.cols;

        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for row in 0..gol.rows {
            for col in 0..gol.cols {
                if gol.field[row][col] {
                    self.canvas.fill_rect(Rect::new(
                        (row * row_height) as i32,
                        (col * col_width) as i32,
                        row_height as u32,
                        col_width as u32)).unwrap();
                }
            }
        }
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.present();
    }
}