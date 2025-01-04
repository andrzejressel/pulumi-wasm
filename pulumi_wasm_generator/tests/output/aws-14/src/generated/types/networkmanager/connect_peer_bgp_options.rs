#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectPeerBgpOptions {
    #[builder(into, default)]
    #[serde(rename = "peerAsn")]
    pub r#peer_asn: Box<Option<i32>>,
}
