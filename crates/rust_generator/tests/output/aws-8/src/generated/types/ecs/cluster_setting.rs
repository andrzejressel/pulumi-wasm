#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterSetting {
    /// Name of the setting to manage. Valid values: `containerInsights`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Value to assign to the setting. Valid values: `enhanced`, `enabled`, `disabled`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
