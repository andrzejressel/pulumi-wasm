#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MongoRoleDefinitionPrivilege {
    /// A list of actions that are allowed.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    /// A `resource` block as defined below.
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Box<super::super::types::cosmosdb::MongoRoleDefinitionPrivilegeResource>,
}