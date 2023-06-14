use std::sync::{Arc, Mutex};

use nih_plug_vizia::vizia::{
    prelude::*,
    vg::{Color, ImageFlags, Paint, Path, PixelFormat},
};

use crate::Sandpile;

pub struct SandpileCanvas {
    sandpile: Arc<Mutex<Sandpile>>,
    pub canvas_xy: Arc<Mutex<(f32, f32)>>,
}

impl SandpileCanvas {
    pub fn new(
        cx: &mut Context,
        sandpile: Arc<Mutex<Sandpile>>,
        xy: Arc<Mutex<(f32, f32)>>,
    ) -> Handle<'_, Self> {
        Self {
            sandpile,
            canvas_xy: xy,
        }
        .build(cx, |_| {})
    }
}

impl View for SandpileCanvas {
    fn element(&self) -> Option<&'static str> {
        Some("sandpile canvas")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        {
            let mut xy = self.canvas_xy.lock();
            xy.as_mut().unwrap().0 = bounds.x;
            xy.as_mut().unwrap().1 = bounds.y;
        }

        let s = self.sandpile.lock().unwrap().clone();

        // grid ignores the first and last row/column
        let grid_size = bounds.w / s.outer_grid_width() as f32;

        let image_id = canvas
            .create_image_empty(
                bounds.w as usize,
                bounds.h as usize,
                PixelFormat::Rgb8,
                ImageFlags::empty(),
            )
            .unwrap();

        // postion of square image on canvas
        //
        //  bounds(x,y) of context
        //      |
        //      ▼
        //      +--------------------+
        //      |                    |
        //      |                    |
        //      |                    |
        //      |                    | heigth
        //      |                    |
        //      |                    |
        //      |                    |
        //      +--------------------+
        //              width
        //
        if let Ok(_size) = canvas.image_size(image_id) {
            // clear background with white
            canvas.clear_rect(
                bounds.x as u32,
                bounds.y as u32,
                bounds.w as u32,
                bounds.h as u32,
                Color::white(),
            );

            // iterate through the tiles
            for x in s.iter_outer_width() {
                for y in s.iter_outer_height() {
                    canvas.clear_rect(
                        (bounds.x as usize + x * grid_size as usize) as u32,
                        (bounds.y as usize + y * grid_size as usize) as u32,
                        grid_size as u32,
                        grid_size as u32,
                        // coloring
                        if x == 0
                            || y == 0
                            || x == s.outer_grid_width() - 1
                            || y == s.outer_grid_height() - 1
                        {
                            Color::white()
                        } else {
                            match s.get_value_at((x, y)) {
                                0 => Color::rgb(200, 210, 209),
                                1 => Color::rgb(104, 144, 77),
                                2 => Color::rgb(20, 71, 30),
                                3 => Color::rgb(238, 155, 1),
                                _ => Color::rgb(218, 106, 0),
                            }
                        },
                    );
                }
            }
        }

        // procedure to display image on canvas
        let mut window_box = Path::new();
        window_box.rect(bounds.x, bounds.y, bounds.w, bounds.h);

        canvas.fill_path(
            &mut window_box,
            &Paint::image(image_id, bounds.x, bounds.y, bounds.w, bounds.h, 0.0, 0.0),
        );

        canvas.restore();
    }
}
