#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryAssociationS3RepositoryDetailCodeArtifact {
    #[builder(into, default)]
    #[serde(rename = "buildArtifactsObjectKey")]
    pub r#build_artifacts_object_key: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "sourceCodeArtifactsObjectKey")]
    pub r#source_code_artifacts_object_key: Box<Option<String>>,
}
