use std::sync::{Arc, Mutex};

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use sandpile::Sandpile;

mod editor;
mod sandpile;
mod subwindow;

struct GranuSandpile {
    params: Arc<GranuSandpileParams>,
    sandpile: Arc<Mutex<Sandpile>>,
    next_bar: f64,
}

#[derive(Params)]
pub struct GranuSandpileParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[nested(group = "sandpile-editor-state")]
    sandpile_editor_state: SandpileEditorParams,
}

#[derive(Params)]
struct SandpileEditorParams {
    #[id = "run-break-button"]
    pub run_break_button: BoolParam,

    #[id = "user_pile_amount"]
    user_pile_amount: IntParam,
}

impl Default for GranuSandpile {
    fn default() -> Self {
        let mut sandpile = Sandpile::new(25, 25);
        sandpile.set_value_at(2000, (12, 12));
        Self {
            params: Arc::new(GranuSandpileParams::default()),
            sandpile: Arc::new(Mutex::new(sandpile)),
            next_bar: 0.0,
        }
    }
}

impl Default for GranuSandpileParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            sandpile_editor_state: SandpileEditorParams {
                run_break_button: BoolParam::new("Run/Break", false),
                user_pile_amount: IntParam::new(
                    "Pile Amount",
                    9,
                    IntRange::Linear { min: 0, max: 1000 },
                ),
            },
        }
    }
}

impl Plugin for GranuSandpile {
    const NAME: &'static str = "Granular Sandpile";
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
                sandpile: self.sandpile.clone(),
            },
        )
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let t = context.transport();
        if t.playing {
            if t.pos_beats().unwrap() * 4.0 > self.next_bar {
                self.next_bar = (t.pos_beats().unwrap() * 4.0).floor() + 1.0;
                {
                    let mut s = self.sandpile.lock().unwrap();
                    s.topple_sandpile();
                }
            }
        } else {
            self.next_bar = t.bar_start_pos_beats().unwrap();
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for GranuSandpile {
    const VST3_CLASS_ID: [u8; 16] = *b"GranularSandpile";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Generator];
}

nih_export_vst3!(GranuSandpile);
