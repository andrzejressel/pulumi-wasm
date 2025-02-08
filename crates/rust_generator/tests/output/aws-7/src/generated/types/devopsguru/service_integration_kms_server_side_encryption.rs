#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceIntegrationKmsServerSideEncryption {
    /// KMS key ID. This value can be a key ID, key ARN, alias name, or alias ARN.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Specifies whether KMS integration is enabled. Valid values are `DISABLED` and `ENABLED`.
    #[builder(into, default)]
    #[serde(rename = "optInStatus")]
    pub r#opt_in_status: Box<Option<String>>,
    /// Type of KMS key used. Valid values are `CUSTOMER_MANAGED_KEY` and `AWS_OWNED_KMS_KEY`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
