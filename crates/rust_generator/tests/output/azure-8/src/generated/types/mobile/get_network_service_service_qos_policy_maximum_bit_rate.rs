#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkServiceServiceQosPolicyMaximumBitRate {
    /// Downlink bit rate.
    #[builder(into)]
    #[serde(rename = "downlink")]
    pub r#downlink: Box<String>,
    /// Uplink bit rate.
    #[builder(into)]
    #[serde(rename = "uplink")]
    pub r#uplink: Box<String>,
}
