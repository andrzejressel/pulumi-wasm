#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SqlRoleDefinitionPermission {
    /// A list of data actions that are allowed for the Cosmos DB SQL Role Definition.
    #[builder(into)]
    #[serde(rename = "dataActions")]
    pub r#data_actions: Box<Vec<String>>,
}