#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppProductionBranch {
    /// Branch name for the production branch.
    #[builder(into, default)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<Option<String>>,
    /// Last deploy time of the production branch.
    #[builder(into, default)]
    #[serde(rename = "lastDeployTime")]
    pub r#last_deploy_time: Box<Option<String>>,
    /// Status of the production branch.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Thumbnail URL for the production branch.
    #[builder(into, default)]
    #[serde(rename = "thumbnailUrl")]
    pub r#thumbnail_url: Box<Option<String>>,
}
