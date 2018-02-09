pub mod track;
pub mod artist;
pub mod album;
pub mod playlist;
pub mod chart;
pub mod comment;
pub mod editorial;
pub mod genre;
pub mod info;
pub mod options;
pub mod radio;
pub mod search;
pub mod user;


use serde_json;
use serde_json::Value;
use serde::{
    Deserialize,
    Deserializer,
};

// TODO: Handle errors
pub(crate) fn deserialize_map<'der, T, D>(de: D) -> Result<Vec<T>, D::Error>
    where D: Deserializer<'der>, for<'de> T: Deserialize<'de>
{
    let helper: Value = Deserialize::deserialize(de)?;
    let mut return_value = Vec::<T>::new();

    for object in helper.get("data").unwrap().as_array().unwrap() {
        match serde_json::from_value(object.clone()) {
            Ok(value) => return_value.push(value),
            Err(e) => println!("{}", e)
        }
    }

    Ok(return_value)
}
