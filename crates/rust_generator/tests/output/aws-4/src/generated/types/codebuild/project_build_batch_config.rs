#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProjectBuildBatchConfig {
    /// Specifies if the build artifacts for the batch build should be combined into a single artifact location.
    #[builder(into, default)]
    #[serde(rename = "combineArtifacts")]
    pub r#combine_artifacts: Box<Option<bool>>,
    /// Configuration block specifying the restrictions for the batch build. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "restrictions")]
    pub r#restrictions: Box<Option<super::super::types::codebuild::ProjectBuildBatchConfigRestrictions>>,
    /// Specifies the service role ARN for the batch build project.
    #[builder(into)]
    #[serde(rename = "serviceRole")]
    pub r#service_role: Box<String>,
    /// Specifies the maximum amount of time, in minutes, that the batch build must be completed in.
    #[builder(into, default)]
    #[serde(rename = "timeoutInMins")]
    pub r#timeout_in_mins: Box<Option<i32>>,
}
