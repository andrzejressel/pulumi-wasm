#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExperimentStep {
    /// One or more `branch` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "branches")]
    pub r#branches: Box<Vec<super::super::types::chaosstudio::ExperimentStepBranch>>,
    /// The name of the Step.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
