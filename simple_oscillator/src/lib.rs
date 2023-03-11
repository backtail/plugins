use embedded_audio_tools::phase_accumulator::PhaseAccumulator;
use nih_plug::prelude::*;
use nih_plug::util;
use std::sync::Arc;

use embedded_audio_tools::{
    oscillator::UnipolarOscillator, phase_accumulator::SoftPhaseAccumulator,
};

use Waveform::*;

#[derive(Enum, PartialEq)]
enum Waveform {
    Sine,
    Rectangle,
    Sawtooth,
    Triangle,
}

struct Oscillator {
    params: Arc<OscillatorParams>,
    osc: UnipolarOscillator<SoftPhaseAccumulator>,
}

#[derive(Params)]
struct OscillatorParams {
    #[id = "Wave"]
    pub wave: EnumParam<Waveform>,

    #[id = "Freq"]
    pub freq: FloatParam,

    #[id = "Gain"]
    pub gain: FloatParam,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            params: Arc::new(OscillatorParams::default()),
            osc: UnipolarOscillator::new(SoftPhaseAccumulator::new(100.0, 48_000.0)),
        }
    }
}

impl Default for OscillatorParams {
    fn default() -> Self {
        Self {
            wave: EnumParam::new("Wave", Waveform::Sine),
            freq: FloatParam::new(
                "Freq",
                100.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20_000.0,
                    factor: 0.5,
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2)),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-10.0),
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: 0.5,
                },
            )
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for Oscillator {
    const NAME: &'static str = "Simple Oscillator";
    const VENDOR: &'static str = "Max Genson";
    const URL: &'static str = "https://www.maxgenson.de";
    const EMAIL: &'static str = "mail@maxgenson.de";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
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
        for channel_samples in buffer.iter_samples() {
            self.osc
                .set_freq_unchecked(self.params.freq.modulated_plain_value());
            self.osc.set_wave(enum_parser(self.params.wave.value()));

            let next = self.osc.next() * self.params.gain.smoothed.next();

            for sample in channel_samples {
                *sample = next;
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Oscillator {
    const VST3_CLASS_ID: [u8; 16] = *b"SimpleOscillator";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Synth];
}

nih_export_vst3!(Oscillator);

fn enum_parser(nih_enum: Waveform) -> embedded_audio_tools::oscillator::Waveform {
    match nih_enum {
        Sine => embedded_audio_tools::oscillator::Waveform::Sine,
        Rectangle => embedded_audio_tools::oscillator::Waveform::Rectangle,
        Sawtooth => embedded_audio_tools::oscillator::Waveform::Sawtooth,
        Triangle => embedded_audio_tools::oscillator::Waveform::Triangle,
    }
}
