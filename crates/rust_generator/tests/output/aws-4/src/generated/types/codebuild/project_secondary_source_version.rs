#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProjectSecondarySourceVersion {
    /// An identifier for a source in the build project.
    #[builder(into)]
    #[serde(rename = "sourceIdentifier")]
    pub r#source_identifier: Box<String>,
    /// The source version for the corresponding source identifier. See [AWS docs](https://docs.aws.amazon.com/codebuild/latest/APIReference/API_ProjectSourceVersion.html#CodeBuild-Type-ProjectSourceVersion-sourceVersion) for more details.
    #[builder(into)]
    #[serde(rename = "sourceVersion")]
    pub r#source_version: Box<String>,
}
