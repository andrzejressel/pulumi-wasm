#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketObjectCustomerEncryption {
    /// Encryption algorithm. Default: AES256
    #[builder(into, default)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Box<Option<String>>,
    /// Base64 encoded Customer-Supplied Encryption Key.
    #[builder(into)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Box<String>,
}
