use std::sync::{Arc, Mutex};

use nih_plug::prelude::*;
use nih_plug_vizia::{vizia::prelude::*, ViziaState};

use sandpile::Sandpile;

use audrey::open;

mod editor;
mod sandpile;
mod sandpile_canvas;
mod waveform_canvas;

struct GranuSandpile {
    params: Arc<GranuSandpileParams>,
    sandpile: Arc<Mutex<Sandpile>>,
    audio_buffer: Arc<Vec<f32>>,
    next_bar: f64,
}

#[derive(Clone, Lens)]
pub struct EditorData {
    pub params: Arc<GranuSandpileParams>,
    pub sandpile: Arc<Mutex<Sandpile>>,

    // Audio
    pub audio_buffer: Arc<Vec<f32>>,

    // Window
    pub mouse_xy: (f32, f32),
    pub canvas_xy: Arc<Mutex<(f32, f32)>>,
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
    #[id = "user_pile_amount"]
    user_pile_amount: IntParam,
}

impl Default for GranuSandpile {
    fn default() -> Self {
        // load wav file
        let guitar_buffer = open("granular_sandpile/src/samples/guitar.wav")
            .unwrap()
            .frames::<[f32; 1]>()
            .map(|frame| frame.unwrap()[0])
            .collect::<Vec<_>>();

        Self {
            params: Arc::new(GranuSandpileParams::default()),
            sandpile: Arc::new(Mutex::new(Sandpile::new(50, 50))),
            audio_buffer: Arc::new(guitar_buffer),
            next_bar: 0.0,
        }
    }
}

impl Default for GranuSandpileParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            sandpile_editor_state: SandpileEditorParams {
                user_pile_amount: IntParam::new(
                    "Pile Amount",
                    9,
                    IntRange::Linear { min: 0, max: 100 },
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
            EditorData {
                params: self.params.clone(),
                sandpile: self.sandpile.clone(),
                audio_buffer: self.audio_buffer.clone(),
                mouse_xy: (0.0, 0.0),
                canvas_xy: Arc::new(Mutex::new((0.0, 0.0))),
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
            let current_pos = t.pos_beats().unwrap() * 4.0;
            if current_pos > self.next_bar {
                self.next_bar = current_pos.floor() + 1.0;
                {
                    let mut s = self.sandpile.lock().unwrap();
                    s.topple_sandpile();
                }

                if current_pos as usize % 8 == 0 {
                    let _s = self.sandpile.lock().unwrap();
                    // println!("{:?}", s.row_averages());
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
