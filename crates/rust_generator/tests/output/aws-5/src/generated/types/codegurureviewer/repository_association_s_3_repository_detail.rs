#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryAssociationS3RepositoryDetail {
    /// The name of the S3 bucket used for associating a new S3 repository. Note: The name must begin with `codeguru-reviewer-`.
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "codeArtifacts")]
    pub r#code_artifacts: Box<Option<Vec<super::super::types::codegurureviewer::RepositoryAssociationS3RepositoryDetailCodeArtifact>>>,
}
