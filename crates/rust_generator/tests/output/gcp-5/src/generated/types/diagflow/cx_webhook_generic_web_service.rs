#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxWebhookGenericWebService {
    /// Specifies a list of allowed custom CA certificates (in DER format) for HTTPS verification.
    #[builder(into, default)]
    #[serde(rename = "allowedCaCerts")]
    pub r#allowed_ca_certs: Box<Option<Vec<String>>>,
    /// The HTTP request headers to send together with webhook requests.
    #[builder(into, default)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// Whether to use speech adaptation for speech recognition.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
