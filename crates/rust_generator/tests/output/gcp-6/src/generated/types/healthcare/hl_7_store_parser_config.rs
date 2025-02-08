#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct Hl7StoreParserConfig {
    /// Determines whether messages with no header are allowed.
    #[builder(into, default)]
    #[serde(rename = "allowNullHeader")]
    pub r#allow_null_header: Box<Option<bool>>,
    /// JSON encoded string for schemas used to parse messages in this
    /// store if schematized parsing is desired.
    #[builder(into, default)]
    #[serde(rename = "schema")]
    pub r#schema: Box<Option<String>>,
    /// Byte(s) to be used as the segment terminator. If this is unset, '\r' will be used as segment terminator.
    /// A base64-encoded string.
    #[builder(into, default)]
    #[serde(rename = "segmentTerminator")]
    pub r#segment_terminator: Box<Option<String>>,
    /// The version of the unschematized parser to be used when a custom `schema` is not set.
    /// Default value is `V1`.
    /// Possible values are: `V1`, `V2`, `V3`.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
