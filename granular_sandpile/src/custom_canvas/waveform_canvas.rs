use std::sync::Arc;

use nih_plug_vizia::vizia::{
    prelude::*,
    vg::{Color, Paint, Path},
};

pub struct WaveformCanvas {
    audio_buffer: Arc<Vec<f32>>,
}

impl WaveformCanvas {
    pub fn new(cx: &mut Context, audio_buffer: Arc<Vec<f32>>) -> Handle<'_, Self> {
        Self { audio_buffer }.build(cx, |_| {})
    }
}

impl View for WaveformCanvas {
    fn element(&self) -> Option<&'static str> {
        Some("waveform canvas")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        let mut clear_canvas = Path::new();
        clear_canvas.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.fill_path(&mut clear_canvas, &Paint::color(Color::white()));

        let mut line = Path::new();
        line.move_to(bounds.x, bounds.y);
        line.line_to(bounds.x + bounds.w, bounds.y + bounds.h);

        canvas.stroke_path(
            &mut line,
            &Paint::color(Color::black()).with_line_width(1.0),
        );

        canvas.restore();
    }
}
