use crate::consts;
use nih_plug::prelude::*;

#[derive(Params)]
pub struct GranuDelayParams {
    #[nested(group = "DelayParams")]
    pub delay: DelayParams,

    #[nested(group = "GranuParams")]
    pub granu: GranuParams,

    // Boolean States
    #[id = "Enable Delay"]
    pub enable_delay: BoolParam,

    #[id = "Granify"]
    pub enable_granu: BoolParam,

    #[id = "Freeze"]
    pub freeze: BoolParam,

    #[id = "Quantize"]
    pub quantize: BoolParam,

    // Util
    #[id = "Mix"]
    pub mix: FloatParam,
}

#[derive(Params)]
pub struct DelayParams {
    #[id = "L Delay"]
    pub time_l: FloatParam,

    #[id = "R Delay"]
    pub time_r: FloatParam,

    #[id = "Feedback"]
    pub feedback: FloatParam,
}

#[derive(Params)]
pub struct GranuParams {
    #[id = "Spray"]
    pub rand_delay: FloatParam,

    #[id = "Density"]
    pub n_grains: FloatParam,

    #[id = "Chaos"]
    pub rand_pitch: FloatParam,

    #[id = "Pitch"]
    pub pitch: FloatParam,

    #[id = "Space"]
    pub rand_grain_length: FloatParam,

    #[id = "Room"]
    pub grain_length: FloatParam,

    #[id = "Bend"]
    pub rand_buffer_offset: FloatParam,

    #[id = "Gravity"]
    pub buffer_offset: FloatParam,
}

impl Default for GranuDelayParams {
    fn default() -> Self {
        GranuDelayParams {
            delay: DelayParams {
                time_l: FloatParam::new(
                    "L Delay",
                    consts::DEFAULT_DELAY_TIME_PERCTENTAGE,
                    FloatRange::Skewed {
                        min: consts::MIN_DELAY_TIME,
                        max: consts::MAX_DELAY_TIME,
                        factor: 0.5,
                    },
                )
                .with_unit(" s")
                .with_value_to_string(formatters::v2s_f32_rounded(3)),

                time_r: FloatParam::new(
                    "R Delay",
                    consts::DEFAULT_DELAY_TIME_PERCTENTAGE,
                    FloatRange::Skewed {
                        min: consts::MIN_DELAY_TIME,
                        max: consts::MAX_DELAY_TIME,
                        factor: 0.5,
                    },
                )
                .with_unit(" s")
                .with_value_to_string(formatters::v2s_f32_rounded(3)),

                feedback: FloatParam::new(
                    "Feedback",
                    consts::DEFAULT_FEEDBACK_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
            },

            granu: GranuParams {
                rand_delay: FloatParam::new(
                    "Spray",
                    consts::DEFAULT_SPRAY_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
                n_grains: FloatParam::new(
                    "Density",
                    consts::DEFAULT_DENSITY_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
                rand_pitch: FloatParam::new(
                    "Chaos",
                    consts::DEFAULT_CHAOS_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
                pitch: FloatParam::new("Pitch", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                    .with_value_to_string(formatters::v2s_f32_percentage(2))
                    .with_unit(" %"),
                rand_grain_length: FloatParam::new(
                    "Space",
                    consts::DEFAULT_SPACE_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
                grain_length: FloatParam::new(
                    "Room",
                    consts::DEFAULT_ROOM_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
                rand_buffer_offset: FloatParam::new(
                    "Bend",
                    consts::DEFAULT_BEND_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
                buffer_offset: FloatParam::new(
                    "Gravity",
                    consts::DEFAULT_GRAVITY_PERCTENTAGE,
                    FloatRange::Linear { min: 0.0, max: 1.0 },
                )
                .with_value_to_string(formatters::v2s_f32_percentage(2))
                .with_unit(" %"),
            },

            // Boolean States
            enable_delay: BoolParam::new("Enable Delay", true),

            enable_granu: BoolParam::new("Granify", false),

            freeze: BoolParam::new("Freeze", false),

            quantize: BoolParam::new("Quantize", false),

            // Util
            mix: FloatParam::new(
                "Dry/Wet",
                consts::DEFAULT_DRY_WET_MIX_PERCTENTAGE,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_unit(" %"),
        }
    }
}
