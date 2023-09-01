use std::time::Duration;

pub const DESTINATION: &'static str = "org.mpris.MediaPlayer2.spotify";
pub const PATH: &'static str = "/org/mpris/MediaPlayer2";
pub const TIMEOUT: Duration = Duration::from_millis(500);
