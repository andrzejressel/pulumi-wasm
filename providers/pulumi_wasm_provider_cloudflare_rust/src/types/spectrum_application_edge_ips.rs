#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SpectrumApplicationEdgeIps {
    /// The IP versions supported for inbound connections on Spectrum anycast IPs. Required when `type` is not `static`. Available values: `all`, `ipv4`, `ipv6`.
    #[builder(into, default)]
    #[serde(rename = "connectivity")]
    pub r#connectivity: Box<Option<String>>,
    /// The collection of customer owned IPs to broadcast via anycast for this hostname and application. Requires [Bring Your Own IP](https://developers.cloudflare.com/spectrum/getting-started/byoip/) provisioned.
    #[builder(into, default)]
    #[serde(rename = "ips")]
    pub r#ips: Box<Option<Vec<String>>>,
    /// The type of edge IP configuration specified. Available values: `dynamic`, `static`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
