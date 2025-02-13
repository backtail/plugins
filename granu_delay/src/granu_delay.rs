use crate::{params::GranuDelayParams, simple_delay::SimpleDelay};
use granulator::{Granulator, UserSettings};
use std::{sync::Arc, time::Instant};
pub struct Stereo<T> {
    pub l: T,
    pub r: T,
}

pub struct GranuDelay {
    pub params: Arc<GranuDelayParams>,
    pub buffer: Stereo<Vec<f32>>,
    pub delay: Stereo<SimpleDelay>,
    pub granu: Stereo<Granulator>,
    pub granu_settings: UserSettings,
    pub sr: f32,
    pub last_time: Instant,
}

impl Default for GranuDelay {
    fn default() -> Self {
        Self {
            params: Arc::new(GranuDelayParams::default()),
            buffer: Stereo {
                l: vec![],
                r: vec![],
            },
            delay: Stereo {
                l: SimpleDelay::init(),
                r: SimpleDelay::init(),
            },
            granu: Stereo {
                l: Granulator::new(48_000),
                r: Granulator::new(48_000),
            },
            granu_settings: UserSettings::new_empty(),
            sr: 48_000.0,
            last_time: Instant::now(),
        }
    }
}
