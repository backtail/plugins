use nih_plug::prelude::*;
use std::sync::Arc;

struct Delay {
    params: Arc<DelayParams>,
    l_delay: yanel_dsp::SimpleDelay,
    r_delay: yanel_dsp::SimpleDelay,
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
            l_delay: yanel_dsp::SimpleDelay::init(48_000),
            r_delay: yanel_dsp::SimpleDelay::init(48_000),
        }
    }
}

impl Default for DelayParams {
    fn default() -> Self {
        DelayParams {
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

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Do this only once
        self.l_delay.check_buffer_alignment();
        self.r_delay.check_buffer_alignment();

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let delay_time = self.params.delay.smoothed.next();
        self.l_delay.set_delay_in_secs(delay_time);
        self.r_delay.set_delay_in_secs(delay_time);

        let feedback = self.params.feedback.smoothed.next();

        self.l_delay.set_feedback(feedback);
        self.r_delay.set_feedback(feedback);

        let mix = self.params.mix.smoothed.next();

        self.l_delay.set_dry(1.0 - mix);
        self.l_delay.set_wet(mix);
        self.r_delay.set_dry(1.0 - mix);
        self.r_delay.set_wet(mix);

        for channel_samples in buffer.iter_samples() {
            let mut samples = channel_samples.into_iter();
            let (left, right) = (samples.next().unwrap(), samples.next().unwrap());
            *left = self.l_delay.tick(*left);
            *right = self.r_delay.tick(*right);
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
