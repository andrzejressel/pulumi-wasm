#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionBuildConfigSource {
    /// If provided, get the source from this location in a Cloud Source Repository.
    #[builder(into)]
    #[serde(rename = "repoSources")]
    pub r#repo_sources: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigSourceRepoSource>>,
    /// If provided, get the source from this location in Google Cloud Storage.
    #[builder(into)]
    #[serde(rename = "storageSources")]
    pub r#storage_sources: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigSourceStorageSource>>,
}
