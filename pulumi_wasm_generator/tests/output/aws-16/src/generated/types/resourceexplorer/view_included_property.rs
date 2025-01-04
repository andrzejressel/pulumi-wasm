#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ViewIncludedProperty {
    /// The name of the property that is included in this view. Valid values: `tags`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
