#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TriggerBuildSource {
    /// Location of the source in a Google Cloud Source Repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "repoSource")]
    pub r#repo_source: Box<Option<super::super::types::cloudbuild::TriggerBuildSourceRepoSource>>,
    /// Location of the source in an archive file in Google Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "storageSource")]
    pub r#storage_source: Box<Option<super::super::types::cloudbuild::TriggerBuildSourceStorageSource>>,
}
