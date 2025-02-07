#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RestorePlanRestoreConfigTransformationRule {
    /// The description is a user specified string description
    /// of the transformation rule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A list of transformation rule actions to take against candidate
    /// resources. Actions are executed in order defined - this order
    /// matters, as they could potentially interfere with each other and
    /// the first operation could affect the outcome of the second operation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fieldActions")]
    pub r#field_actions: Box<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigTransformationRuleFieldAction>>,
    /// This field is used to specify a set of fields that should be used to
    /// determine which resources in backup should be acted upon by the
    /// supplied transformation rule actions, and this will ensure that only
    /// specific resources are affected by transformation rule actions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceFilter")]
    pub r#resource_filter: Box<Option<super::super::types::gkebackup::RestorePlanRestoreConfigTransformationRuleResourceFilter>>,
}
