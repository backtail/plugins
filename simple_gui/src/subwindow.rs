use nih_plug_vizia::vizia::{
    prelude::*,
    vg::{Color, ImageFlags, Paint, Path, PixelFormat},
};

pub struct Subwindow {}

impl Subwindow {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self {}.build(cx, |_| {})
    }
}

impl View for Subwindow {
    fn element(&self) -> Option<&'static str> {
        Some("subwindow")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        // Prepare the image, in this case a grid.
        let grid_size: usize = 8;
        let edge_length = 32 * grid_size + 1;
        let image_id = canvas
            .create_image_empty(
                edge_length,
                edge_length,
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
        //      |                    | edge_length
        //      |                    |
        //      |                    |
        //      |                    |
        //      +--------------------+
        //            edge_length
        //
        if let Ok(size) = canvas.image_size(image_id) {
            // clear background with black
            canvas.clear_rect(
                bounds.x as u32,
                bounds.y as u32,
                size.0 as u32,
                size.1 as u32,
                Color::black(),
            );

            // setup grid
            let x_max = (size.0 / grid_size) - 1;
            let y_max = (size.1 / grid_size) - 1;

            // iterate through the tiles
            for x in 0..(size.0 / grid_size) {
                for y in 0..(size.1 / grid_size) {
                    // offset in image with bounds.x and bounds.y
                    canvas.clear_rect(
                        (bounds.x as usize + x * grid_size + 1) as u32,
                        (bounds.y as usize + y * grid_size + 1) as u32,
                        (grid_size - 1) as u32,
                        (grid_size - 1) as u32,
                        // coloring
                        if x == 0 || y == 0 || x == x_max || y == y_max {
                            Color::rgb(40, 80, 40)
                        } else {
                            match (x % 2, y % 2) {
                                (0, 0) => Color::rgb(125, 125, 125),
                                (1, 0) => Color::rgb(155, 155, 155),
                                (0, 1) => Color::rgb(155, 155, 155),
                                (1, 1) => Color::rgb(105, 105, 155),
                                _ => Color::rgb(255, 0, 255),
                            }
                        },
                    );
                }
            }
        }

        // procedure to display image on canvas
        let mut window_box = Path::new();
        window_box.rect(bounds.x, bounds.y, edge_length as f32, edge_length as f32);

        canvas.fill_path(
            &mut window_box,
            &Paint::image(
                image_id,
                bounds.x,
                bounds.y,
                edge_length as f32,
                edge_length as f32,
                0.0,
                0.0,
            ),
        );

        canvas.restore();
    }
}
