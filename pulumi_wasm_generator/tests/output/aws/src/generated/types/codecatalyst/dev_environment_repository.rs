#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DevEnvironmentRepository {
    /// The name of the branch in a source repository.
    /// 
    /// persistent storage (` persistent_storage`) supports the following:
    #[builder(into, default)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<Option<String>>,
    /// The name of the source repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
}