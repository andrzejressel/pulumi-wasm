#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTopicIngestionDataSourceSettingCloudStorageTextFormat {
    /// The delimiter to use when using the 'text' format. Each line of text as
    /// specified by the delimiter will be set to the 'data' field of a Pub/Sub
    /// message. When unset, '\n' is used.
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<String>,
}
