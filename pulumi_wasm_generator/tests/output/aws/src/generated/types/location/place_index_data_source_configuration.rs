#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PlaceIndexDataSourceConfiguration {
    /// Specifies how the results of an operation will be stored by the caller. Valid values: `SingleUse`, `Storage`. Default: `SingleUse`.
    #[builder(into, default)]
    #[serde(rename = "intendedUse")]
    pub r#intended_use: Box<Option<String>>,
}