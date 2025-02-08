#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointGroupPortOverride {
    /// The endpoint port that you want a listener port to be mapped to. This is the port on the endpoint, such as the Application Load Balancer or Amazon EC2 instance.
    #[builder(into)]
    #[serde(rename = "endpointPort")]
    pub r#endpoint_port: Box<i32>,
    /// The listener port that you want to map to a specific endpoint port. This is the port that user traffic arrives to the Global Accelerator on.
    #[builder(into)]
    #[serde(rename = "listenerPort")]
    pub r#listener_port: Box<i32>,
}
