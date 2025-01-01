#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PerInstanceConfigPreservedStateInternalIpIpAddress {
    /// The URL of the reservation for this IP address.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
}
