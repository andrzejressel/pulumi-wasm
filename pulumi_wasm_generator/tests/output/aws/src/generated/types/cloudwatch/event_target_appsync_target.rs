#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventTargetAppsyncTarget {
    /// Contains the GraphQL mutation to be parsed and executed.
    #[builder(into, default)]
    #[serde(rename = "graphqlOperation")]
    pub r#graphql_operation: Box<Option<String>>,
}