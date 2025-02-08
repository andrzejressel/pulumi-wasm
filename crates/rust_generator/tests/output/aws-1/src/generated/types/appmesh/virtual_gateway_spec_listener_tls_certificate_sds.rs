#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecListenerTlsCertificateSds {
    /// Name of the secret for a virtual gateway's Transport Layer Security (TLS) Secret Discovery Service validation context trust.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
}
