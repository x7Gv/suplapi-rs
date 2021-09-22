//! Access Nelonen Media Supla API from Rust.
//!
//! # Example(s)
//! extern crate suplapi;
//!
//! let groove_fm = 70;
//!
//! let supla = suplapi::SuplAPI::<suplapi::http::default::Client>::default();
//!
//! tokio_test::block_on(async {
//!     let playlist = supla.playlist(groove_fm, 20, None).await.unwrap();
//!     assert!(playlist.items.len() == 20);
//! });
//! ```
#[cfg(feature="http-client")] extern crate reqwest;
#[cfg(feature="http-client")] extern crate url;
#[macro_use] extern crate failure;
extern crate serde_json;

use std::io;
use std::result;

use data::Playlist;

pub mod http;

/// Data structures related to API JSON output.
pub mod data {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Track {
        /// Timestamp at when the track is/was played.
        pub timestamp: i64,
        /// Playing date
        pub date: String,
        /// Channel ID // TODO: enums
        pub channel: i32,
        /// Name(s) of the tracks artist(s)
        pub artist: String,
        /// Name of the tracks song.
        pub song: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Playlist {
        /// Collection of played tracks
        pub items: Vec<Track>,
        /// Token to the next track
        pub next_token: i64,
    }
}

#[derive(Fail, Debug)]
pub enum Error {

    #[fail(display = "HTTP Error")]
    HTTPError,

    #[fail(display = "IO Error: {}", _0)]
    IOError(#[cause] io::Error),

    #[fail(display = "JSON Error: {}", _0)]
    JSONError(#[cause] serde_json::error::Error),

    #[fail(display = "JSON Path Error")]
    JSONPathError,

    #[fail(display = "Invalid Parameter: {}", _0)]
    InvalidParameter(String),
}

pub type Result<T> = result::Result<T, Error>;

/// The main interface to interact with.
#[derive(Debug, Clone)]
pub struct SuplAPI<A: http::HttpClient> {
    pub client: A,
    pub base_url: String,
}

impl<A: http::HttpClient + Default> Default for SuplAPI<A> {
    fn default() -> Self {
        SuplAPI::new(A::default())
    }
}

impl<A: http::HttpClient> SuplAPI<A> {

    pub fn new(mut client: A) -> Self {
        client.user_agent("suplapi ()".to_owned());
        SuplAPI {
            client: client,
            base_url: ".nm-services.nelonenmedia.fi".to_owned(),
        }
    }

    pub fn playlist_url(&self) -> String {
        format!("{}{}", "https://supla-playlist", self.base_url)
    }

    async fn query<'a, I>(&self, base_url: String, args: I) -> Result<serde_json::Value>
        where I: Iterator<Item=(&'a str, &'a str)> + Send {

        let resp = self.client.get(&base_url, args).await.map_err(|_| Error::HTTPError)?;
        let json = serde_json::from_str(&resp).map_err(Error::JSONError)?;

        Ok(json)
    }

    /// Query playlist
    pub async fn playlist(&self, channel: i32, limit: i32, next_token: Option<i64>) -> Result<Playlist> {

        let url = format!("{}{}", self.playlist_url(), "/playlist?");

        let data: Playlist;

        if let Some(token) = next_token {
             data = serde_json::from_value(self.query(url, vec![
                 ("channel", format!("{}", channel).as_str()),
                 ("limit", format!("{}", limit).as_str()),
                 ("next_token", format!("{}", token).as_str()),
             ].into_iter()).await?).map_err(Error::JSONError)?;
        } else {
             data = serde_json::from_value(self.query(url, vec![
                ("channel", format!("{}", channel).as_str()),
                ("limit", format!("{}", limit).as_str()),
             ].into_iter()).await?).map_err(Error::JSONError)?;
        }

        Ok(data)
    }
}
