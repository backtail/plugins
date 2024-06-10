mod params;
mod consts;

mod simple_delay;
mod granu_delay;

use nih_plug::prelude::*;
use std::{sync::Arc, time::Instant};
use yanel_dsp::DSPUtility;

use params::GranuDelayParams;
use granu_delay::GranuDelay;

use granulator::UserSettings;

const SETTINGS: UserSettings = UserSettings {
    master_volume: 0.7,
    active_grains: 0.2,
    offset: 0.0,
    grain_size: 0.5,
    pitch: 0.5,
    delay: 0.5,
    velocity: 0.5,

    sp_offset: 0.05,
    sp_grain_size: 0.2,
    sp_pitch: 0.2,
    sp_delay: 0.5,
    sp_velocity: 0.5,

    window_function: 0,
    window_param: 0.0,

    scale: 0,
    mode: 0,
};

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

        self.buffer.l = vec![0_f32; consts::MAX_DELAY_TIME.seconds_to_samples(self.sr) as usize];
        self.buffer.r = vec![0_f32; consts::MAX_DELAY_TIME.seconds_to_samples(self.sr) as usize];

        self.delay.l.set_buffer(self.buffer.l.as_mut_slice());
        self.delay.r.set_buffer(self.buffer.r.as_mut_slice());

        self.granu.l.set_audio_buffer(self.buffer.l.as_slice());
        self.granu.r.set_audio_buffer(self.buffer.r.as_slice());
        self.granu.r.set_sample_rate(self.sr as usize).unwrap();
        self.granu.r.set_sample_rate(self.sr as usize).unwrap();

        // temporary

        self.granu.l.update_all_user_settings(&SETTINGS);
        self.granu.r.update_all_user_settings(&SETTINGS);

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

        self.delay.l.set_delay(
            self.params
                .l_delay_time
                .smoothed
                .next_step(n_samples)
                .seconds_to_samples(self.sr),
        );
        self.delay.r.set_delay(
            self.params
                .r_delay_time
                .smoothed
                .next_step(n_samples)
                .seconds_to_samples(self.sr),
        );

        let feedback = self.params.feedback.smoothed.next_step(n_samples);

        self.delay.l.set_feedback(feedback);
        self.delay.r.set_feedback(feedback);

        let mix = self.params.mix.smoothed.next_step(n_samples);

        self.granu.l.update_scheduler(now - self.last_time);
        self.granu.r.update_scheduler(now - self.last_time);

        self.last_time = now;

        for channel_samples in buffer.iter_samples() {
            let mut samples = channel_samples.into_iter();
            let (l_out, r_out) = (samples.next().unwrap(), samples.next().unwrap());
            let (l_in, r_in) = (l_out.clone(), r_out.clone());
            let (mut l_sample, mut r_sample) = (l_in.clone(), r_in.clone());

            // granular processing before delay creates pitch chains, but also infinte amplitudes, since the feedback is applied after grains

            if self.params.enable_delay.value(){
                l_sample = self.delay.l.tick(l_sample);
                r_sample = self.delay.r.tick(r_sample);
            } else {
                l_sample = self.delay.l.tick(0.0);
                r_sample = self.delay.r.tick(0.0);
            }

            if self.params.enable_granu.value() {
                l_sample += self.granu.l.get_next_sample();
                r_sample += self.granu.r.get_next_sample();
            }

            // dry/wet mixing
            (*l_out, *r_out) = ((1.0 - mix) * l_in + mix * l_sample, (1.0 - mix) * r_in + mix * r_sample);
        
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
