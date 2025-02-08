#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketReplicationConfigRuleSourceSelectionCriteriaSseKmsEncryptedObjects {
    /// Whether the existing objects should be replicated. Either `"Enabled"` or `"Disabled"`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
