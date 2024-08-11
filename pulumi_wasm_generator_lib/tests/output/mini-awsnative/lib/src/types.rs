
#[derive(serde::Serialize)]
pub struct IgnoreTags {
    #[serde(rename = "keyPrefixes")]
    pub r#key_prefixes: Box<Option<Vec<String>>>,
    #[serde(rename = "keys")]
    pub r#keys: Box<Option<Vec<String>>>,
}


pub type Region = String;
