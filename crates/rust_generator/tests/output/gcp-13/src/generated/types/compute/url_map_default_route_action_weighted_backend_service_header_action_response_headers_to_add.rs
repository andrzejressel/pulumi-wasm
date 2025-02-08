#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UrlMapDefaultRouteActionWeightedBackendServiceHeaderActionResponseHeadersToAdd {
    /// The name of the header to add.
    #[builder(into, default)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<Option<String>>,
    /// The value of the header to add.
    #[builder(into, default)]
    #[serde(rename = "headerValue")]
    pub r#header_value: Box<Option<String>>,
    /// If false, headerValue is appended to any values that already exist for the header.
    /// If true, headerValue is set for the header, discarding any values that were set for that header.
    #[builder(into, default)]
    #[serde(rename = "replace")]
    pub r#replace: Box<Option<bool>>,
}
