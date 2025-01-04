#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HubRoutingVpc {
    /// The URI of the VPC network.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
