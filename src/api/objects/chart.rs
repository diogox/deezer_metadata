//! Contains the [`Chart`](Chart) struct.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

use serde_json;
use serde_json::Value;
use serde::{
    Deserialize,
    Deserializer,
};

use api::objects::user::User;
use api::objects::track::Track;
use api::objects::album::Album;
use api::objects::artist::Artist;
use api::objects::playlist::Playlist;

#[derive(Deserialize, Serialize, Debug)]
pub struct Chart {

    /// Vector of ChartTrack objects in the Chart
    #[serde(deserialize_with = "deserialize_chart")]
    pub tracks: Vec<ChartTrack>,

    /// Vector of ChartAlbum objects in the Chart
    #[serde(deserialize_with = "deserialize_chart")]
    pub albums: Vec<ChartAlbum>,

    /// Vector of ChartArtist objects in the Chart
    #[serde(deserialize_with = "deserialize_chart")]
    pub artists: Vec<ChartArtist>,

    /// Vector of Playlist objects in the Chart
    #[serde(deserialize_with = "deserialize_chart")]
    pub playlists: Vec<ChartPlaylist>,
}

impl Chart {

    pub fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns the `Chart` for a specified genre.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get() -> Self {

        // Get the 'reqwest' import
        use ::reqwest;

        // Get the chart api
        let chart_api = get_chart_api();

        // Get the json for the chart
        let mut resp = reqwest::get(&chart_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Shortened version of [`Track`].
/// Use [`.get_full()`] for the corresponding [`Track`] struct.
///
/// [`Track`]: Track
/// [`.get_full()`]: struct.ChartTrack.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartTrack {

    /// `The track's Deezer id`
    pub id: u32,

    /// `The track's full title`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track's version`
    pub title_version: String,

    /// `The url of the track on Deezer`
    pub link: String,

    /// `The track's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u32,

    /// `The track's Deezer rank`
    pub rank: u32,

    /// `Whether the track contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `The url of track's preview file. This file contains the first 30 seconds of the track`
    #[serde(default)]
    pub preview_url: Option<String>,

    /// `The position of the track in the charts`
    pub position: u32,

    /// `Returns an ChartTrackArtist object of the artist this track belongs to`
    pub artist: ChartTrackArtist,

    /// `Returns an ChartTrackAlbum object of the album this track belongs to`
    pub album: ChartTrackAlbum,
}

impl ChartTrack {

    /// Returns the corresponding [`Track`](Track) with all the information available.
    pub fn get_full(&self) -> Track {
        Track::get(self.id)
    }
}

/// Shortened version of [`Artist`].
/// Use [`.get_full()`] for the corresponding [`Artist`] struct.
///
/// [`Artist`]: Artist
/// [`.get_full()`]: struct.ChartTrackArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartTrackArtist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The url of the artist picture`
    pub picture: String,

    /// `The url of the artist picture in size small`
    pub picture_small: String,

    /// `The url of the artist picture in size medium`
    pub picture_medium: String,

    /// `The url of the artist picture in size big`
    pub picture_big: String,

    /// `The url of the artist picture in size xl`
    pub picture_xl: String,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,
}

impl ChartTrackArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}


/// Shortened version of [`Album`].
/// Use [`.get_full()`] for the corresponding [`Album`] struct.
///
/// [`Album`]: Album
/// [`.get_full()`]: struct.ChartTrackAlbum.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartTrackAlbum {

    /// `The Deezer album id`
    pub id: u32,

    /// `The album title`
    pub title: String,

    /// `The url of the album's cover.`
    pub cover: String,

    /// `The url of the album's cover in size small.`
    pub cover_small: String,

    /// `The url of the album's cover in size medium.`
    pub cover_medium: String,

    /// `The url of the album's cover in size big.`
    pub cover_big: String,

    /// `The url of the album's cover in size xl.`
    pub cover_xl: String,
}

impl ChartTrackAlbum {

    /// Returns the corresponding [`Album`](Album) with all the information available.
    pub fn get_full(&self) -> Album {
        Album::get(self.id)
    }
}

