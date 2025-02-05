#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExperimentTemplateTarget {
    /// Filter(s) for the target. Filters can be used to select resources based on specific attributes returned by the respective describe action of the resource type. For more information, see [Targets for AWS FIS](https://docs.aws.amazon.com/fis/latest/userguide/targets.html#target-filters). See below.
    #[builder(into, default)]
    #[serde(rename = "filters")]
    pub r#filters: Box<Option<Vec<super::super::types::fis::ExperimentTemplateTargetFilter>>>,
    /// Friendly name given to the target.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The resource type parameters.
    /// 
    /// > **NOTE:** The `target` configuration block requires either `resource_arns` or `resource_tag`.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Set of ARNs of the resources to target with an action. Conflicts with `resource_tag`.
    #[builder(into, default)]
    #[serde(rename = "resourceArns")]
    pub r#resource_arns: Box<Option<Vec<String>>>,
    /// Tag(s) the resources need to have to be considered a valid target for an action. Conflicts with `resource_arns`. See below.
    #[builder(into, default)]
    #[serde(rename = "resourceTags")]
    pub r#resource_tags: Box<Option<Vec<super::super::types::fis::ExperimentTemplateTargetResourceTag>>>,
    /// AWS resource type. The resource type must be supported for the specified action. To find out what resource types are supported, see [Targets for AWS FIS](https://docs.aws.amazon.com/fis/latest/userguide/targets.html#resource-types).
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
    /// Scopes the identified resources. Valid values are `ALL` (all identified resources), `COUNT(n)` (randomly select `n` of the identified resources), `PERCENT(n)` (randomly select `n` percent of the identified resources).
    #[builder(into)]
    #[serde(rename = "selectionMode")]
    pub r#selection_mode: Box<String>,
}
