#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplatePlacementManagedClusterConfigEncryptionConfig {
    /// The Cloud KMS key name to use for PD disk encryption for all instances in the cluster.
    #[builder(into, default)]
    #[serde(rename = "gcePdKmsKeyName")]
    pub r#gce_pd_kms_key_name: Box<Option<String>>,
}
