#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouterPeerCustomLearnedIpRange {
    /// The IP range to learn. The value must be a
    /// CIDR-formatted string.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<String>,
}
