#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HadoopClusterRolesEdgeNodeHttpsEndpoint {
    /// A list of access modes for the application.
    #[builder(into, default)]
    #[serde(rename = "accessModes")]
    pub r#access_modes: Box<Option<Vec<String>>>,
    /// The destination port to connect to.
    #[builder(into, default)]
    #[serde(rename = "destinationPort")]
    pub r#destination_port: Box<Option<i32>>,
    /// The value indicates whether the gateway authentication is enabled or not.
    #[builder(into, default)]
    #[serde(rename = "disableGatewayAuth")]
    pub r#disable_gateway_auth: Box<Option<bool>>,
    /// The private ip address of the endpoint.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The application's subdomain suffix.
    #[builder(into, default)]
    #[serde(rename = "subDomainSuffix")]
    pub r#sub_domain_suffix: Box<Option<String>>,
}
