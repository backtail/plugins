use embedded_audio_tools::stereo::stereo_pan_unchecked;
use nih_plug::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};

struct StereoVCA {
    params: Arc<StereoVCAParams>,
    adsr: embedded_audio_tools::AudioRateADSR,
}

#[derive(Params)]
struct StereoVCAParams {
    #[id = "Pan"]
    pub pan: FloatParam,

    #[id = "Attack"]
    pub attack: FloatParam,

    #[id = "Decay"]
    pub decay: FloatParam,

    #[id = "Sustain"]
    pub sustain: FloatParam,

    #[id = "Release"]
    pub release: FloatParam,

    #[id = "Gate"]
    pub gate: BoolParam,

    gate_state: AtomicBool,
}

impl Default for StereoVCA {
    fn default() -> Self {
        Self {
            params: Arc::new(StereoVCAParams::default()),
            adsr: embedded_audio_tools::AudioRateADSR::new(0.01, 0.1, 0.0, 0.1, 0.5, 48_000.0),
        }
    }
}

impl Default for StereoVCAParams {
    fn default() -> Self {
        Self {
            pan: FloatParam::new(
                "Pan",
                0.0,
                FloatRange::SymmetricalSkewed {
                    min: -1.0,
                    max: 1.0,
                    factor: 0.5,
                    center: 0.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_panning()),

            attack: FloatParam::new(
                "Attack",
                0.5,
                FloatRange::Linear {
                    min: 0.001,
                    max: 3.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(3))
            .with_unit(" s"),

            decay: FloatParam::new(
                "Decay",
                0.5,
                FloatRange::Linear {
                    min: 0.001,
                    max: 5.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(3))
            .with_unit(" s"),

            sustain: FloatParam::new("Sustain", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_rounded(2)),

            release: FloatParam::new(
                "Release",
                0.5,
                FloatRange::Linear {
                    min: 0.001,
                    max: 5.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(3))
            .with_unit(" s"),

            gate: BoolParam::new("Gate", false),
            gate_state: AtomicBool::new(false),
        }
    }
}

impl Plugin for StereoVCA {
    const NAME: &'static str = "StereoVCA";
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

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // update adsr
        self.adsr.set_attack(self.params.attack.smoothed.next());
        self.adsr.set_decay(self.params.decay.smoothed.next());
        self.adsr.set_sustain(self.params.sustain.smoothed.next());
        self.adsr.set_release(self.params.release.smoothed.next());

        // check if gate was triggers
        let gate_state = self.params.gate_state.load(Relaxed);
        let current_gate = self.params.gate.value();

        if gate_state != current_gate && current_gate {
            self.params.gate_state.store(true, Relaxed);
            self.adsr.trigger_on();
        } else if gate_state != current_gate && !current_gate {
            self.params.gate_state.store(false, Relaxed);
            self.adsr.trigger_off();
        }

        // process buffer
        for channel_samples in buffer.iter_samples() {
            let envelope_gain = self.adsr.tick();

            let mut samples = channel_samples.into_iter();
            let (left, right) = (samples.next().unwrap(), samples.next().unwrap());

            let panned = stereo_pan_unchecked(self.params.pan.smoothed.next(), (*left, *right));
            (*left, *right) = (panned.0 * envelope_gain, panned.1 * envelope_gain);
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for StereoVCA {
    const VST3_CLASS_ID: [u8; 16] = *b"StereoVCAMG.....";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
        Vst3SubCategory::Reverb,
    ];
}

nih_export_vst3!(StereoVCA);
