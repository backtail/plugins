use nih_plug::prelude::*;
use crate::consts;

#[derive(Params)]
pub struct GranuDelayParams {
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

impl Default for GranuDelayParams {
    fn default() -> Self {
        GranuDelayParams {
            l_delay_time: FloatParam::new(
                "L Delay",
                0.4,
                FloatRange::Skewed {
                    min: 0.01,
                    max: consts::MAX_DELAY_TIME,
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
                    max: consts::MAX_DELAY_TIME,
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