/// Shortened version of [`Album`].
/// Use [`.get_full()`] for the corresponding [`Album`] struct.
///
/// [`Album`]: Album
/// [`.get_full()`]: struct.ChartAlbum.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartAlbum {

    /// `The Deezer album id`
    pub id: u32,

    /// `The album title`
    pub title: String,

    /// `The url of the album's cover.`
    pub cover: String,

    /// `The url of the album's cover in size small.`
    pub cover_small: String,

    /// `The url of the album's cover in size medium.`
    pub cover_medium: String,

    /// `The url of the album's cover in size big.`
    pub cover_big: String,

    /// `The url of the album's cover in size xl.`
    pub cover_xl: String,

    /// `The record type of the album (EP / ALBUM / etc..)`
    pub record_type: String,

    /// `Whether the album contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `The position of the album in the charts`
    pub position: u32,

    /// `Returns an ChartAlbumArtist object of the artist this album belongs to`
    pub artist: ChartAlbumArtist,
}

impl ChartAlbum {

    /// Returns the corresponding [`Album`](Album) with all the information available.
    pub fn get_full(&self) -> Album {
        Album::get(self.id)
    }
}

/// Shortened version of [`Artist`].
/// Use [`.get_full()`] for the corresponding [`Artist`] struct.
///
/// [`Artist`]: Artist
/// [`.get_full()`]: struct.ChartAlbumArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartAlbumArtist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The url of the artist picture.`
    pub picture: String,

    /// `The url of the artist picture in size small`
    pub picture_small: String,

    /// `The url of the artist picture in size medium`
    pub picture_medium: String,

    /// `The url of the artist picture in size big`
    pub picture_big: String,

    /// `The url of the artist picture in size xl`
    pub picture_xl: String,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,
}

impl ChartAlbumArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}

/// Shortened version of [`Artist`].
/// Use [`.get_full()`] for the corresponding [`Artist`] struct.
///
/// [`Artist`]: Artist
/// [`.get_full()`]: struct.ChartArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartArtist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The url of the artist picture.`
    pub picture: String,

    /// `The url of the artist picture in size small`
    pub picture_small: String,

    /// `The url of the artist picture in size medium`
    pub picture_medium: String,

    /// `The url of the artist picture in size big`
    pub picture_big: String,

    /// `The url of the artist picture in size xl`
    pub picture_xl: String,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `The position of the artist in the charts`
    pub position: u32,
}

impl ChartArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}

/// Shortened version of [`Playlist`].
/// Use [`.get_full()`] for the corresponding [`Playlist`] struct.
///
/// [`Playlist`]: Playlist
/// [`.get_full()`]: struct.ChartPlaylist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartPlaylist {

    /// The playlist's Deezer id
    pub id: u32,

    /// The playlist's title
    pub title: String,

    /// If the playlist is public or not
    #[serde(rename = "public")]
    pub is_public: bool,

    /// The url of the playlist on Deezer
    pub link: String,

    /// The url of the playlist's cover
    pub picture: String,

    /// The url of the playlist's cover in size small
    pub picture_small: String,

    /// The url of the playlist's cover in size medium
    pub picture_medium: String,

    /// The url of the playlist's cover in size big
    pub picture_big: String,

    /// The url of the playlist's cover in size xl
    pub picture_xl: String,

    /// The position of the playlist in the charts
    #[serde(default)]
    pub position: u32,

    /// User object containing : id, name
    pub user: ChartPlaylistUser,
}

impl ChartPlaylist {

    /// Returns the corresponding [`Playlist`](Playlist) with all the information available.
    pub fn get_full(&self) -> Playlist {
        Playlist::get(self.id)
    }
}

/// Shortened version of [`User`].
/// Use [`.get_full()`] for the corresponding [`User`] struct.
///
/// [`User`]: User
/// [`.get_full()`]: struct.ChartPlaylistUser.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartPlaylistUser {

    /// The user's Deezer ID
    pub id: u32,

    /// The user's Deezer nickname
    pub name: String,
}

impl ChartPlaylistUser {

    /// Returns the corresponding [`User`](User) with all the information available.
    pub fn get_full(&self) -> User {
        User::get(self.id)
    }
}

fn deserialize_chart<'der, T, D>(de: D) -> Result<Vec<T>, D::Error>
    where D: Deserializer<'der>, for<'de> T: Deserialize<'de>
{
    let helper: Value = Deserialize::deserialize(de)?;
    let mut return_value = Vec::<T>::new();

    for object in helper.get("data").unwrap().as_array().unwrap() {
        let object: T = serde_json::from_value(object.clone()).unwrap();
        return_value.push(object);
    }

    Ok(return_value)
}

/// Takes an id and produces the appropriate api url.
pub fn get_chart_api() -> String {

    // Construct the api url
    "https://api.deezer.com/chart".to_owned()
}