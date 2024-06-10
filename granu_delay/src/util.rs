use granulator::{Granulator, UserSettings};

use crate::params::GranuParams;

pub fn apply_granu_settings(
    granu_settings: &mut UserSettings,
    vst_settings: &GranuParams,
    granu: &mut Granulator,
) {
    granu_settings.active_grains = vst_settings.n_grains.value();
    granu_settings.sp_delay = vst_settings.rand_delay.value();
    granu_settings.sp_pitch = vst_settings.rand_pitch.value();
    granu_settings.pitch = vst_settings.pitch.value();
    granu_settings.sp_grain_size = vst_settings.rand_grain_length.value();
    granu_settings.grain_size = vst_settings.grain_length.value();
    granu_settings.sp_offset = vst_settings.rand_buffer_offset.value();
    granu_settings.offset = vst_settings.buffer_offset.value();

    granu.update_all_user_settings(&granu_settings);
}
