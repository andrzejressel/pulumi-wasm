#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExperimentStepBranch {
    /// One or more `actions` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<super::super::types::chaosstudio::ExperimentStepBranchAction>>,
    /// The name of the branch.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
