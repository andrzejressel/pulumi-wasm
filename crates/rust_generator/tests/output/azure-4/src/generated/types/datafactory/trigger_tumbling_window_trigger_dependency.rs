#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerTumblingWindowTriggerDependency {
    /// The offset of the dependency trigger. Must be in Timespan format (±hh:mm:ss) and must be a negative offset for a self dependency.
    #[builder(into, default)]
    #[serde(rename = "offset")]
    pub r#offset: Box<Option<String>>,
    /// The size of the dependency tumbling window. Must be in Timespan format (hh:mm:ss).
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<String>>,
    /// The dependency trigger name. If not specified, it will use self dependency.
    #[builder(into, default)]
    #[serde(rename = "triggerName")]
    pub r#trigger_name: Box<Option<String>>,
}
