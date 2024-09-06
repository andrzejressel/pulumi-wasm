#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetAccountRolesRole {
    /// Description of role's permissions.
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Role identifier tag.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Role Name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
