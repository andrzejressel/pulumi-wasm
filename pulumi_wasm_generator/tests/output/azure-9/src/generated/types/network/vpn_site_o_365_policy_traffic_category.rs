#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnSiteO365PolicyTrafficCategory {
    /// Is allow endpoint enabled? The `Allow` endpoint is required for connectivity to specific O365 services and features, but are not as sensitive to network performance and latency as other endpoint types. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "allowEndpointEnabled")]
    pub r#allow_endpoint_enabled: Box<Option<bool>>,
    /// Is default endpoint enabled? The `Default` endpoint represents O365 services and dependencies that do not require any optimization, and can be treated by customer networks as normal Internet bound traffic. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "defaultEndpointEnabled")]
    pub r#default_endpoint_enabled: Box<Option<bool>>,
    /// Is optimize endpoint enabled? The `Optimize` endpoint is required for connectivity to every O365 service and represents the O365 scenario that is the most sensitive to network performance, latency, and availability. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "optimizeEndpointEnabled")]
    pub r#optimize_endpoint_enabled: Box<Option<bool>>,
}
