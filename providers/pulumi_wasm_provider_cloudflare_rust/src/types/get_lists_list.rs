#[derive(serde::Serialize)]
pub struct GetListsList {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "numitems")]
    pub r#numitems: Box<Option<i32>>,
}
