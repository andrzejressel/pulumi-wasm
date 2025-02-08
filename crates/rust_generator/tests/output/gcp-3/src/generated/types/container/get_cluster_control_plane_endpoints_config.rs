#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterControlPlaneEndpointsConfig {
    /// DNS endpoint configuration.
    #[builder(into)]
    #[serde(rename = "dnsEndpointConfigs")]
    pub r#dns_endpoint_configs: Box<Vec<super::super::types::container::GetClusterControlPlaneEndpointsConfigDnsEndpointConfig>>,
}
