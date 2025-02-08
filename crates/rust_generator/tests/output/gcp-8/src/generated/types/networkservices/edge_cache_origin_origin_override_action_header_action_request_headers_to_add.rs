#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EdgeCacheOriginOriginOverrideActionHeaderActionRequestHeadersToAdd {
    /// The name of the header to add.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<String>,
    /// The value of the header to add.
    #[builder(into)]
    #[serde(rename = "headerValue")]
    pub r#header_value: Box<String>,
    /// Whether to replace all existing headers with the same name.
    /// By default, added header values are appended
    /// to the response or request headers with the
    /// same field names. The added values are
    /// separated by commas.
    /// To overwrite existing values, set `replace` to `true`.
    #[builder(into, default)]
    #[serde(rename = "replace")]
    pub r#replace: Box<Option<bool>>,
}
