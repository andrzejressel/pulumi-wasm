#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTriggerBuildSource {
    /// Location of the source in a Google Cloud Source Repository.
    #[builder(into)]
    #[serde(rename = "repoSources")]
    pub r#repo_sources: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildSourceRepoSource>>,
    /// Location of the source in an archive file in Google Cloud Storage.
    #[builder(into)]
    #[serde(rename = "storageSources")]
    pub r#storage_sources: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildSourceStorageSource>>,
}
