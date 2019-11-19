//! Contains the [`Genre`](Genre) struct.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

/// Contains all the information provided for a Genre.
///
/// # Examples
///
/// For single uses, you can get a genre using the `Genre` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::objects::genre::Genre;
/// # fn main() {
/// // Pass the genre id into the 'get' method
/// let genre = Genre::get(0);
/// # assert_eq!(genre.id, 0);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::api::objects::genre::Genre;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many albums as you want with the same Api Client
/// let genre1 = deezer.get_genre(0);
/// let genre2 = deezer.get_genre(132);
/// let genre3 = deezer.get_genre(165);
/// # assert_eq!(genre1.id, 0);
/// # assert_eq!(genre2.id, 132);
/// # assert_eq!(genre3.id, 165);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {

    /// The editorial's Deezer id
    pub id: u32,

    /// The editorial's name
    pub name: String,

    /// The url of the genre picture.
    pub picture: String,

    /// The url of the genre picture in size small.
    pub picture_small: String,

    /// The url of the genre picture in size medium.
    pub picture_medium: String,

    /// The url of the genre picture in size big.
    pub picture_big: String,

    /// The url of the genre picture in size xl.
    pub picture_xl: String,
}

impl Genre {

    pub(crate) fn new(json: &str) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    /// Returns a `Genre` from a genre id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {

        // Get the track api
        let genre_api = get_genre_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&genre_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Takes an id and produces the appropriate api url.
pub(crate) fn get_genre_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/genre/".to_owned() + &id.to_string()
}