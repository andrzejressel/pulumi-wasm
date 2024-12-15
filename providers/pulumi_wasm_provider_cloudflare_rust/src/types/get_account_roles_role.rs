#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetAccountRolesRole {
    /// Description of role's permissions.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Role identifier tag.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Role Name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
