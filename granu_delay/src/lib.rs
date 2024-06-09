
mod simple_delay;

use nih_plug::prelude::*;
use std::{sync::Arc, time::Instant};
use yanel_dsp::DSPUtility;

use embedded_audio_tools as tools;
use simple_delay::SimpleStereoDelay;
use granulator::{Granulator, UserSettings};


const MAX_DELAY_TIME: f32 = 2.0; // seconds
const SETTINGS: UserSettings = UserSettings::new_empty();

struct GranuDelay {
    params: Arc<GranuDelayParams>,
    l_buffer: Vec<f32>,
    r_buffer: Vec<f32>,
    delay: SimpleStereoDelay,
    granu: Granulator,
    sr: f32,
    last_time: Instant,
}

#[derive(Params)]
struct GranuDelayParams {
    #[id = "L Delay"]
    pub l_delay_time: FloatParam,

    #[id = "R Delay"]
    pub r_delay_time: FloatParam,

    #[id = "Feedback"]
    pub feedback: FloatParam,

    #[id = "Mix"]
    pub mix: FloatParam,

    #[id = "Enable Delay"]
    pub enable_delay: BoolParam,

    #[id = "Enable Granu"]
    pub enable_granu: BoolParam,
}

impl Default for GranuDelay {
    fn default() -> Self {
        Self {
            params: Arc::new(GranuDelayParams::default()),
            l_buffer: vec![],
            r_buffer: vec![],
            delay: SimpleStereoDelay::init(),
            granu: Granulator::new(48_000),
            sr: 48_000.0,
            last_time: Instant::now(),
        }
    }
}

impl Default for GranuDelayParams {
    fn default() -> Self {
        GranuDelayParams {
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

            enable_delay: BoolParam::new("Enable Delay", true),

            enable_granu: BoolParam::new("Enable Granu", false),
        }
    }
}


impl Plugin for GranuDelay {
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

        self.l_buffer = vec![0_f32; MAX_DELAY_TIME.seconds_to_samples(self.sr) as usize];
        self.r_buffer = vec![0_f32; MAX_DELAY_TIME.seconds_to_samples(self.sr) as usize];

        self.delay.left.set_buffer(self.l_buffer.as_mut_slice());
        self.delay.right.set_buffer(self.r_buffer.as_mut_slice());

        self.granu.set_audio_buffer(self.l_buffer.as_slice());
        self.granu.set_sample_rate(self.sr as usize).unwrap();

        // temporary

        self.granu.update_all_user_settings(&SETTINGS);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let n_samples = buffer.samples() as u32;
        let now = Instant::now();

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

        self.granu.update_scheduler(now - self.last_time);

        self.last_time = now;

        for channel_samples in buffer.iter_samples() {
            let mut samples = channel_samples.into_iter();
            let (l_out, r_out) = (samples.next().unwrap(), samples.next().unwrap());

            let (mut l_sample, mut r_sample) = (l_out.clone(), r_out.clone());

            // granular processing before delay creates pitch chains

            if self.params.enable_delay.value(){
                l_sample = self.delay.left.tick(l_sample);
                r_sample = self.delay.right.tick(r_sample);
            }


            // sounds fine here
            if self.params.enable_granu.value() {
                l_sample += self.granu.get_next_sample();
            }

            (*l_out, *r_out) = (l_sample, r_sample);
        
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for GranuDelay {
    const VST3_CLASS_ID: [u8; 16] = *b"GranuDelayMG....";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_vst3!(GranuDelay);
