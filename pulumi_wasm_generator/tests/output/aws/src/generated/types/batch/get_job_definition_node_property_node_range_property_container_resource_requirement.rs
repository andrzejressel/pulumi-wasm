#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerResourceRequirement {
    /// The type of resource to assign to a container. The supported resources include `GPU`, `MEMORY`, and `VCPU`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The quantity of the specified resource to reserve for the container.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
