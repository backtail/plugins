use nih_plug::prelude::*;
use std::sync::Arc;

struct Kick {
    params: Arc<KickParams>,
    kick: yanel_dsp::SynthKick,
}

#[derive(Params)]
struct KickParams {
    #[id = "Pitch"]
    pub pitch: FloatParam,

    #[id = "Attack"]
    pub attack: FloatParam,

    #[id = "Decay"]
    pub decay: FloatParam,
}

impl Default for Kick {
    fn default() -> Self {
        Self {
            params: Arc::new(KickParams::default()),
            kick: yanel_dsp::SynthKick::init(48_000.0),
        }
    }
}

impl Default for KickParams {
    fn default() -> Self {
        KickParams {
            pitch: FloatParam::new("Pitch", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_rounded(2)),

            attack: FloatParam::new("Attack", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_rounded(2)),

            decay: FloatParam::new("Decay", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
        }
    }
}

impl Plugin for Kick {
    const NAME: &'static str = "Synthetic Kick";
    const VENDOR: &'static str = "Max Genson";
    const URL: &'static str = "https://www.maxgenson.de";
    const EMAIL: &'static str = "mail@maxgenson.de";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_output_channels: NonZeroU32::new(1),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
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
        self.kick.update_sr(buffer_config.sample_rate);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Handle Params
        self.kick.set_pitch(self.params.pitch.smoothed.next());
        self.kick.set_attack(self.params.attack.smoothed.next());
        self.kick.set_decay(self.params.decay.smoothed.next());

        // Setup
        let mut next_event = context.next_event();

        // Process per sample
        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Handle MIDI
            while let Some(event) = next_event {
                if event.timing() != sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn {
                        channel: _,
                        note: _,
                        timing: _,
                        velocity: _,
                        voice_id: _,
                    } => self.kick.trigger(),

                    _ => (),
                }

                next_event = context.next_event();
            }

            // Handle Audio
            let mut samples = channel_samples.into_iter();
            let out_sample = samples.next().unwrap();
            *out_sample = self.kick.tick();
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Kick {
    const VST3_CLASS_ID: [u8; 16] = *b"SynthKickMG.....";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Drum,
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Generator,
    ];
}

nih_export_vst3!(Kick);
