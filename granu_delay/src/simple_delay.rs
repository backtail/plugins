use core::ops::Neg;

use embedded_audio_tools as tools;

use tools::{
    memory_access::{from_slice_mut, null_mut},
    stereo::crossfade_correlated_unchecked,
    DelayLine,
};

const MIN_DELAY_SAMPLES: f32 = 32.0;

pub struct SimpleDelay {
    delay_line: tools::DelayLine,

    delay_samples: f32,
    feedback: f32,

    delay_time_changed: bool,
    last_delay_samples: f32,
    crossfade_counter: usize,
    crossfade_samples: usize,
}

impl SimpleDelay {
    pub fn init() -> SimpleDelay {
        SimpleDelay {
            delay_line: (DelayLine::new(null_mut())),

            delay_samples: MIN_DELAY_SAMPLES,
            feedback: 0.5,

            delay_time_changed: false,
            last_delay_samples: 0.0,
            crossfade_counter: 0,
            crossfade_samples: 480,
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Public Interface
    ///////////////////////////////////////////////////////////////////////////////

    pub fn tick(&mut self, input: f32) -> f32 {
        let output = self.get_delayed_sample() * self.feedback;

        self.delay_line.write_and_advance(input + output);

        output
    }

    pub fn set_buffer(&mut self, buffer: &mut [f32]) {
        self.delay_line.change_buffer(from_slice_mut(buffer));
    }

    /// Set the delay length in samples
    ///
    /// Sample rate depending calculations should be performed earlier!
    pub fn set_delay(&mut self, samples: f32) {
        let new_delay = samples.clamp(MIN_DELAY_SAMPLES, self.delay_line.len() as f32);

        if new_delay != self.delay_samples {
            self.last_delay_samples = self.delay_samples;
            self.delay_time_changed = true;
        }

        self.delay_samples = new_delay;
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 1.0);
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Private Functions
    ///////////////////////////////////////////////////////////////////////////////

    fn get_delayed_sample(&mut self) -> f32 {
        // get delayed sample from newest delay time
        let new_delayed = self
            .delay_line
            .read_lerp_wrapped_at(self.delay_samples.neg());

        // crossfade between new and old delay time samples
        if self.delay_time_changed {
            if self.crossfade_counter < self.crossfade_samples {
                self.crossfade_counter += 1;

                return crossfade_correlated_unchecked(
                    self.get_normalized_bipolar_crossfade(),
                    (
                        self.delay_line
                            .read_lerp_wrapped_at(self.last_delay_samples.neg()),
                        new_delayed,
                    ),
                );
            } else {
                self.delay_time_changed = false;
                self.crossfade_counter = 0;

                return new_delayed;
            }
        } else {
            return new_delayed;
        }
    }

    #[inline(always)]
    fn get_normalized_bipolar_crossfade(&self) -> f32 {
        (self.crossfade_counter as f32 / self.crossfade_samples as f32) * 2.0 - 1.0
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // use super::*;
}
