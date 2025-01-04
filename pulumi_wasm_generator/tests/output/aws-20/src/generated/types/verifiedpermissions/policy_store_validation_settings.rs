#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyStoreValidationSettings {
    /// The mode for the validation settings. Valid values: `OFF`, `STRICT`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
