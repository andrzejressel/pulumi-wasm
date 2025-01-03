#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetScriptDagEdge {
    /// ID of the node at which the edge starts.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// ID of the node at which the edge ends.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// Target of the edge.
    #[builder(into, default)]
    #[serde(rename = "targetParameter")]
    pub r#target_parameter: Box<Option<String>>,
}
