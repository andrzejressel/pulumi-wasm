#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnSiteLinkBgp {
    /// The BGP speaker's ASN.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: Box<i32>,
    /// The BGP peering IP address.
    #[builder(into)]
    #[serde(rename = "peeringAddress")]
    pub r#peering_address: Box<String>,
}
