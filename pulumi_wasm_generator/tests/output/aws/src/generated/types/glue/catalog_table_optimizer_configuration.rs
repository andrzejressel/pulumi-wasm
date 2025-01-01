#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogTableOptimizerConfiguration {
    /// Indicates whether the table optimizer is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The configuration block for an orphan file deletion optimizer. See Orphan File Deletion Configuration for additional details.
    #[builder(into, default)]
    #[serde(rename = "orphanFileDeletionConfiguration")]
    pub r#orphan_file_deletion_configuration: Box<Option<super::super::types::glue::CatalogTableOptimizerConfigurationOrphanFileDeletionConfiguration>>,
    /// The configuration block for a snapshot retention optimizer. See Retention Configuration for additional details.
    #[builder(into, default)]
    #[serde(rename = "retentionConfiguration")]
    pub r#retention_configuration: Box<Option<super::super::types::glue::CatalogTableOptimizerConfigurationRetentionConfiguration>>,
    /// The ARN of the IAM role to use for the table optimizer.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
