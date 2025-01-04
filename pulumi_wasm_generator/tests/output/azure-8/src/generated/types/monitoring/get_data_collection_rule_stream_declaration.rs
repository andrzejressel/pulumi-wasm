#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCollectionRuleStreamDeclaration {
    /// One or more `column` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleStreamDeclarationColumn>>,
    /// The name of the custom stream. This name should be unique across all `stream_declaration` blocks.
    #[builder(into)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Box<String>,
}
