//! Last.fm Artist API Endpoints
//!
//! Contains structs and methods related to working with the artist-related endpoints
//! available through the Last.fm API

use serde::Deserialize;

pub mod album;

#[derive(Debug, Deserialize)]
pub struct Endpoints {
    pub album: Option<album::Album>,
}
