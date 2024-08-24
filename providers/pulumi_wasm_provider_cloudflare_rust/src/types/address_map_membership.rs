#[derive(serde::Serialize)]
pub struct AddressMapMembership {
    /// Controls whether the membership can be deleted via the API or not.
    #[serde(rename = "canDelete")]
    pub r#can_delete: Box<Option<bool>>,
    /// Identifier of the account or zone.
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    /// The type of the membership.
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
}
