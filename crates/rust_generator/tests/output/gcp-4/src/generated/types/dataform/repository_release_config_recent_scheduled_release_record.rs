#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RepositoryReleaseConfigRecentScheduledReleaseRecord {
    /// (Output)
    /// The name of the created compilation result, if one was successfully created. Must be in the format projects/*/locations/*/repositories/*/compilationResults/*.
    #[builder(into, default)]
    #[serde(rename = "compilationResult")]
    pub r#compilation_result: Box<Option<String>>,
    /// (Output)
    /// The error status encountered upon this attempt to create the compilation result, if the attempt was unsuccessful.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorStatuses")]
    pub r#error_statuses: Box<Option<Vec<super::super::types::dataform::RepositoryReleaseConfigRecentScheduledReleaseRecordErrorStatus>>>,
    /// (Output)
    /// The timestamp of this release attempt.
    #[builder(into, default)]
    #[serde(rename = "releaseTime")]
    pub r#release_time: Box<Option<String>>,
}
