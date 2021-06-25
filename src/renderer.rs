use crate::gol::GameOfLife;
use sdl2::rect::Rect;
use sdl2::VideoSubsystem;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct Renderer {
    pub canvas: WindowCanvas,
    width: usize,
    height: usize
}

impl Renderer {
    pub fn new(video_subsystems: &VideoSubsystem, width: usize, height: usize) -> Renderer {
        let window = video_subsystems.window("Rust Game of Life", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        return Renderer {
            canvas: window.into_canvas().build().unwrap(),
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