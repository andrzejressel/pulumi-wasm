#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MlTransformSchema {
    /// The type of data in the column.
    #[builder(into, default)]
    #[serde(rename = "dataType")]
    pub r#data_type: Box<Option<String>>,
    /// The name you assign to this ML Transform. It must be unique in your account.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
