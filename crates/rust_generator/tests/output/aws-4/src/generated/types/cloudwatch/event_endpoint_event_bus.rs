#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventEndpointEventBus {
    /// The ARN of the event bus the endpoint is associated with.
    #[builder(into)]
    #[serde(rename = "eventBusArn")]
    pub r#event_bus_arn: Box<String>,
}
