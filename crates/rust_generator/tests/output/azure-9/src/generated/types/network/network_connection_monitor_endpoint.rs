#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkConnectionMonitorEndpoint {
    /// The IP address or domain name of the Network Connection Monitor endpoint.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// The test coverage for the Network Connection Monitor endpoint. Possible values are `AboveAverage`, `Average`, `BelowAverage`, `Default`, `Full` and `Low`.
    #[builder(into, default)]
    #[serde(rename = "coverageLevel")]
    pub r#coverage_level: Box<Option<String>>,
    /// A list of IPv4/IPv6 subnet masks or IPv4/IPv6 IP addresses to be excluded to the Network Connection Monitor endpoint.
    #[builder(into, default)]
    #[serde(rename = "excludedIpAddresses")]
    pub r#excluded_ip_addresses: Box<Option<Vec<String>>>,
    /// A `filter` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::network::NetworkConnectionMonitorEndpointFilter>>,
    /// A list of IPv4/IPv6 subnet masks or IPv4/IPv6 IP addresses to be included to the Network Connection Monitor endpoint.
    #[builder(into, default)]
    #[serde(rename = "includedIpAddresses")]
    pub r#included_ip_addresses: Box<Option<Vec<String>>>,
    /// The name of the endpoint for the Network Connection Monitor .
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The resource ID which is used as the endpoint by the Network Connection Monitor.
    #[builder(into, default)]
    #[serde(rename = "targetResourceId")]
    pub r#target_resource_id: Box<Option<String>>,
    /// The endpoint type of the Network Connection Monitor. Possible values are `AzureArcVM`, `AzureSubnet`, `AzureVM`, `AzureVNet`, `ExternalAddress`, `MMAWorkspaceMachine` and `MMAWorkspaceNetwork`.
    #[builder(into, default)]
    #[serde(rename = "targetResourceType")]
    pub r#target_resource_type: Box<Option<String>>,
}
