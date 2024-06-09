
mod granu_delay;

use nih_plug::prelude::*;
use std::sync::Arc;
use yanel_dsp::DSPUtility;

use embedded_audio_tools as tools;
use granu_delay::SimpleStereoDelay;

const MAX_DELAY_TIME: f32 = 10.0; // seconds

struct Delay {
    params: Arc<DelayParams>,
    l_delay_buffer: Vec<f32>,
    r_delay_buffer: Vec<f32>,
    delay: SimpleStereoDelay,
    sr: f32,
}

#[derive(Params)]
struct DelayParams {
    #[id = "L Delay"]
    pub l_delay_time: FloatParam,

    #[id = "R Delay"]
    pub r_delay_time: FloatParam,

    #[id = "Feedback"]
    pub feedback: FloatParam,

    #[id = "Mix"]
    pub mix: FloatParam,
}

impl Default for Delay {
    fn default() -> Self {
        Self {
            params: Arc::new(DelayParams::default()),
            l_delay_buffer: vec![],
            r_delay_buffer: vec![],
            delay: SimpleStereoDelay::init(),
            sr: 48_000.0,
        }
    }
}

impl Default for DelayParams {
    fn default() -> Self {
        DelayParams {
            l_delay_time: FloatParam::new(
                "L Delay",
                0.4,
                FloatRange::Skewed {
                    min: 0.01,
                    max: MAX_DELAY_TIME,
                    factor: 0.5,
                },
            )
            .with_unit(" s")
            .with_value_to_string(formatters::v2s_f32_rounded(3)),

            r_delay_time: FloatParam::new(
                "R Delay",
                0.4,
                FloatRange::Skewed {
                    min: 0.01,
                    max: MAX_DELAY_TIME,
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
    const NAME: &'static str = "Granular Delay";
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
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sr = buffer_config.sample_rate;

        self.l_delay_buffer = vec![0_f32; MAX_DELAY_TIME.seconds_to_samples(self.sr) as usize];
        self.r_delay_buffer = vec![0_f32; MAX_DELAY_TIME.seconds_to_samples(self.sr) as usize];

        self.delay.left.set_buffer(self.l_delay_buffer.as_mut_slice());
        self.delay.right.set_buffer(self.r_delay_buffer.as_mut_slice());

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let n_samples = buffer.samples() as u32;

        self.delay.left.set_delay(
            self.params
                .l_delay_time
                .smoothed
                .next_step(n_samples)
                .seconds_to_samples(self.sr),
        );
        self.delay.right.set_delay(
            self.params
                .r_delay_time
                .smoothed
                .next_step(n_samples)
                .seconds_to_samples(self.sr),
        );

        let feedback = self.params.feedback.smoothed.next_step(n_samples);

        self.delay.left.set_feedback(feedback);
        self.delay.right.set_feedback(feedback);

        let mix = self.params.mix.smoothed.next_step(n_samples);

        self.delay.left.set_dry(1.0 - mix);
        self.delay.left.set_wet(mix);
        self.delay.right.set_dry(1.0 - mix);
        self.delay.right.set_wet(mix);

        for channel_samples in buffer.iter_samples() {
            let mut samples = channel_samples.into_iter();
            let (left, right) = (samples.next().unwrap(), samples.next().unwrap());
            *left = self.delay.left.tick(*left);
            *right = self.delay.right.tick(*right);
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Delay {
    const VST3_CLASS_ID: [u8; 16] = *b"GranuDelayMG....";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_vst3!(Delay);
