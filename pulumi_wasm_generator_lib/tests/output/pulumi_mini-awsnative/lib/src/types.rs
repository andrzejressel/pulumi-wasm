
#[derive(serde::Serialize)]
pub struct IgnoreTags {
    #[serde(rename = "keyPrefixes")]
    pub r#key_prefixes: Option<Vec<String>>,
    #[serde(rename = "keys")]
    pub r#keys: Option<Vec<String>>,
}


pub type Region = String;
