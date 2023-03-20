use nih_plug::prelude::*;
use std::sync::Arc;

struct Freeverb {
    params: Arc<FreeverbParams>,
    freeverb: yanel_dsp::Freeverb,
}

#[derive(Params)]
struct FreeverbParams {
    #[id = "Damp"]
    pub damp: FloatParam,

    #[id = "Room Size"]
    pub room_size: FloatParam,

    #[id = "Width"]
    pub width: FloatParam,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "Freeze"]
    pub freeze: BoolParam,
}

impl Default for Freeverb {
    fn default() -> Self {
        Self {
            params: Arc::new(FreeverbParams::default()),
            freeverb: yanel_dsp::Freeverb::new(48_000),
        }
    }
}

impl Default for FreeverbParams {
    fn default() -> Self {
        Self {
            damp: FloatParam::new("Damp", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_unit(" %"),
            room_size: FloatParam::new("Room Size", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_unit(" %"),

            width: FloatParam::new("Width", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_unit(" %"),

            mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_unit(" %"),

            freeze: BoolParam::new("Freeze", false)
                .with_value_to_string(formatters::v2s_bool_bypass()),
        }
    }
}

impl Plugin for Freeverb {
    const NAME: &'static str = "Freeverb";
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
        self.freeverb.check_buffer_alignment();

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.freeverb.set_all(
            self.params.damp.smoothed.next(),
            self.params.room_size.smoothed.next(),
            self.params.width.smoothed.next(),
            self.params.freeze.value(),
            self.params.mix.smoothed.next(),
        );

        for channel_samples in buffer.iter_samples() {
            let mut samples = channel_samples.into_iter();
            let (left, right) = (samples.next().unwrap(), samples.next().unwrap());
            (*left, *right) = self.freeverb.tick((*left, *right));
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Freeverb {
    const VST3_CLASS_ID: [u8; 16] = *b"FreeverbMG......";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools,
        Vst3SubCategory::Reverb,
    ];
}

nih_export_vst3!(Freeverb);
