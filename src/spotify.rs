use dbus::{
    arg::PropMap,
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
};

use crate::grimoire;

pub struct Spotify {
    connection: Connection,
}

#[derive(Default)]
pub struct SpotifyData {
    data: PropMap,
}

impl SpotifyData {
    pub fn artist(&self) -> Result<String, dbus::Error> {
        let artists = self
            .data
            .get("xesam:artist")
            .ok_or(dbus::Error::new_failed("Empty artists list"))?
            .0
            .as_iter()
            .ok_or(dbus::Error::new_failed("Could not iterate over artists"))?
            .map(|a| a.as_str().unwrap())
            .collect::<Vec<&str>>();
        Ok(artists.join(", ").trim_end_matches(", ").to_string())
    }
    pub fn title(&self) -> Result<String, dbus::Error> {
        let title = self
            .data
            .get("xesam:title")
            .ok_or(dbus::Error::new_failed("Failed to retrieve title"))?
            .0
            .as_str()
            .ok_or(dbus::Error::new_failed("Failed to transform into str"))?;
        Ok(title.to_string())
    }
}

impl Spotify {
    pub fn new() -> Result<Self, dbus::Error> {
        let connection = Connection::new_session()?;
        Ok(Self { connection })
    }

    pub fn get_data(&self) -> Result<SpotifyData, dbus::Error> {
        self.connection
            .with_proxy(grimoire::DESTINATION, grimoire::PATH, grimoire::TIMEOUT)
            .get("org.mpris.MediaPlayer2.Player", "Metadata")
            .map(|data| SpotifyData { data })
    }
}
