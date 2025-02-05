#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateScaling {
    /// Maximum number of serving instances that this resource should have.
    #[builder(into, default)]
    #[serde(rename = "maxInstanceCount")]
    pub r#max_instance_count: Box<Option<i32>>,
    /// Minimum number of instances for the service, to be divided among all revisions receiving traffic.
    #[builder(into, default)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Box<Option<i32>>,
}
