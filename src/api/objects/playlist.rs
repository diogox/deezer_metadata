//! Contains the [`Playlist`](Playlist) struct.
//! Also contains a few other helper structs.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

use api::objects::user::User;
use api::objects::album::Album;
use api::objects::artist::Artist;
use api::objects::track::Track;
use api::objects::deserialize_map;

/// Contains all the information provided for an Album.
///
/// # Examples
///
/// For single uses, you can get an album using the `Playlist` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::objects::playlist::Playlist;
/// # fn main() {
/// // Pass the playlist id into the 'get' method
/// let playlist = Playlist::get(908622995);
/// # assert_eq!(playlist.id, 908622995);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::api::objects::playlist::Playlist;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many albums as you want with the same Api Client
/// let playlist1 = deezer.get_playlist(908622995);
/// let playlist2 = deezer.get_playlist(1924111242);
/// let playlist3 = deezer.get_playlist(754725481);
/// # assert_eq!(playlist1.id, 908622995);
/// # assert_eq!(playlist2.id, 1924111242);
/// # assert_eq!(playlist3.id, 754725481);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {

    /// The playlist's Deezer id
    pub id: u32,

    /// The playlist's title
    pub title: String,

    /// The playlist description
    pub description: String,

    /// The playlist's duration in seconds
    #[serde(rename = "duration")]
    pub duration_in_seconds: u32,

    /// If the playlist is public or not
    #[serde(rename = "public")]
    pub is_public: bool,

    /// If the playlist is the love tracks playlist
    pub is_loved_track: bool,

    /// If the playlist is collaborative or not
    #[serde(rename = "collaborative")]
    pub is_collaborative: bool,

    /// The playlist's rate
    #[serde(default)]
    pub rating: Option<u32>,

    /// Number of tracks in the playlist
    pub nb_tracks: u32,

    /// Number of tracks not seen
    #[serde(default)]
    pub unseen_track_count: Option<u32>,

    /// The number of playlist's fans
    pub fans: u32,

    /// The url of the playlist on Deezer
    pub link: String,

    /// The share link of the playlist on Deezer
    #[serde(rename = "share")]
    pub share_link: String,

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

    /// The checksum for the track list
    pub checksum: String,

    /// User object containing : id, name
    pub creator: PlaylistUser,

    /// Vector of Track object
    #[serde(deserialize_with = "deserialize_map")]
    pub tracks: Vec<PlaylistTrack>,
}

impl Playlist {

    pub(crate) fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns a `Playlist` from a playlist id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {
        use ::reqwest;

        // Get the track api
        let playlist_api = get_playlist_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&playlist_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Shortened version of [`User`].
/// Use [`.get_full()`] for the corresponding [`User`] struct.
///
/// [`User`]: User
/// [`.get_full()`]: struct.PlaylistUser.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistUser {

    /// The user's Deezer ID
    pub id: u32,

    /// The user's Deezer nickname
    pub name: String,
}

impl PlaylistUser {

    /// Returns the corresponding [`User`](User) with all the information available.
    pub fn get_full(&self) -> User {
        User::get(self.id)
    }
}

/// Shortened version of [`Track`].
/// Use [`.get_full()`] for the corresponding [`Track`] struct.
///
/// [`Track`]: Track
/// [`.get_full()`]: struct.PlaylistTrack.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTrack {

    /// `The track's Deezer id`
    pub id: u32,

    /// `True if the track is readable in the player for the current user`
    pub readable: bool,

    /// `The track's fulltitle`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track's version`
    pub title_version: String,

    /// `The track's unseen status`
    #[serde(default)]
    pub unseen: bool,

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
    pub preview_url: String,

    /// `The time when the track has been added to the playlist`
    #[serde(rename = "time_add")]
    pub added_on: u64,

    /// `Artist Object`
    pub artist: PlaylistTrackArtist,

    /// `Album Object`
    pub album: PlaylistTrackAlbum,
}

impl PlaylistTrack {

    /// Returns the corresponding [`Track`](Track) with all the information available.
    pub fn get_full(&self) -> Track {
        Track::get(self.id)
    }
}

/// Shortened version of [`Artist`].
/// Use [`.get_full()`] for the corresponding [`Artist`] struct.
///
/// [`Artist`]: Artist
/// [`.get_full()`]: struct.PlaylistTrackArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTrackArtist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,
}

impl PlaylistTrackArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}

/// Shortened version of [`Album`].
/// Use [`.get_full()`] for the corresponding [`Album`] struct.
///
/// [`Album`]: Album
/// [`.get_full()`]: struct.PlaylistTrackAlbum.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistTrackAlbum {

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

impl PlaylistTrackAlbum {

    /// Returns the corresponding [`Album`](Album) with all the information available.
    pub fn get_full(&self) -> Album {
        Album::get(self.id)
    }
}

/// Takes an id and produces the appropriate api url.
pub(crate) fn get_playlist_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/playlist/".to_owned() + &id.to_string()
}
