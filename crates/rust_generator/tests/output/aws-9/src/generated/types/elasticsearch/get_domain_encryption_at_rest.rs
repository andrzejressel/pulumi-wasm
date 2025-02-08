#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainEncryptionAtRest {
    /// Whether node to node encryption is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The KMS key id used to encrypt data at rest.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<String>,
}
