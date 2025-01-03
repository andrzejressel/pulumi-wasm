#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BlockchainNodesEthereumDetailsValidatorConfig {
    /// URLs for MEV-relay services to use for block building. When set, a managed MEV-boost service is configured on the beacon client.
    #[builder(into, default)]
    #[serde(rename = "mevRelayUrls")]
    pub r#mev_relay_urls: Box<Option<Vec<String>>>,
}
