//! Contains the [`Info`](Info) struct.
//! Also contains the helper struct [`Offer`](Offer).
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

use serde_json;
use serde_json::Value;
use serde::{
    Deserialize,
    Deserializer,
};

/// Contains all the information about the API in the current country.
///
/// # Examples
///
/// For single uses, you can get a genre using the `Info` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::objects::info::Info;
/// # fn main() {
/// let info = Info::get();
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::objects::info::Info;
/// # use deezer_metadata::objects::track::Track;
/// # use deezer_metadata::objects::album::Album;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Make as many Api requests as you want with the same Client
/// let info = deezer.get_info();
/// let track = deezer.get_track(912486);
/// let album = deezer.get_album(302127);
/// # assert_eq(track.id, 912486);
/// # assert_eq(album.id, 302127);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Info {

    /// The current country ISO code
    pub country_iso: String,

    /// The current country name
    pub country: String,

    /// Indicates if Deezer is open in the current country or not
    pub open: bool,

    /// An array of available offers in the current country
    #[serde(deserialize_with = "deserialize_offers")]
    pub offers: Vec<Offer>
}

/// Contains all the information provided for an Offer.
#[derive(Deserialize, Serialize, Debug)]
pub struct Offer {
    pub id: u32,
    pub name: String,
    pub amount: String,
    pub currency: String,
    pub displayed_amount: String,
    pub tc: String,
    pub tc_html: String,
    pub tc_txt: String,
    pub try_and_buy: u32,
}

impl Info {

    pub fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns `Info`.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get() -> Self {

        // Get the 'reqwest' import
        use ::reqwest;

        // Get the info api
        let info_api = get_info_api();

        // Get the json for the info
        let mut resp = reqwest::get(&info_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

fn deserialize_offers<'der, D>(de: D) -> Result<Vec<Offer>, D::Error>
    where D: Deserializer<'der>
{
    let helper: Value = Deserialize::deserialize(de)?;
    let mut return_value = Vec::<Offer>::new();

    for object in helper.as_array().unwrap() {
        let offer: Offer = serde_json::from_value(object.clone()).unwrap();
        return_value.push(offer);
    }

    Ok(return_value)
}

/// Takes an id and produces the appropriate api url.
pub fn get_info_api() -> String {

    // Construct the api url
    "https://api.deezer.com/infos".to_owned()
}