use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

mod editor;
mod subwindow;

struct SimpleGUI {
    params: Arc<SimpleGUIParams>,
}

#[derive(Params)]
pub struct SimpleGUIParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "Slider"]
    pub slider: FloatParam,

    #[id = "Button"]
    pub button: BoolParam,
}

impl Default for SimpleGUI {
    fn default() -> Self {
        Self {
            params: Arc::new(SimpleGUIParams::default()),
        }
    }
}

impl Default for SimpleGUIParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            slider: FloatParam::new(
                "Pan",
                0.0,
                FloatRange::SymmetricalSkewed {
                    min: -1.0,
                    max: 1.0,
                    factor: 0.5,
                    center: 0.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            button: BoolParam::new("Button", false),
        }
    }
}

impl Plugin for SimpleGUI {
    const NAME: &'static str = "Simple GUI";
    const VENDOR: &'static str = "Max Genson";
    const URL: &'static str = "https://www.maxgenson.de";
    const EMAIL: &'static str = "mail@maxgenson.de";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.editor_state.clone(),
            editor::Data {
                params: self.params.clone(),
            },
        )
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // do nothing

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for SimpleGUI {
    const VST3_CLASS_ID: [u8; 16] = *b"SimpleGUIMG.....";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
        Vst3SubCategory::Reverb,
    ];
}

nih_export_vst3!(SimpleGUI);
