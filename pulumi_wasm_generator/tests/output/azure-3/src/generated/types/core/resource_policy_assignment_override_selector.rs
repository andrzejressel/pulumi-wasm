#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourcePolicyAssignmentOverrideSelector {
    #[builder(into, default)]
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    /// Specifies which characteristic will narrow down the set of evaluated resources. Possible values are `resourceLocation`, `resourceType` and `resourceWithoutLocation`.
    #[builder(into, default)]
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}
