use nih_plug::prelude::*;
use std::sync::Arc;

struct Delay {
    params: Arc<DelayParams>,
    delay: yanel_dsp::SimpleDelay,
}

#[derive(Params)]
struct DelayParams {
    #[id = "Delay"]
    pub delay: FloatParam,

    #[id = "Feedback"]
    pub feedback: FloatParam,

    #[id = "Mix"]
    pub mix: FloatParam,
}

impl Default for Delay {
    fn default() -> Self {
        Self {
            params: Arc::new(DelayParams::default()),
            delay: yanel_dsp::SimpleDelay::init(48_000),
        }
    }
}

impl Default for DelayParams {
    fn default() -> Self {
        Self {
            delay: FloatParam::new(
                "Delay",
                0.4,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 3.0,
                    factor: 0.5,
                },
            )
            .with_unit(" s")
            .with_value_to_string(formatters::v2s_f32_rounded(3)),

            feedback: FloatParam::new("Feedback", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),

            mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
        }
    }
}

impl Plugin for Delay {
    const NAME: &'static str = "Simple Delay";
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
        self.delay
            .set_delay_in_secs(self.params.delay.smoothed.next());
        self.delay
            .set_feedback(self.params.feedback.smoothed.next());
        let mix = self.params.mix.smoothed.next();

        self.delay.set_dry(1.0 - mix);
        self.delay.set_wet(mix);

        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                *sample = self.delay.tick(*sample);
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Delay {
    const VST3_CLASS_ID: [u8; 16] = *b"SimpleDelayMG...";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_vst3!(Delay);
