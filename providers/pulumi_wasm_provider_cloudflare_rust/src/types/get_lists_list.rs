#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetListsList {
    /// List description.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// List identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// List kind.
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    /// The list name to target for the resource.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Number of items in list.
    #[serde(rename = "numitems")]
    pub r#numitems: Box<Option<i32>>,
}
