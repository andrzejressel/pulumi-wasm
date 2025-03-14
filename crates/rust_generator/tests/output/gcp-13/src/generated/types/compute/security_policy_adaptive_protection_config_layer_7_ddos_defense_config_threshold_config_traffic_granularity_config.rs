#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfigTrafficGranularityConfig {
    /// If enabled, traffic matching each unique value for the specified type constitutes a separate traffic unit. It can only be set to true if value is empty.
    #[builder(into, default)]
    #[serde(rename = "enableEachUniqueValue")]
    pub r#enable_each_unique_value: Box<Option<bool>>,
    /// The type of this configuration, a granular traffic unit can be one of the following:
    /// * `HTTP_HEADER_HOST`
    /// * `HTTP_PATH`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Requests that match this value constitute a granular traffic unit.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
