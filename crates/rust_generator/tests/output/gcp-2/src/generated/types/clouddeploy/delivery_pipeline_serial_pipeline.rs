#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DeliveryPipelineSerialPipeline {
    /// Each stage specifies configuration for a `Target`. The ordering of this list defines the promotion flow.
    #[builder(into, default)]
    #[serde(rename = "stages")]
    pub r#stages: Box<Option<Vec<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStage>>>,
}
