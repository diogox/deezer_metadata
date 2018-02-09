//! Contains the [`Options`](Options) struct.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

/// Contains all the information provided for a user's Options.
///
/// # Examples
///
/// For single uses, you can get a user's options using the `Options` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::objects::options::Options;
/// # fn main() {
/// let options = Options::get();
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::objects::options::Options;
/// # use deezer_metadata::objects::artist::Artist;
/// # use deezer_metadata::objects::track::Track;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as much metadata as you want with the same Api Client
/// let options = deezer.get_options();
/// let artist = deezer.get_artist(27);
/// let track = deezer.get_track(912486);
/// # assert_eq(artist.id, 27);
/// # assert_eq(track.id, 912486);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Options {

    /// If the user can stream on the platform
    pub streaming: bool,

    /// the streaming duration of the user (in seconds?)
    pub streaming_duration: u32,

    /// The user can listen to the music in offline mode
    pub offline: bool,

    /// The HQ can be activated
    pub hq: bool,

    /// Displays ads
    pub ads_display: bool,

    /// Activates audio ads
    pub ads_audio: bool,

    /// If the user reached the limit of linked devices
    #[serde(rename = "too_many_devices")]
    pub has_too_many_devices: bool,

    /// If the user can subscribe to the service
    pub can_subscribe: bool,

    /// The limit of radio skips. 0 = no limit
    pub radio_skips: u32,

    /// Lossless is available
    pub lossless: bool,

    /// Allows to display the preview of the tracks
    pub preview: bool,

    /// Allows to stream the radio
    pub radio: bool
}

impl Options {

    pub fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns an `Options`.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get() -> Self {

        // Get the 'reqwest' import
        use ::reqwest;

        // Get the options api
        let options_api = get_options_api();

        // Get the json for the options
        let mut resp = reqwest::get(&options_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Takes an id and produces the appropriate api url.
pub fn get_options_api() -> String {

    // Construct the api url
    "https://api.deezer.com/options".to_owned()
}