#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrameworkControlSet {
    /// Configuration block(s) for the controls within the control set. See `controls` Block below for details.
    #[builder(into, default)]
    #[serde(rename = "controls")]
    pub r#controls: Box<Option<Vec<super::super::types::auditmanager::FrameworkControlSetControl>>>,
    /// Unique identifier for the framework.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Name of the control set.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
