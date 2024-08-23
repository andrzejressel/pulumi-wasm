#[derive(serde::Serialize)]
pub struct GetAccountRolesRole {
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
