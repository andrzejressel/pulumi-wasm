#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSourceMappingProvisionedPollerConfig {
    /// The maximum number of event pollers this event source can scale up to. The range is between 1 and 2000.
    #[builder(into, default)]
    #[serde(rename = "maximumPollers")]
    pub r#maximum_pollers: Box<Option<i32>>,
    /// The minimum number of event pollers this event source can scale down to. The range is between 1 and 200.
    #[builder(into, default)]
    #[serde(rename = "minimumPollers")]
    pub r#minimum_pollers: Box<Option<i32>>,
}
