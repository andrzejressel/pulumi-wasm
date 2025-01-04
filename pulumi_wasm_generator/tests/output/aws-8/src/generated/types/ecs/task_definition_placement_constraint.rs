#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskDefinitionPlacementConstraint {
    /// Cluster Query Language expression to apply to the constraint. For more information, see [Cluster Query Language in the Amazon EC2 Container Service Developer Guide](http://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html).
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Type of constraint. Use `memberOf` to restrict selection to a group of valid candidates. Note that `distinctInstance` is not supported in task definitions.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
