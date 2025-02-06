#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerCapabilities {
    /// List of linux capabilities to add.
    #[builder(into, default)]
    #[serde(rename = "adds")]
    pub r#adds: Box<Option<Vec<String>>>,
    /// List of linux capabilities to drop.
    #[builder(into, default)]
    #[serde(rename = "drops")]
    pub r#drops: Box<Option<Vec<String>>>,
}
