use serde::Deserialize;
use std::marker::PhantomData;

use crate::{
    error::{Error, LastFMError},
    model::Attributes,
    user::{Album, User},
    Client, RequestBuilder,
};
use crate::user::top_artists::Period;

/// The main top albums structure.
///
/// This is splitted off into two areas: One, the attributes (used for displaying various
/// user-associated attributes), and two, the user's top albums.
///
/// For details on the attributes available, refer to [Attributes]. For details on the album information
/// available, refer to [Album].
#[derive(Debug, Deserialize)]
pub struct TopAlbums {
    /// A [Vec] array containing a user's Top Albums.
    #[serde(rename = "album")]
    pub albums: Vec<Album>,
    /// Various internal API attributes.
    #[serde(rename = "@attr")]
    pub attrs: Attributes,
}

impl TopAlbums {
    /// Constructs / builds the request to the user.getTopArtists API endpoint.
    pub async fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, TopAlbums> {
        let url = client.build_url(vec![("method", "user.getTopAlbums"), ("user", user)]).await;
        RequestBuilder { client, url, phantom: PhantomData }
    }
}

impl<'a> RequestBuilder<'a, TopAlbums> {
    add_param!(with_limit, limit, usize);
    add_param!(within_period, period, Period);
    add_param!(with_page, page, usize);

    pub async fn send(&'a mut self) -> Result<TopAlbums, Error> {
        match self.client.request(&self.url).await {
            Ok(response) => {
                let body = response.text().await.unwrap();
                match serde_json::from_str::<LastFMError>(&body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&body) {
                        Ok(user) => Ok(user.top_albums.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub async fn top_albums(&'a mut self, user: &str) -> RequestBuilder<'a, TopAlbums> {
        TopAlbums::build(self, user).await
    }
}
