//! Contains the [`Artist`](Artist) struct.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

/// Contains all the information provided for an Artist.
///
/// # Examples
///
/// For single uses, you can get an artist using the `Artist` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::objects::artist::Artist;
/// # fn main() {
/// // Pass the artist id into the 'get' method
/// let artist = Artist::get(27);
/// # assert_eq(artist.id, 27);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::objects::artist::Artist;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many artists as you want with the same Api Client
/// let artist1 = deezer.get_artist(27);
/// let artist2 = deezer.get_artist(28);
/// let artist3 = deezer.get_artist(29);
/// # assert_eq(artist1.id, 27);
/// # assert_eq(artist2.id, 28);
/// # assert_eq(artist3.id, 29);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {

    /// `The artist's Deezer id`
    pub id: u32,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The share link of the artist on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

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

    /// `The number of artist's albums`
    pub nb_album: u32,

    /// `The number of artist's fans`
    pub nb_fan: u32,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `API Link to the top of this artist`
    pub tracklist: String,
}

impl Artist {

    pub(crate) fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns an `Artist` from an artist id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {
        use ::reqwest;
        
        // Get the track api
        let artist_api = get_artist_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&artist_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Takes an id and produces the appropriate api url.
pub(crate) fn get_artist_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/artist/".to_owned() + &id.to_string()
}