//! Contains the [`Album`](Album) struct.
//! Also contains a few other helper structs.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

use api::objects::deserialize_map;
use api::objects::artist::Artist;
use api::objects::track::Track;
use api::objects::genre::Genre;

/// Contains all the information provided for an Album.
///
/// # Examples
///
/// For single uses, you can get an album using the `Album` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::objects::album::Album;
/// # fn main() {
/// // Pass the album id into the 'get' method
/// let album = Album::get(302127);
/// # assert_eq!(album.id, 302127);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::api::objects::album::Album;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many albums as you want with the same Api Client
/// let album1 = deezer.get_album(302127);
/// let album2 = deezer.get_album(302128);
/// let album3 = deezer.get_album(302129);
/// # assert_eq!(album1.id, 302127);
/// # assert_eq!(album2.id, 302128);
/// # assert_eq!(album3.id, 302129);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Album {

    /// `The Deezer album id`
    pub id: u32,

    /// `The album title`
    pub title: String,

    /// `The album UPC`
    pub upc: String,

    /// `The url of the album on Deezer`
    pub link: String,

    /// `The share link of the album on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

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

    /// `The album's first genre id (You should use the genre list instead).`
    pub genre_id: Option<i32>,

    /// `List of genre object`
    #[serde(deserialize_with = "deserialize_map")]
    pub genres: Vec<AlbumGenre>,

    /// `The album's label name`
    pub label: String,

    /// `Number of tracks in the album`
    pub nb_tracks: u32,

    /// `The album's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u32,

    /// `The number of album's Fans`
    pub fans: u32,

    /// `The album's rate`
    pub rating: u32,

    /// `The album's release date`
    pub release_date: String,

    /// `The record type of the album (EP / ALBUM / etc..)`
    pub record_type: String,

    /// `Whether it's available right now`
    pub available: bool,

    /// `Return an alternative album object if the current album is not available`
    #[serde(default)]
    #[serde(rename = "alternative")]
    pub alternative_album: Option<Box<Album>>,

    /// `API Link to the tracklist of this album`
    #[serde(rename = "tracklist")]
    tracklist_api_url: String,

    /// `Whether the album contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `Return a list of contributors on the album`
    pub contributors: Vec<ContributorArtist>,

    // Create a get_full() function for this, that runs the artist api with the provided id
    // to get the full version
    /// `Returns an AlbumArtist object of the artist this album belongs to`
    pub artist: AlbumArtist,

    /// `list of Track objects that belong to this album`
    #[serde(deserialize_with = "deserialize_map")]
    pub tracks: Vec<AlbumTrack>,
}

impl Album {

    pub(crate) fn new(json: &str) -> Self {

        let mut album: Self = serde_json::from_str(&json).unwrap();

        // TODO: when 'new' and 'get' are made into a trait impl, add a local method call here so
        // any struct specific checks like what is here below can be done without overriding 'new'

        // If the value of genre_id is -1 make it a None
        if let Some(-1) = album.genre_id {
            album.genre_id = None;
        }

        album
    }

    /// Returns an `Album` from a album id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {

        // Get the track api
        let album_api = get_album_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&album_api).unwrap();
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
/// [`.get_full()`]: struct.AlbumArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct AlbumArtist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

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
}

impl AlbumArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}

/// Shortened version of [`Artist`].
/// Use [`.get_full()`] for the corresponding [`Artist`] struct.
///
/// [`Artist`]: Artist
/// [`.get_full()`]: struct.AlbumTrackArtist.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct AlbumTrackArtist {

    /// `Artist's Deezer Id`
    pub id: u32,

    /// `Artist's name`
    pub name: String,

    /// `Artist's Deezer tracklist`
    pub tracklist: String,
}

impl AlbumTrackArtist {

    /// Returns the corresponding [`Artist`](Artist) with all the information available.
    pub fn get_full(&self) -> Artist {
        Artist::get(self.id)
    }
}

/// Shortened version of [`Track`].
/// Use [`.get_full()`] for the corresponding [`Track`] struct.
///
/// [`Track`]: Track
/// [`.get_full()`]: struct.AlbumTrack.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct AlbumTrack {

    /// `The track's Deezer id`
    pub id: u32,

    /// `True if the track is readable in the player for the current user`
    pub readable: bool,

    /// `The track's full title`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track version`
    pub title_version: String,

    /// `The url of the track on Deezer`
    pub link: String,

    /// `The track's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u32,

    /// `The track's Deezer rank`
    pub rank: u32,

    /// `Whether the track contains explicit lyrics`
    pub explicit_lyrics: bool,

    /// `The url of track's preview file. This file contains the first 30 seconds of the track`
    pub preview: String,

    // Eliminar artist de AlbumTrack? é repetido uma vez que Album já tem artist..
    /// `AlbumTrackArtist object`
    pub artist: AlbumTrackArtist,
}

impl AlbumTrack {

    /// Returns the corresponding [`Track`](Track) with all the information available.
    pub fn get_full(&self) -> Track {
        Track::get(self.id)
    }
}

/// Shortened version of [`Genre`].
/// Use [`.get_full()`] for the corresponding [`Genre`] struct.
///
/// [`Genre`]: Genre
/// [`.get_full()`]: struct.AlbumGenre.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct AlbumGenre {

    /// `The Genre's id`
    pub id: u32,

    /// `The Genre's name`
    pub name: String,

    /// `The url of the genre picture`
    pub picture: String,
}

impl AlbumGenre {

    /// Returns the corresponding [`Genre`](Genre) with all the information available.
    pub fn get_full(&self) -> Genre {
        Genre::get(self.id)
    }
}

/// Takes an id and produces the appropriate api url.
pub(crate) fn get_album_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/album/".to_owned() + &id.to_string()
}