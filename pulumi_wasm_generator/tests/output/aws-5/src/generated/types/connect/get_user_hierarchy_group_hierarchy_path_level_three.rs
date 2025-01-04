#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserHierarchyGroupHierarchyPathLevelThree {
    /// ARN of the hierarchy group.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The identifier of the hierarchy group.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Returns information on a specific hierarchy group by name
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
