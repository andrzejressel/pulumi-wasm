#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleEvaluationMode {
    /// The mode of an evaluation.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}