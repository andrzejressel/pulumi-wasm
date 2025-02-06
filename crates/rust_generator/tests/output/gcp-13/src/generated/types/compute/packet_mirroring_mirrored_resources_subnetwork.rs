#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PacketMirroringMirroredResourcesSubnetwork {
    /// The URL of the subnetwork where this rule should be active.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
