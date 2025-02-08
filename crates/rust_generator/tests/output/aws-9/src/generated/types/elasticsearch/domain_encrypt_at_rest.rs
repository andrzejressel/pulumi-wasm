#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainEncryptAtRest {
    /// Whether to enable encryption at rest. If the `encrypt_at_rest` block is not provided then this defaults to `false`. Enabling encryption on new domains requires `elasticsearch_version` 5.1 or greater.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// KMS key ARN to encrypt the Elasticsearch domain with. If not specified then it defaults to using the `aws/es` service KMS key. Note that KMS will accept a KMS key ID but will return the key ARN. To prevent the provider detecting unwanted changes, use the key ARN instead.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
}
