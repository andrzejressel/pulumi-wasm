#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct Type2 {
    #[builder(into, default)]
    #[serde(rename = "property2")]
    pub r#property_2: Box<Option<f64>>,
}
