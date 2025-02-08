#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamInputEventHubSerialization {
    /// The encoding of the incoming data in the case of input and the encoding of outgoing data in the case of output. It currently can only be set to `UTF8`.
    /// 
    /// > **NOTE:** This is required when `type` is set to `Csv` or `Json`.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// The delimiter that will be used to separate comma-separated value (CSV) records. Possible values are ` ` (space), `,` (comma), `	` (tab), `|` (pipe) and `;`.
    /// 
    /// > **NOTE:** This is required when `type` is set to `Csv`.
    #[builder(into, default)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Box<Option<String>>,
    /// The serialization format used for incoming data streams. Possible values are `Avro`, `Csv` and `Json`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
