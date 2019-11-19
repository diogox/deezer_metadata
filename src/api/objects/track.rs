//! Contains the [`Track`](Track) struct.
//! Also contains a few other helper structs.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

use api::objects::artist::Artist;
use api::objects::album::Album;

/// Contains all the information provided for a Track.
///
/// # Examples
///
/// For single uses, you can get a track using the `Track` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::objects::track::Track;
/// # fn main() {
/// // Pass the track id into the 'get' method
/// let track = Track::get(912486);
/// # assert_eq!(track.id, 912486);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::api::objects::track::Track;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many tracks as you want with the same Api Client
/// let track1 = deezer.get_track(912486);
/// let track2 = deezer.get_track(912487);
/// let track3 = deezer.get_track(912488);
/// # assert_eq!(track1.id, 912486);
/// # assert_eq!(track2.id, 912487);
/// # assert_eq!(track3.id, 912488);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Track {

    /// `The track's Deezer id`
    pub id: u32,

    /// `True if the track is readable in the player for the current user`
    pub readable: bool,

    /// `The track's full title`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track's version`
    pub title_version: String,

    /// `The track's unseen status`
    #[serde(default)]
    pub unseen: Option<bool>,

    /// `The track's isrc`
    pub isrc: String,

    /// `The url of the track on Deezer`
    pub link: String,

    /// `The share link of the track on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

    /// `The track's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u32,

    /// `The position of the track in its album`
    #[serde(rename = "track_position")]
    pub track_position_in_album: u32,

    /// `The track's album's disk number`
    #[serde(rename = "disk_number")]
    pub album_disk_number: u32,

    /// `The track's Deezer rank`
    pub rank: u32,

    /// `The track's release date`
    pub release_date: String,

    /// `Whether the track contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `The url of track's preview file. This file contains the first 30 seconds of the track`
    #[serde(default)]
    pub preview_url: Option<String>,

    /// `Beats per minute`
    pub bpm: f32,

    /// `Signal strength`
    pub gain: f32,

    //TODO: Make countries an Enum?
    /// `List of countries where the track is available`
    pub available_countries: Vec<String>,

    /// `Return an alternative readable track if the current track is not readable`
    #[serde(rename = "alternative")]
    #[serde(default)]
    pub alternative_track_id: Option<u32>,

    /// `Return a list of contributors on the track`
    pub contributors: Vec<ContributorArtist>,

    /// `Artist Object`
    pub artist: TrackArtist,

    /// `Album Object`
    pub album: TrackAlbum,
}

impl Track {

    pub(crate) fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns a `Track` from a track id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {

        // Get the 'reqwest' import
        use ::reqwest;
        
        // Get the track api
        let track_api = get_track_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&track_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Shortened version of [`Artist`].
/// Use [`.get_full()`] for the corresponding [`Artist`] struct.
///
/// [`Artist`]: Artist
/// [`.get_full()`]: struct.ContributorArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct ContributorArtist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The share link of the artist on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

    /// `The url of the artist picture in size small.`
    pub picture_small: String,

    /// `The url of the artist picture in size medium.`
    pub picture_medium: String,

    /// `The url of the artist picture in size big.`
    pub picture_big: String,

    /// `The url of the artist picture in size xl.`
    pub picture_xl: String,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `API Link to the top of this artist`
    pub tracklist: String,
}

impl ContributorArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}

/// Shortened version of [`Artist`].
/// Use [`.get_full()`] for the corresponding [`Artist`] struct.
///
/// [`Artist`]: Artist
/// [`.get_full()`]: struct.TrackArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct TrackArtist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The share link of the artist on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

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

    /// `The number of artist's albums`
    #[serde(default)]
    pub nb_album: Option<u32>,

    /// `The number of artist's fans`
    #[serde(default)]
    pub nb_fan: Option<u32>,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `API Link to the top of this artist`
    pub tracklist: String,
}

impl TrackArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}

/// Shortened version of [`Album`].
/// Use [`.get_full()`] for the corresponding [`Album`] struct.
///
/// [`Album`]: Album
/// [`.get_full()`]: struct.TrackAlbum.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct TrackAlbum {

    /// `The Deezer album id`
    pub id: u32,

    /// `The album title`
    pub title: String,

    /// `The url of the album on Deezer`
    pub link: String,

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

    /// `The album's release date`
    pub release_date: String,
}

impl TrackAlbum {

    /// Returns the corresponding [`Album`](Album) with all the information available.
    pub fn get_full(&self) -> Album {
        Album::get(self.id)
    }
}

/// Takes an id and produces the appropriate api url.
pub fn get_track_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/track/".to_owned() + &id.to_string()
}