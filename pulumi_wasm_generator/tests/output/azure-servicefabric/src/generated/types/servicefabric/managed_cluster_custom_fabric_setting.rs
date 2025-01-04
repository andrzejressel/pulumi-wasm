#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedClusterCustomFabricSetting {
    /// Parameter name.
    #[builder(into)]
    #[serde(rename = "parameter")]
    pub r#parameter: Box<String>,
    /// Section name.
    #[builder(into)]
    #[serde(rename = "section")]
    pub r#section: Box<String>,
    /// Parameter value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
