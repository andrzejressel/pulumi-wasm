#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEndpointConnectionNetworkInterface {
    /// The ID of the network interface associated with the private endpoint.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Specifies the Name of the private endpoint.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
