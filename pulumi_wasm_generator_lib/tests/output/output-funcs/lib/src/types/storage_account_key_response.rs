#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct StorageAccountKeyResponse {
    /// Creation time of the key, in round trip date format.
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Box<String>,
    /// Name of the key.
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: Box<String>,
    /// Permissions for the key -- read-only or full permissions.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<String>,
    /// Base 64-encoded value of the key.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
