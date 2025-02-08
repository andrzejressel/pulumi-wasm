#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualNetworkPanoramaDestinationNat {
    /// A `backend_config` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "backendConfig")]
    pub r#backend_config: Box<Option<super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaDestinationNatBackendConfig>>,
    /// A `frontend_config` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "frontendConfig")]
    pub r#frontend_config: Box<Option<super::super::types::paloalto::NextGenerationFirewallVirtualNetworkPanoramaDestinationNatFrontendConfig>>,
    /// The name which should be used for this Destination NAT.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Protocol for this Destination NAT configuration. Possible values include `TCP` and `UDP`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}
