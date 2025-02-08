#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAiIndexMetadata {
    /// The configuration of the Matching Engine Index.
    #[builder(into)]
    #[serde(rename = "configs")]
    pub r#configs: Box<Vec<super::super::types::vertex::GetAiIndexMetadataConfig>>,
    /// Allows inserting, updating  or deleting the contents of the Matching Engine Index.
    /// The string must be a valid Cloud Storage directory path. If this
    /// field is set when calling IndexService.UpdateIndex, then no other
    /// Index field can be also updated as part of the same call.
    /// The expected structure and format of the files this URI points to is
    /// described at https://cloud.google.com/vertex-ai/docs/matching-engine/using-matching-engine#input-data-format
    #[builder(into)]
    #[serde(rename = "contentsDeltaUri")]
    pub r#contents_delta_uri: Box<String>,
    /// If this field is set together with contentsDeltaUri when calling IndexService.UpdateIndex,
    /// then existing content of the Index will be replaced by the data from the contentsDeltaUri.
    #[builder(into)]
    #[serde(rename = "isCompleteOverwrite")]
    pub r#is_complete_overwrite: Box<bool>,
}
