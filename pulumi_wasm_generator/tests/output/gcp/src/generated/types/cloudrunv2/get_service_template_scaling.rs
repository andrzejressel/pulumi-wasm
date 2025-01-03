#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateScaling {
    /// Maximum number of serving instances that this resource should have.
    #[builder(into)]
    #[serde(rename = "maxInstanceCount")]
    pub r#max_instance_count: Box<i32>,
    /// Minimum number of serving instances that this resource should have.
    #[builder(into)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Box<i32>,
}
