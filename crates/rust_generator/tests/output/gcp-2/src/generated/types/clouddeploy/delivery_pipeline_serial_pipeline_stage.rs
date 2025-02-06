#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStage {
    /// Optional. The deploy parameters to use for the target in this stage.
    #[builder(into, default)]
    #[serde(rename = "deployParameters")]
    pub r#deploy_parameters: Box<Option<Vec<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageDeployParameter>>>,
    /// Skaffold profiles to use when rendering the manifest for this stage's `Target`.
    #[builder(into, default)]
    #[serde(rename = "profiles")]
    pub r#profiles: Box<Option<Vec<String>>>,
    /// Optional. The strategy to use for a `Rollout` to this stage.
    #[builder(into, default)]
    #[serde(rename = "strategy")]
    pub r#strategy: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategy>>,
    /// The target_id to which this stage points. This field refers exclusively to the last segment of a target name. For example, this field would just be `my-target` (rather than `projects/project/locations/location/targets/my-target`). The location of the `Target` is inferred to be the same as the location of the `DeliveryPipeline` that contains this `Stage`.
    #[builder(into, default)]
    #[serde(rename = "targetId")]
    pub r#target_id: Box<Option<String>>,
}
