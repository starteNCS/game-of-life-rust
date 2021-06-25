use sdl2::VideoSubsystem;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::path::Path;
use sdl2::video::WindowContext;
use sdl2::ttf::Sdl2TtfContext;

pub struct DebugStats {
    pub rows: usize,
    pub cols: usize,
    pub latest_calculation_time: i64,
    pub latest_draw_time: i64,
    pub current_generation: i64,
    pub elapsed_seconds: i64
}

pub struct Debug {
    pub stats: DebugStats,
    canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
}

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 250;

impl Debug {
    pub fn open_debug_window(video_systems: &VideoSubsystem, cols: usize, rows: usize) -> Debug {
        let window = video_systems.window("Game of Live - Debug", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position(50, 50)
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas
            .texture_creator();


        canvas.set_draw_color(Color::BLACK);
        canvas.present();

        Debug {
            stats: DebugStats {
                cols,
                rows,
                latest_calculation_time: 0,
                latest_draw_time: 0,
                current_generation: 0,
                elapsed_seconds: 0
            },
            canvas,
            ttf_context,
            texture_creator,
        }
    }

    pub fn display_stats(&mut self) -> () {
        self.canvas.clear();

        self.draw_text(format!("Fieldsize: {}x{}", self.stats.rows, self.stats.cols).as_str(), 0);
        self.draw_text(format!("Generation took: {}ms", self.stats.latest_calculation_time).as_str(), 1);
        self.draw_text(format!("Drawing took: {}ms", self.stats.latest_draw_time).as_str(), 2);
        self.draw_text(format!("Generation: {}", self.stats.current_generation).as_str(), 3);
        self.draw_text(format!("Elapsed Seconds: {}", self.stats.elapsed_seconds).as_str(), 4);
        self.draw_text(format!("Theoretical Gen/Sec: {}",
                               if self.stats.latest_calculation_time > 0 {1000 / self.stats.latest_calculation_time} else {-1}).as_str(), 5);
        self.draw_text(format!("Real Gen/Sec: {:.2}",
                               if self.stats.elapsed_seconds > 0 {self.stats.current_generation as f64 / (self.stats.elapsed_seconds as f64 / 1000f64)} else {-1.0}).as_str(), 6);

        self.canvas.present();
    }

    fn draw_text(&mut self, text: &str, y: i32) -> () {
        let font = self.ttf_context
            .load_font(Path::new("./assets/roboto.ttf"), 32)
            .unwrap();

        let text_surface = font
            .render(text)
            .blended_wrapped(Color::WHITE, WINDOW_WIDTH)
            .unwrap();
        let texture = self.texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        self.canvas.copy(
            &texture,
            None,
            Some(Rect::new(10, 5 + (y * 30), (text.len() * 16) as u32, 25)))
            .unwrap();
    }
}