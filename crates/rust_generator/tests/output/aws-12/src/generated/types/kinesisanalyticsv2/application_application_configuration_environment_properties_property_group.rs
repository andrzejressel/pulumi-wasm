#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationApplicationConfigurationEnvironmentPropertiesPropertyGroup {
    /// The key of the application execution property key-value map.
    #[builder(into)]
    #[serde(rename = "propertyGroupId")]
    pub r#property_group_id: Box<String>,
    /// Application execution property key-value map.
    #[builder(into)]
    #[serde(rename = "propertyMap")]
    pub r#property_map: Box<std::collections::HashMap<String, String>>,
}
