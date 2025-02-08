#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionBuildConfigSource {
    /// If provided, get the source from this location in a Cloud Source Repository.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "repoSource")]
    pub r#repo_source: Box<Option<super::super::types::cloudfunctionsv2::FunctionBuildConfigSourceRepoSource>>,
    /// If provided, get the source from this location in Google Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "storageSource")]
    pub r#storage_source: Box<Option<super::super::types::cloudfunctionsv2::FunctionBuildConfigSourceStorageSource>>,
}
