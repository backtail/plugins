use nih_plug::prelude::*;
use std::{sync::Arc, time::Instant};
use yanel_dsp::DSPUtility;

use crate::{consts, granu_delay::GranuDelay, util::apply_granu_settings};

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

        apply_granu_settings(&mut self.granu_settings, &self.params, &mut self.granu.l);
        apply_granu_settings(&mut self.granu_settings, &self.params, &mut self.granu.r);

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
                .delay
                .time_l
                .smoothed
                .next_step(n_samples)
                .seconds_to_samples(self.sr),
        );
        self.delay.r.set_delay(
            self.params
                .delay
                .time_r
                .smoothed
                .next_step(n_samples)
                .seconds_to_samples(self.sr),
        );

        let feedback = self.params.delay.feedback.smoothed.next_step(n_samples);

        self.delay.l.set_feedback(feedback);
        self.delay.r.set_feedback(feedback);

        let mix = self.params.mix.smoothed.next_step(n_samples);

        apply_granu_settings(&mut self.granu_settings, &self.params, &mut self.granu.l);
        apply_granu_settings(&mut self.granu_settings, &self.params, &mut self.granu.r);

        self.granu.l.update_scheduler(now - self.last_time);
        self.granu.r.update_scheduler(now - self.last_time);

        self.last_time = now;

        for channel_samples in buffer.iter_samples() {
            let mut samples = channel_samples.into_iter();
            let (l_out, r_out) = (samples.next().unwrap(), samples.next().unwrap());
            let (l_in, r_in) = (l_out.clone(), r_out.clone());

            let (mut l_sample, mut r_sample) = (0.0, 0.0);

            if self.params.enable_delay.value() {
                l_sample = self.delay.l.get_delayed_sample();
                r_sample = self.delay.r.get_delayed_sample();
            }
            // granular processing before delay creates pitch chains, but also infinte amplitudes, since the feedback is applied after grains

            l_sample += self.granu.l.get_next_sample();
            r_sample += self.granu.r.get_next_sample();

            if self.params.freeze.value() {
                self.delay.l.advance_on_delay_line();
                self.delay.r.advance_on_delay_line();
            } else {
                self.delay
                    .l
                    .add_to_delay_line(l_in + l_sample * self.params.delay.feedback.value());
                self.delay
                    .r
                    .add_to_delay_line(r_in + r_sample * self.params.delay.feedback.value());
            }

            // dry/wet mixing
            (*l_out, *r_out) = (
                (1.0 - mix) * l_in + mix * l_sample,
                (1.0 - mix) * r_in + mix * r_sample,
            );
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}
