#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceEventPublishConfig {
    /// Option to enable Event Publishing.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The resource name of the Pub/Sub topic. Format: projects/{projectId}/topics/{topic_id}
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Box<String>,
}
