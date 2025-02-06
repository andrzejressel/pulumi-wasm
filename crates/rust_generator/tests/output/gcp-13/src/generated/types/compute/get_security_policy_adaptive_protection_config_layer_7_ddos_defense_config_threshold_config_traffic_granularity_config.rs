#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfigTrafficGranularityConfig {
    /// If enabled, traffic matching each unique value for the specified type constitutes a separate traffic unit. It can only be set to true if value is empty.
    #[builder(into)]
    #[serde(rename = "enableEachUniqueValue")]
    pub r#enable_each_unique_value: Box<bool>,
    /// Type of this configuration.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Requests that match this value constitute a granular traffic unit.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
