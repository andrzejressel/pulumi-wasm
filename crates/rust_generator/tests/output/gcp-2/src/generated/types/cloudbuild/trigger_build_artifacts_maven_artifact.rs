#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerBuildArtifactsMavenArtifact {
    /// Maven artifactId value used when uploading the artifact to Artifact Registry.
    #[builder(into, default)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: Box<Option<String>>,
    /// Maven groupId value used when uploading the artifact to Artifact Registry.
    #[builder(into, default)]
    #[serde(rename = "groupId")]
    pub r#group_id: Box<Option<String>>,
    /// Path to an artifact in the build's workspace to be uploaded to Artifact Registry. This can be either an absolute path, e.g. /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar or a relative path from /workspace, e.g. my-app/target/my-app-1.0.SNAPSHOT.jar.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Artifact Registry repository, in the form "https://$REGION-maven.pkg.dev/$PROJECT/$REPOSITORY"
    /// Artifact in the workspace specified by path will be uploaded to Artifact Registry with this location as a prefix.
    #[builder(into, default)]
    #[serde(rename = "repository")]
    pub r#repository: Box<Option<String>>,
    /// Maven version value used when uploading the artifact to Artifact Registry.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
