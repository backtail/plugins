use std::sync::Arc;

use nih_plug_vizia::vizia::{
    prelude::*,
    vg::{Color, Paint, Path},
};

pub const WAVEFORM_SCALE_FACTOR: f32 = 0.8;

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
        draw_waveform(cx, canvas, &self.audio_buffer[..]);
    }
}

fn draw_waveform(cx: &mut DrawContext, canvas: &mut Canvas, sample_buffer: &[f32]) {
    let bounds = cx.bounds();

    let mut clear_canvas = Path::new();
    clear_canvas.rect(bounds.x, bounds.y, bounds.w, bounds.h);
    canvas.fill_path(&mut clear_canvas, &Paint::color(Color::white()));

    let buffer_length = sample_buffer.len() as f32;
    let waveform_iter = sample_buffer
        .iter()
        .enumerate()
        .step_by(50)
        .map(|(i, sample)| {
            let x = (i as f32 / buffer_length) * bounds.w;
            let y = (sample * WAVEFORM_SCALE_FACTOR) * (bounds.h / 2.0) + (bounds.h / 2.0);

            let color_strength1 = i as f32 / buffer_length;
            let color_strength2 = 1.0 - i as f32 / buffer_length;

            (
                x,
                y,
                Color::rgbf(
                    0.0,
                    color_strength2 * 0.5 + 0.5,
                    color_strength1 * 0.5 + 0.5,
                ),
            )
        });

    let mut last_xy = (bounds.x, bounds.y + bounds.h / 2.0);
    for (x, y, color) in waveform_iter {
        let mut line = Path::new();
        line.move_to(last_xy.0, last_xy.1);
        line.line_to(bounds.x + x, bounds.y + y);
        last_xy = (bounds.x + x, bounds.y + y);
        canvas.stroke_path(&mut line, &Paint::color(color).with_line_width(1.0));
    }
}
