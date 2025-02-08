#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfig {
    /// Cloud Run runtime configuration.
    #[builder(into, default)]
    #[serde(rename = "cloudRun")]
    pub r#cloud_run: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigCloudRun>>,
    /// Kubernetes runtime configuration.
    #[builder(into, default)]
    #[serde(rename = "kubernetes")]
    pub r#kubernetes: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetes>>,
}
