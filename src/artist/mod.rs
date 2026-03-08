//! Last.fm Artist API Endpoints
//!
//! Contains structs and methods related to working with the artist-related endpoints
//! available through the Last.fm API

use serde::Deserialize;

pub mod top_tracks;

#[derive(Debug, Deserialize)]
pub struct Endpoints {
    #[serde(rename = "toptracks")]
    pub top_tracks: Option<top_tracks::TopTracks>,
}
