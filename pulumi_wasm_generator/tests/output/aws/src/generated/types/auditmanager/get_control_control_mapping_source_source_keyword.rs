#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetControlControlMappingSourceSourceKeyword {
    #[builder(into)]
    #[serde(rename = "keywordInputType")]
    pub r#keyword_input_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "keywordValue")]
    pub r#keyword_value: Box<String>,
}
