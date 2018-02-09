//! `deezloader_metadata` allows you to use deezloader's public API
//! to get their available information on tracks, artists, albums, ...

extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod api;