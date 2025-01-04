#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PeeringAttachmentOptions {
    /// Indicates whether dynamic routing is enabled or disabled.. Supports `enable` and `disable`.
    #[builder(into, default)]
    #[serde(rename = "dynamicRouting")]
    pub r#dynamic_routing: Box<Option<String>>,
}
