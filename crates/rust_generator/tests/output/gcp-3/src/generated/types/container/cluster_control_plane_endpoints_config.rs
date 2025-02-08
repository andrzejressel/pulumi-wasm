#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterControlPlaneEndpointsConfig {
    /// DNS endpoint configuration.
    #[builder(into, default)]
    #[serde(rename = "dnsEndpointConfig")]
    pub r#dns_endpoint_config: Box<Option<super::super::types::container::ClusterControlPlaneEndpointsConfigDnsEndpointConfig>>,
}
