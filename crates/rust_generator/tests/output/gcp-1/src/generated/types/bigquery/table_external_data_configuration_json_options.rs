#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableExternalDataConfigurationJsonOptions {
    /// The character encoding of the data. The supported values are UTF-8, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
}
