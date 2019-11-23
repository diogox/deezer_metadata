//! Contains the [`Comment`](Comment) struct.
//! Also contains a few other helper structs.
#[deny(warnings, missing_docs)]
#[allow(dead_code)]

use api::objects::user::User;

/// Contains all the information provided for a Comment.
///
/// # Examples
///
/// For single uses, you can get a comment using the `Comment` struct:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::objects::comment::Comment;
/// # fn main() {
/// // Pass the comment id into the 'get' method
/// let comment = Comment::get(4179157801);
/// # assert_eq!(comment.id, 4179157801);
/// # }
/// ```
///
/// Or, you can use the [`Api`](Api) struct for multiple requests:
///
/// ```rust
/// # extern crate deezer_metadata;
/// # use deezer_metadata::api::Api;
/// # use deezer_metadata::api::objects::comment::Comment;
/// # fn main() {
/// // Get a new Api Client
/// let deezer = Api::new();
///
/// // Get as many comments as you want with the same Api Client
/// let comment1 = deezer.get_comment(4179157801);
/// # assert_eq!(comment1.id, 4179157801);
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Comment {

    /// The comment's Deezer id
    pub id: u32,

    /// The content of the comment
    pub text: String,

    /// The date the comment was posted
    pub date: u32,

    /// Object the comment belongs to, containing: id, type.
    /// Type can be "artist", "album" or "playlist".
    object: CommentParent,

    /// User this comment belongs to
    pub author: CommentAuthor,
}

impl Comment {

    pub(crate) fn new(json: &str) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    /// Returns a `Comment` from a comment id.
    ///
    /// Doesn't use [`Api`](Api), better suited for single uses.
    ///
    /// If you need to make a lot of requests, use [`Api`](Api).
    pub fn get(id: u32) -> Self {

        // Get the track api
        let comment_api = get_comment_api(id);

        // Get the json for the track
        let mut resp = reqwest::get(&comment_api).unwrap();
        let json = resp.text().unwrap();

        Self::new(&json)
    }
}

/// Shortened version of [`User`].
/// Use [`.get_full()`] for the corresponding [`User`] struct.
///
/// [`User`]: User
/// [`.get_full()`]: struct.CommentAuthor.html#method.get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct CommentAuthor {

    /// The comment's Deezer id
    pub id: u32,

    /// The user's Deezer nickname
    pub name: String,

    /// The url of the profil for the user on Deezer
    pub link: String,

    /// The url of the user's profile picture
    pub picture: String,

    /// The url of the user's profile picture in size small
    pub picture_small: String,

    /// The url of the user's profile picture in size medium
    pub picture_medium: String,

    /// The url of the user's profile picture in size big
    pub picture_big: String,

    /// The url of the user's profile picture in size xl
    pub picture_xl: String,
}

impl CommentAuthor {

    /// Returns the corresponding [`User`](User) with all the information available.
    pub fn get_full(&self) -> User {
        User::get(self.id)
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct CommentParent {

    id: String,

    #[serde(rename = "type")]
    object_type: String,
}

/// Takes an id and produces the appropriate api url.
pub(crate) fn get_comment_api(id: u32) -> String {

    // Construct the api url with the given id
    "https://api.deezer.com/comment/".to_owned() + &id.to_string()
}