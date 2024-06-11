use granulator::{Granulator, UserSettings};

use crate::params::GranuDelayParams;

pub fn apply_granu_settings(
    granu_settings: &mut UserSettings,
    ui_settings: &GranuDelayParams,
    granu: &mut Granulator,
) {
    if ui_settings.enable_granu.value() {
        granu_settings.active_grains = ui_settings.granu.n_grains.value();
    } else {
        granu_settings.active_grains = 0.0;
    }
    granu_settings.sp_delay = ui_settings.granu.rand_delay.value();
    granu_settings.sp_pitch = ui_settings.granu.rand_pitch.value();
    granu_settings.pitch = ui_settings.granu.pitch.value();
    granu_settings.sp_grain_size = ui_settings.granu.rand_grain_length.value();
    granu_settings.grain_size = ui_settings.granu.grain_length.value();
    granu_settings.sp_offset = ui_settings.granu.rand_buffer_offset.value();
    granu_settings.offset = ui_settings.granu.buffer_offset.value();

    // temp
    granu_settings.velocity = 1.0;
    granu_settings.sp_velocity = 0.0;

    granu.update_all_user_settings(&granu_settings);
}
