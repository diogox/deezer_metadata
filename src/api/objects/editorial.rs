//! Contains the [`Editorial`](Editorial) struct.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

/// Contains all the information provided for an Editorial.
///
/// # Examples
///
/// For single uses, you can get an editorial using the `Editorial` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::objects::editorial::Editorial;
/// # fn main() {
/// // Pass the editorial id into the 'get' method
/// let editorial = Editorial::get(0);
/// # assert_eq!(editorial.id, 0);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::api::objects::editorial::Editorial;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many editorials as you want with the same Api Client
/// let editorial1 = deezer.get_editorial(0);
/// let editorial2 = deezer.get_editorial(132);
/// let editorial3 = deezer.get_editorial(152);
/// # assert_eq!(editorial1.id, 0);
/// # assert_eq!(editorial2.id, 132);
/// # assert_eq!(editorial3.id, 152);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Editorial {

    /// The editorial's Deezer id
    pub id: u32,

    /// The editorial's name
    pub name: String,

    /// The url of the editorial picture
    pub picture: String,

    /// The url of the editorial picture in size small
    pub picture_small: String,

    /// The url of the editorial picture in size medium
    pub picture_medium: String,

    /// The url of the editorial picture in size big
    pub picture_big: String,

    /// The url of the editorial picture in size xl
    pub picture_xl: String,
}

impl Editorial {

    pub fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns an `Editorial` from an editorial id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {

        // Get the 'reqwest' import
        use ::reqwest;

        // Get the track api
        let editorial_api = get_editorial_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&editorial_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }

    pub fn all() -> Vec<Self> {

        // TODO: implement in `Api` aswell
        // TODO: Change documentation for the struct after implementing
        unimplemented!();
    }
}

/// Takes an id and produces the appropriate api url.
pub fn get_editorial_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/editorial/".to_owned() + &id.to_string()
}