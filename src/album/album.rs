use std::marker::PhantomData;

use crate::{
    album::Endpoints,
    error::{Error, LastFMError},
    Client, RequestBuilder,
};

/// The main album structure.
///
/// This structure provides the information about an album.
pub type Album = crate::model::Album;

impl Album {
    pub async fn info_by_mbid<'a>(client: &'a mut Client, mbid: &str) -> RequestBuilder<'a, Album> {
        let url = client.build_url(vec![("method", "album.getInfo"), ("mbid", mbid)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }

    pub async fn info<'a>(client: &'a mut Client, artist: &str, album: &str) -> RequestBuilder<'a, Album> {
        let url = client.build_url(vec![("method", "album.getInfo"), ("artist", artist), ("album", album)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, Album> {
    add_param!(with_limit, limit, usize);
    add_param!(with_page, page, usize);

    pub async fn send(&'a mut self) -> Result<Album, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastfm_error) => Err(Error::LastFMError(lastfm_error.into())),
                    Err(_) => match serde_json::from_str::<Endpoints>(&body) {
                        Ok(albums) => Ok(albums.album.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn album_info_by_mbid(&'a mut self, mbid: &str) -> RequestBuilder<'a, Album> {
        Album::info_by_mbid(self, mbid).await
    }

    pub async fn album_info(&'a mut self, artist: &str, album: &str) -> RequestBuilder<'a, Album> {
        Album::info(self, artist, album).await
    }
}
