#[derive(serde::Serialize)]
pub struct AddressMapMembership {
    #[serde(rename = "canDelete")]
    pub r#can_delete: Box<Option<bool>>,
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
}
