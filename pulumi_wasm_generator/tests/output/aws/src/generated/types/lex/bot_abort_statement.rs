#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BotAbortStatement {
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Box<Vec<super::super::types::lex::BotAbortStatementMessage>>,
    #[builder(into, default)]
    #[serde(rename = "responseCard")]
    pub r#response_card: Box<Option<String>>,
}