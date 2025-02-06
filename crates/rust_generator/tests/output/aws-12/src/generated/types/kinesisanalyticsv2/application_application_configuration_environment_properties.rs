#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationEnvironmentProperties {
    /// Describes the execution property groups.
    #[builder(into)]
    #[serde(rename = "propertyGroups")]
    pub r#property_groups: Box<Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationEnvironmentPropertiesPropertyGroup>>,
}
