mod consts;
mod granu_delay;
mod params;
mod plugin;
mod simple_delay;
mod util;

use granu_delay::GranuDelay;
use nih_plug::prelude::*;

impl Vst3Plugin for GranuDelay {
    const VST3_CLASS_ID: [u8; 16] = *b"GranuDelayMG....";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_vst3!(GranuDelay);
