use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::{
    assets, create_vizia_editor, vizia::prelude::*, widgets::*, ViziaState, ViziaTheming,
};

use crate::{sandpile_canvas::SandpileCanvas, waveform_canvas::WaveformCanvas, EditorData};

pub const GUI_WIDTH: u32 = 400;
pub const GUI_HEIGHT: u32 = 400;
pub const SANDPILE_CANVAS_SIDE_LENGTH: f32 = 200.0;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (GUI_WIDTH, GUI_HEIGHT))
}

pub enum SandpileEvent {
    Reset,
    Add,
    Remove,
    UpdateMousePosition,
}

impl Model for EditorData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|sandpile_event, _| match sandpile_event {
            SandpileEvent::Reset => {
                let mut s = self.sandpile.lock().unwrap();
                s.reset();
                let samples = &self.audio_buffer.as_ref()[0..50];

                println!("{:?}", samples);
            }
            SandpileEvent::Add => {
                let mut s = self.sandpile.lock().unwrap();
                s.add_at(
                    self.params.sandpile_editor_state.user_pile_amount.value() as usize,
                    (self.mouse_xy.0 as usize, self.mouse_xy.1 as usize),
                );
            }
            SandpileEvent::Remove => {
                let mut s = self.sandpile.lock().unwrap();
                s.remove_at(
                    self.params.sandpile_editor_state.user_pile_amount.value() as usize,
                    (self.mouse_xy.0 as usize, self.mouse_xy.1 as usize),
                );
            }
            SandpileEvent::UpdateMousePosition => {
                let xy = self.canvas_xy.lock();
                let s = self.sandpile.lock().unwrap();

                // relative pixels
                self.mouse_xy = (
                    cx.mouse.cursorx - xy.as_ref().unwrap().0,
                    cx.mouse.cursory - xy.as_ref().unwrap().1,
                );

                // apply scale factor
                self.mouse_xy.0 /= cx.user_scale_factor() as f32;
                self.mouse_xy.1 /= cx.user_scale_factor() as f32;

                // normalize
                self.mouse_xy.0 = self.mouse_xy.0 / SANDPILE_CANVAS_SIDE_LENGTH;
                self.mouse_xy.1 = self.mouse_xy.1 / SANDPILE_CANVAS_SIDE_LENGTH;

                // scale to grid size
                self.mouse_xy.0 *= s.outer_grid_width() as f32;
                self.mouse_xy.1 *= s.outer_grid_height() as f32;

                // saturate in case of mistake
                self.mouse_xy.0 = self
                    .mouse_xy
                    .0
                    .clamp(0.0, s.outer_grid_width() as f32 - 1.0);
                self.mouse_xy.1 = self
                    .mouse_xy
                    .1
                    .clamp(0.0, s.outer_grid_height() as f32 - 1.0);
            }
        });
    }
}

pub(crate) fn create(
    editor_state: Arc<ViziaState>,
    editor_data: EditorData,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        editor_data.clone().build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Granular Sandpile")
                .font_family(vec![FamilyOwned::Name(String::from(
                    assets::NOTO_SANS_THIN,
                ))])
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Sandpile Cellular Automata").top(Units::Pixels(10.0));
            SandpileCanvas::new(
                cx,
                editor_data.sandpile.clone(),
                editor_data.canvas_xy.clone(),
            )
            .top(Units::Pixels(4.0))
            .size(Units::Pixels(SANDPILE_CANVAS_SIDE_LENGTH))
            .on_mouse_down(|a, button| {
                a.emit(SandpileEvent::UpdateMousePosition);
                match button {
                    MouseButton::Left => {
                        a.emit(SandpileEvent::Add);
                    }
                    MouseButton::Right => {
                        a.emit(SandpileEvent::Remove);
                    }
                    _ => {}
                }
            });

            Label::new(cx, "Add/Remove Sand Grains").top(Units::Pixels(10.0));

            ParamSlider::new(cx, EditorData::params, |params| {
                &params.sandpile_editor_state.user_pile_amount
            })
            .top(Units::Pixels(4.0));

            Button::new(
                cx,
                |cx| cx.emit(SandpileEvent::Reset),
                |cx| Label::new(cx, "Reset"),
            )
            .top(Units::Pixels(10.0));

            WaveformCanvas::new(cx, editor_data.audio_buffer.clone()).top(Units::Pixels(10.0));
        })
        // .row_between(Pixels(0.0));
        .child_left(Stretch(0.5))
        .child_right(Stretch(0.5));
    })
}
