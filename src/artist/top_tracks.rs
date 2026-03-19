use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    artist::Endpoints,
    error::{Error, LastFMError},
    model::Track,
    Client, RequestBuilder,
};

/// The main artist top tracks structure.
#[derive(Debug, Deserialize)]
pub struct TopTracks {
    /// A [Vec] containing the artists' top played tracks.
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

impl TopTracks {
    pub async fn top_tracks_by_mbid<'a>(client: &'a mut Client, mbid: &str) -> RequestBuilder<'a, TopTracks> {
        let url = client.build_url(vec![("method", "artist.getTopTracks"), ("mbid", mbid)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }

    pub async fn top_tracks<'a>(client: &'a mut Client, artist: &str) -> RequestBuilder<'a, TopTracks> {
        let url = client.build_url(vec![("method", "artist.getTopTracks"), ("artist", artist)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, TopTracks> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);

    pub async fn send(&'a mut self) -> Result<TopTracks, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastfm_error) => Err(Error::LastFMError(lastfm_error.into())),
                    Err(_) => match serde_json::from_str::<Endpoints>(&body) {
                        Ok(artists) => Ok(artists.top_tracks.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn artist_top_tracks_by_mbid(&'a mut self, mbid: &str) -> RequestBuilder<'a, TopTracks> {
        TopTracks::top_tracks_by_mbid(self, mbid).await
    }

    pub async fn artist_top_tracks(&'a mut self, artist: &str) -> RequestBuilder<'a, TopTracks> {
        TopTracks::top_tracks(self, artist).await
    }
}
