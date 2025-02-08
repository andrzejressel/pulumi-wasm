#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutomationRulePromoteReleaseRule {
    /// Optional. The starting phase of the rollout created by this operation. Default to the first phase.
    #[builder(into, default)]
    #[serde(rename = "destinationPhase")]
    pub r#destination_phase: Box<Option<String>>,
    /// Optional. The ID of the stage in the pipeline to which this `Release` is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine if the target is one of the stages in the promotion sequence defined in the pipeline. * "@next", the next target in the promotion sequence.
    #[builder(into, default)]
    #[serde(rename = "destinationTargetId")]
    pub r#destination_target_id: Box<Option<String>>,
    /// Required. ID of the rule. This id must be unique in the `Automation` resource to which this rule belongs. The format is `a-z{0,62}`.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Optional. How long the release need to be paused until being promoted to the next target.
    #[builder(into, default)]
    #[serde(rename = "wait")]
    pub r#wait: Box<Option<String>>,
}
