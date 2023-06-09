use std::sync::{Arc, Mutex};

use nih_plug_vizia::vizia::{
    prelude::*,
    vg::{Color, ImageFlags, Paint, Path, PixelFormat},
};

use crate::Sandpile;

pub struct Subwindow {
    sandpile: Arc<Mutex<Sandpile>>,
    pub(crate) subwindow_xy: Arc<Mutex<(f32, f32)>>,
    pub(crate) subwindow_wh: Arc<Mutex<(f32, f32)>>,
}

impl Subwindow {
    pub fn new(
        cx: &mut Context,
        sandpile: Arc<Mutex<Sandpile>>,
        xy: Arc<Mutex<(f32, f32)>>,
        wh: Arc<Mutex<(f32, f32)>>,
    ) -> Handle<'_, Self> {
        Self {
            sandpile,
            subwindow_xy: xy,
            subwindow_wh: wh,
        }
        .build(cx, |_| {})
    }
}

impl View for Subwindow {
    fn element(&self) -> Option<&'static str> {
        Some("subwindow")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        {
            let mut xy = self.subwindow_xy.lock();
            xy.as_mut().unwrap().0 = bounds.x;
            xy.as_mut().unwrap().1 = bounds.y;
        }

        {
            let mut wh = self.subwindow_wh.lock();
            wh.as_mut().unwrap().0 = bounds.w;
            wh.as_mut().unwrap().1 = bounds.h;
        }

        let s = self.sandpile.lock().unwrap().clone();

        // Prepare the image, in this case a sandpile.
        let grid_size: usize = 12;
        let len_x = s.len_x() * grid_size + 1;
        let len_y = s.len_y() * grid_size + 1;
        let image_id = canvas
            .create_image_empty(len_x, len_y, PixelFormat::Rgb8, ImageFlags::empty())
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
        //      |                    | len_y
        //      |                    |
        //      |                    |
        //      |                    |
        //      +--------------------+
        //              len_x
        //
        if let Ok(_size) = canvas.image_size(image_id) {
            // clear background with black
            canvas.clear_rect(
                bounds.x as u32,
                bounds.y as u32,
                len_x as u32,
                len_y as u32,
                Color::black(),
            );

            // iterate through the tiles
            for x in 0..s.len_x() {
                for y in 0..s.len_y() {
                    // offset in image with bounds.x and bounds.y
                    canvas.clear_rect(
                        (bounds.x as usize + x * grid_size + 1) as u32,
                        (bounds.y as usize + y * grid_size + 1) as u32,
                        (grid_size - 1) as u32,
                        (grid_size - 1) as u32,
                        // coloring
                        match s.get_value_at((x, y)) {
                            0 => Color::rgb(200, 210, 209),
                            1 => Color::rgb(104, 144, 77),
                            2 => Color::rgb(20, 71, 30),
                            // 3 => Color::rgb(238, 155, 1),
                            _ => Color::rgb(218, 106, 0),
                        },
                    );
                }
            }
        }

        // procedure to display image on canvas
        let mut window_box = Path::new();
        window_box.rect(bounds.x, bounds.y, len_x as f32, len_y as f32);

        canvas.fill_path(
            &mut window_box,
            &Paint::image(
                image_id,
                bounds.x,
                bounds.y,
                len_x as f32,
                len_y as f32,
                0.0,
                0.0,
            ),
        );

        canvas.restore();
    }
}
