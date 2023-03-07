use nih_plug::prelude::*;
use nih_plug::util;
use std::sync::Arc;

#[derive(Enum, PartialEq)]
enum FilterType {
    Lowpass,
    Highpass,
    Allpass,
    Notch,
    Bell,
    LowShelf,
}

struct Filters {
    params: Arc<LowpassParams>,
    filter: yanel_dsp::MultiFilter,
}

#[derive(Params)]
struct LowpassParams {
    #[id = "Filter Type"]
    pub filter_type: EnumParam<FilterType>,

    #[id = "Cutoff"]
    pub cutoff: FloatParam,

    #[id = "Q"]
    pub q: FloatParam,

    #[id = "Gain"]
    pub gain: FloatParam,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            params: Arc::new(LowpassParams::default()),
            filter: yanel_dsp::MultiFilter::init(48_000),
        }
    }
}

impl Default for LowpassParams {
    fn default() -> Self {
        Self {
            filter_type: EnumParam::new("Filter Type", FilterType::Lowpass),
            cutoff: FloatParam::new(
                "Cutoff",
                5_100.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20_000.0,
                    factor: 0.5,
                },
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2)),

            q: FloatParam::new(
                "Q",
                1.0,
                FloatRange::SymmetricalSkewed {
                    min: 0.1,
                    max: 10.0,
                    factor: 0.5,
                    center: 1.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            gain: FloatParam::new(
                "Gain",
                1.0,
                FloatRange::Skewed {
                    min: util::db_to_gain(-20.0),
                    max: util::db_to_gain(20.0),
                    factor: FloatRange::gain_skew_factor(-20.0, 20.0),
                },
            )
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for Filters {
    const NAME: &'static str = "Multi Filter";
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
        for channel_samples in buffer.iter_samples() {
            self.filter.set_all(
                self.params.filter_type.value() as u8,
                self.params.cutoff.smoothed.next(),
                self.params.q.smoothed.next(),
                self.params.gain.smoothed.next(),
            );

            for sample in channel_samples {
                *sample = self.filter.next(*sample);
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Filters {
    const VST3_CLASS_ID: [u8; 16] = *b"MultiFilterMG...";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
        Vst3SubCategory::Filter,
    ];
}

nih_export_vst3!(Filters);
