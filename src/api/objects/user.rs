//! Contains the [`User`](User) struct.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

/// Contains all the information provided for a User.
///
/// # Examples
///
/// For single uses, you can get a user using the `User` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::objects::user::User;
/// # fn main() {
/// // Pass the user id into the 'get' method
/// let user = User::get(12);
/// # assert_eq!(user.id, 12);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::api::objects::user::User;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many albums as you want with the same Api Client
/// let user1 = deezer.get_user(12);
/// let user2 = deezer.get_user(13);
/// let user3 = deezer.get_user(14);
/// # assert_eq!(user1.id, 12);
/// # assert_eq!(user2.id, 13);
/// # assert_eq!(user3.id, 14);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct User {

    /// The user's Deezer ID
    pub id: u32,

    /// The user's Deezer nickname
    pub name: String,

    /// The user's last name
    #[serde(default)]
    #[serde(rename = "lastname")]
    pub last_name: String,

    /// The user's first name
    #[serde(default)]
    #[serde(rename = "firstname")]
    pub first_name: String,

    /// The user's email
    #[serde(default)]
    pub email: String,

    /// The user's status
    #[serde(default)]
    pub status: u32,

    /// The user's birthday
    #[serde(default)]
    pub birthday: String,

    /// The user's inscription date
    #[serde(default)]
    pub inscription_date: String,

    /// The user's gender : F or M
    #[serde(default)]
    pub gender: String,

    /// The url of the profil for the user on Deezer
    pub link: String,

    /// The url of the user's profile picture.
    pub picture: String,

    /// The url of the user's profile picture in size small.
    pub picture_small: String,

    /// The url of the user's profile picture in size medium.
    pub picture_medium: String,

    /// The url of the user's profile picture in size big.
    pub picture_big: String,

    /// The url of the user's profile picture in size xl.
    pub picture_xl: String,

    /// The user's country
    pub country: String,

    /// The user's language
    #[serde(default)]
    pub lang: String,

    /// If the user is a kid or not
    #[serde(default)]
    pub is_kid: bool,

    /// API Link to the flow of this user
    #[serde(rename = "tracklist")]
    pub track_list: String,
}

impl User {

    pub(crate) fn new(json: &str) -> Self {
        use ::serde_json;

        serde_json::from_str(&json).unwrap()
    }

    /// Returns a `User` from a user id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {
        use ::reqwest;

        // Get the track api
        let user_api = get_user_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&user_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Takes an id and produces the appropriate api url.
pub(crate) fn get_user_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/user/".to_owned() + &id.to_string()
}