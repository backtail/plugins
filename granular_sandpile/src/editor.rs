use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::{Arc, Mutex};

use crate::subwindow::Subwindow;
use crate::GranuSandpileParams;
use crate::Sandpile;

#[derive(Clone, Lens)]
pub struct Data {
    pub(crate) params: Arc<GranuSandpileParams>,
    pub(crate) sandpile: Arc<Mutex<Sandpile>>,
}

pub enum SandpileEvent {
    Reset,
    Add,
    Remove,
}

impl Model for Data {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|sandpile_event, _| match sandpile_event {
            SandpileEvent::Reset => {
                let mut s = self.sandpile.lock().unwrap();
                s.reset();
                s.set_value_at(5000, (12, 12));
            }
            SandpileEvent::Add => {
                let mut s = self.sandpile.lock().unwrap();
                s.add_at(
                    self.params.sandpile_editor_state.user_pile_amount.value() as usize,
                    (12, 12),
                );
            }
            SandpileEvent::Remove => {
                let mut s = self.sandpile.lock().unwrap();
                s.remove_at(
                    self.params.sandpile_editor_state.user_pile_amount.value() as usize,
                    (12, 12),
                );
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 600))
}

pub(crate) fn create(editor_state: Arc<ViziaState>, editor_data: Data) -> Option<Box<dyn Editor>> {
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
            Subwindow::new(cx, editor_data.sandpile.clone()).top(Units::Pixels(4.0));

            ParamButton::new(cx, Data::params, |params| {
                &params.sandpile_editor_state.run_break_button
            })
            .top(Units::Pixels(4.0));

            Button::new(
                cx,
                |cx| cx.emit(SandpileEvent::Reset),
                |cx| Label::new(cx, "Reset"),
            )
            .top(Units::Pixels(4.0));

            ParamSlider::new(cx, Data::params, |params| {
                &params.sandpile_editor_state.user_pile_amount
            })
            .top(Units::Pixels(10.0));

            Button::new(
                cx,
                |cx| cx.emit(SandpileEvent::Add),
                |cx| Label::new(cx, "Add"),
            )
            .top(Units::Pixels(4.0));

            Button::new(
                cx,
                |cx| cx.emit(SandpileEvent::Remove),
                |cx| Label::new(cx, "Remove"),
            )
            .top(Units::Pixels(4.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}
