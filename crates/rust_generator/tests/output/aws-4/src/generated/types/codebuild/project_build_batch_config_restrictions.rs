#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProjectBuildBatchConfigRestrictions {
    /// An array of strings that specify the compute types that are allowed for the batch build. See [Build environment compute types](https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html) in the AWS CodeBuild User Guide for these values.
    #[builder(into, default)]
    #[serde(rename = "computeTypesAlloweds")]
    pub r#compute_types_alloweds: Box<Option<Vec<String>>>,
    /// Specifies the maximum number of builds allowed.
    #[builder(into, default)]
    #[serde(rename = "maximumBuildsAllowed")]
    pub r#maximum_builds_allowed: Box<Option<i32>>,
}
