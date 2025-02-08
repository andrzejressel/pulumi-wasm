#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterEncryptionConfig {
    /// Configuration block with provider for encryption. Detailed below.
    #[builder(into)]
    #[serde(rename = "provider")]
    pub r#provider: Box<super::super::types::eks::ClusterEncryptionConfigProvider>,
    /// List of strings with resources to be encrypted. Valid values: `secrets`.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Vec<String>>,
}
