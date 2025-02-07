#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LifecyclePolicyPolicyDetailFilter {
    /// For age-based filters, this is the number of resources to keep on hand after the lifecycle DELETE action is applied. Impacted resources are only deleted if you have more than this number of resources. If you have fewer resources than this number, the impacted resource is not deleted.
    #[builder(into, default)]
    #[serde(rename = "retainAtLeast")]
    pub r#retain_at_least: Box<Option<i32>>,
    /// Filter resources based on either age or count. Valid values: `AGE` or `COUNT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Defines the unit of time that the lifecycle policy uses to determine impacted resources. This is required for age-based rules. Valid values: `DAYS`, `WEEKS`, `MONTHS` or `YEARS`.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
    /// The number of units for the time period or for the count. For example, a value of 6 might refer to six months or six AMIs.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
