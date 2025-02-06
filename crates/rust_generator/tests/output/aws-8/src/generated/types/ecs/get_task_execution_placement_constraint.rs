#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTaskExecutionPlacementConstraint {
    /// A cluster query language expression to apply to the constraint. The expression can have a maximum length of 2000 characters. You can't specify an expression if the constraint type is `distinctInstance`.
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// The type of constraint. Valid values are `distinctInstance` or `memberOf`. Use `distinctInstance` to ensure that each task in a particular group is running on a different container instance. Use `memberOf` to restrict the selection to a group of valid candidates.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
