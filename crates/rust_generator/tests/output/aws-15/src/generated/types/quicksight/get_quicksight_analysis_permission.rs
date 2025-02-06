#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetQuicksightAnalysisPermission {
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
}
