#[allow(dead_code)]

pub mod objects;

use reqwest::Client;

use self::objects::*;

pub struct Api {
    client: Client,
}

impl Api {

    pub fn new() -> Self {

        Api {
            client: Client::new(),
        }
    }
}

impl Api {

    /// Returns the [`Track`](Track) with the given id.
    pub fn get_track(&self, id: u32) -> track::Track {
        let json = self.client.get(&track::get_track_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        track::Track::new(&json)
    }

    /// Returns the [`Artist`](Artist) with the given id.
    pub fn get_artist(&self, id: u32) -> artist::Artist {
        let json = self.client.get(&artist::get_artist_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        artist::Artist::new(&json)
    }

    /// Returns the [`Album`](Album) with the given id.
    pub fn get_album(&self, id: u32) -> album::Album {
        let json = self.client.get(&album::get_album_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        album::Album::new(&json)
    }

    /// Returns the [`Genre`](Genre) with the given id.
    pub fn get_genre(&self, id: u32) -> genre::Genre {
        let json = self.client.get(&genre::get_genre_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        genre::Genre::new(&json)
    }

    /// Returns the [`Comment`](Comment) with the given id.
    pub fn get_comment(&self, id: u32) -> comment::Comment {
        let json = self.client.get(&comment::get_comment_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        comment::Comment::new(&json)
    }

    /// Returns the [`User`](User) with the given id.
    pub fn get_user(&self, id: u32) -> user::User {
        let json = self.client.get(&user::get_user_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        user::User::new(&json)
    }

    /// Returns the [`Playlist`](Playlist) with the given id.
    pub fn get_playlist(&self, id: u32) -> playlist::Playlist {
        let json = self.client.get(&playlist::get_playlist_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        playlist::Playlist::new(&json)
    }

    /// Returns the [`Editorial`](Editorial) with the given id.
    pub fn get_editorial(&self, id: u32) -> editorial::Editorial {
        let json = self.client.get(&editorial::get_editorial_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        editorial::Editorial::new(&json)
    }

    /// Returns the [`Radio`](Radio) with the given id.
    pub fn get_radio(&self, id: u32) -> radio::Radio {
        let json = self.client.get(&radio::get_radio_api(id))
            .send()
            .unwrap()
            .text()
            .unwrap();

        radio::Radio::new(&json)
    }

    /// Returns the [`Info`](Info) for the current country.
    pub fn get_info(&self) -> info::Info {
        let json = self.client.get(&info::get_info_api())
            .send()
            .unwrap()
            .text()
            .unwrap();

        info::Info::new(&json)
    }

    pub fn get_chart(&self) -> chart::Chart {
        let json = self.client.get(&chart::get_chart_api())
            .send()
            .unwrap()
            .text()
            .unwrap();

        chart::Chart::new(&json)
    }

    /// Returns the [`Options`](Options) for the current user.
    pub fn get_options(&self) -> options::Options {
        let json = self.client.get(&options::get_options_api())
            .send()
            .unwrap()
            .text()
            .unwrap();

        options::Options::new(&json)
    }
}