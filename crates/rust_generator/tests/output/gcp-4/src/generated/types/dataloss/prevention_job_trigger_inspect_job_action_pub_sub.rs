#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerInspectJobActionPubSub {
    /// Cloud Pub/Sub topic to send notifications to.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Box<String>,
}
