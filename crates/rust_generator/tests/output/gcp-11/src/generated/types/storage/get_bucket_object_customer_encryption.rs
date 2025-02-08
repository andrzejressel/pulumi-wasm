#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBucketObjectCustomerEncryption {
    /// The encryption algorithm. Default: AES256
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Box<String>,
    /// Base64 encoded customer supplied encryption key.
    #[builder(into)]
    #[serde(rename = "encryptionKey")]
    pub r#encryption_key: Box<String>,
}
