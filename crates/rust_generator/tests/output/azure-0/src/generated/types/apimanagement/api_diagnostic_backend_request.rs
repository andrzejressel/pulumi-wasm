#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiDiagnosticBackendRequest {
    /// Number of payload bytes to log (up to 8192).
    #[builder(into, default)]
    #[serde(rename = "bodyBytes")]
    pub r#body_bytes: Box<Option<i32>>,
    /// A `data_masking` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dataMasking")]
    pub r#data_masking: Box<Option<super::super::types::apimanagement::ApiDiagnosticBackendRequestDataMasking>>,
    /// Specifies a list of headers to log.
    #[builder(into, default)]
    #[serde(rename = "headersToLogs")]
    pub r#headers_to_logs: Box<Option<Vec<String>>>,
}
