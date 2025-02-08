#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterDatabaseEncryption {
    /// The key to use to encrypt/decrypt secrets.
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: Box<String>,
    /// ENCRYPTED or DECRYPTED.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
