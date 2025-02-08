#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSecretVersionsVersion {
    #[builder(into)]
    #[serde(rename = "createdTime")]
    pub r#created_time: Box<String>,
    /// Date that this version of the secret was last accessed.
    #[builder(into)]
    #[serde(rename = "lastAccessedDate")]
    pub r#last_accessed_date: Box<String>,
    /// Unique version identifier of this version of the secret.
    #[builder(into)]
    #[serde(rename = "versionId")]
    pub r#version_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "versionStages")]
    pub r#version_stages: Box<Vec<String>>,
}
