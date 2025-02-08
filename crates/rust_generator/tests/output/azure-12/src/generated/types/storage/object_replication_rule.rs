#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ObjectReplicationRule {
    /// The time after which the Block Blobs created will be copies to the destination. Possible values are `OnlyNewObjects`, `Everything` and time in RFC3339 format: `2006-01-02T15:04:00Z`. Defaults to `OnlyNewObjects`.
    #[builder(into, default)]
    #[serde(rename = "copyBlobsCreatedAfter")]
    pub r#copy_blobs_created_after: Box<Option<String>>,
    /// The destination storage container name.
    #[builder(into)]
    #[serde(rename = "destinationContainerName")]
    pub r#destination_container_name: Box<String>,
    /// Specifies a list of filters prefixes, the blobs whose names begin with which will be replicated.
    #[builder(into, default)]
    #[serde(rename = "filterOutBlobsWithPrefixes")]
    pub r#filter_out_blobs_with_prefixes: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The source storage container name.
    #[builder(into)]
    #[serde(rename = "sourceContainerName")]
    pub r#source_container_name: Box<String>,
}
