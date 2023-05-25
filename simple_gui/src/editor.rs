use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::subwindow::Subwindow;
use crate::SimpleGUIParams;

#[derive(Clone, Lens)]
pub struct Data {
    pub(crate) params: Arc<SimpleGUIParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 1000))
}

pub(crate) fn create(editor_state: Arc<ViziaState>, editor_data: Data) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        editor_data.clone().build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Simple GUI")
                .font_family(vec![FamilyOwned::Name(String::from(
                    assets::NOTO_SANS_THIN,
                ))])
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Slider").top(Units::Pixels(10.0));
            ParamSlider::new(cx, Data::params, |params| &params.slider).top(Units::Pixels(4.0));

            ParamButton::new(cx, Data::params, |params| &params.button).top(Units::Pixels(10.0));

            Label::new(cx, "Window").top(Units::Pixels(10.0));
            Subwindow::new(cx).top(Units::Pixels(4.0));

            Label::new(cx, "Example used for UI expermients!").top(Units::Pixels(10.0));
            Label::new(cx, "Does not make or influence any sound").top(Units::Pixels(10.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}
