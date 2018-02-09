//! Contains the [`Radio`](Radio) struct.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

/// Contains all the information provided for a Radio.
///
/// # Examples
///
/// For single uses, you can get a radio using the `Radio` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::objects::radio::Radio;
/// # fn main() {
/// // Pass the radio id into the 'get' method
/// let radio = Radio::get(6);
/// # assert_eq(radio.id, 6);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::objects::radio::Radio;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many albums as you want with the same Api Client
/// let radio1 = deezer.get_radio(6);
/// let radio2 = deezer.get_radio(7);
/// let radio3 = deezer.get_radio(10);
/// # assert_eq(radio1.id, 6);
/// # assert_eq(radio2.id, 7);
/// # assert_eq(radio3.id, 10);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Radio {

    /// The radio deezer ID
    pub id: u32,

    /// The radio title
    pub title: String,

    /// The radio description
    pub description: String,

    /// The share link of the radio on Deezer
    #[serde(rename = "share")]
    pub share_link: String,

    /// The url of the radio picture
    pub picture: String,

    /// The url of the radio picture in size small
    pub picture_small: String,

    /// The url of the radio picture in size medium
    pub picture_medium: String,

    /// The url of the radio picture in size big
    pub picture_big: String,

    /// The url of the radio picture in size xl
    pub picture_xl: String,

    /// API Link to the tracklist of this radio
    #[serde(rename = "tracklist")]
    pub track_list: String,
}

impl Radio {

    pub fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns a `Radio` from a radio id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {

        // Get the 'reqwest' import
        use ::reqwest;

        // Get the track api
        let radio_api = get_radio_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&radio_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Takes an id and produces the appropriate api url.
pub fn get_radio_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/radio/".to_owned() + &id.to_string()
}