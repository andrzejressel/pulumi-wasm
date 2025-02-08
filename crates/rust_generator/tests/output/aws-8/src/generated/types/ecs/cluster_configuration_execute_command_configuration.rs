#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterConfigurationExecuteCommandConfiguration {
    /// AWS Key Management Service key ID to encrypt the data between the local client and the container.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Log configuration for the results of the execute command actions. Required when `logging` is `OVERRIDE`. See `log_configuration` Block for details.
    #[builder(into, default)]
    #[serde(rename = "logConfiguration")]
    pub r#log_configuration: Box<Option<super::super::types::ecs::ClusterConfigurationExecuteCommandConfigurationLogConfiguration>>,
    /// Log setting to use for redirecting logs for your execute command results. Valid values: `NONE`, `DEFAULT`, `OVERRIDE`.
    #[builder(into, default)]
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<String>>,
}
