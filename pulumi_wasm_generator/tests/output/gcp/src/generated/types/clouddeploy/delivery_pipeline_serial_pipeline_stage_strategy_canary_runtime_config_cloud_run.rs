#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigCloudRun {
    /// Whether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments.
    #[builder(into, default)]
    #[serde(rename = "automaticTrafficControl")]
    pub r#automatic_traffic_control: Box<Option<bool>>,
    /// Optional. A list of tags that are added to the canary revision while the canary phase is in progress.
    #[builder(into, default)]
    #[serde(rename = "canaryRevisionTags")]
    pub r#canary_revision_tags: Box<Option<Vec<String>>>,
    /// Optional. A list of tags that are added to the prior revision while the canary phase is in progress.
    #[builder(into, default)]
    #[serde(rename = "priorRevisionTags")]
    pub r#prior_revision_tags: Box<Option<Vec<String>>>,
    /// Optional. A list of tags that are added to the final stable revision when the stable phase is applied.
    #[builder(into, default)]
    #[serde(rename = "stableRevisionTags")]
    pub r#stable_revision_tags: Box<Option<Vec<String>>>,
}
