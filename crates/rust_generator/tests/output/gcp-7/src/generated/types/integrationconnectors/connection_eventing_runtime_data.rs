#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionEventingRuntimeData {
    /// Events listener endpoint. The value will populated after provisioning the events listener.
    #[builder(into, default)]
    #[serde(rename = "eventsListenerEndpoint")]
    pub r#events_listener_endpoint: Box<Option<String>>,
    /// (Output)
    /// Current status of eventing.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<super::super::types::integrationconnectors::ConnectionEventingRuntimeDataStatus>>>,
}
