#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRegionalSecretVersionCustomerManagedEncryption {
    /// The resource name of the Cloud KMS CryptoKey used to encrypt secret payloads.
    #[builder(into)]
    #[serde(rename = "kmsKeyVersionName")]
    pub r#kms_key_version_name: Box<String>,
}
