#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterControlPlaneEndpointsConfigDnsEndpointConfig {
    /// Controls whether user traffic is allowed over this endpoint. Note that GCP-managed services may still use the endpoint even if this is false.
    #[builder(into)]
    #[serde(rename = "allowExternalTraffic")]
    pub r#allow_external_traffic: Box<bool>,
    /// The cluster's DNS endpoint.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<String>,
}
