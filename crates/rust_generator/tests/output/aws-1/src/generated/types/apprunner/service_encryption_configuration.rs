#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceEncryptionConfiguration {
    /// ARN of the KMS key used for encryption.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<String>,
}
