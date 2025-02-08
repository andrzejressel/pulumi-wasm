#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustDeviceManagedNetworksConfig {
    /// The SHA-256 hash of the TLS certificate presented by the host found at tls_sockaddr. If absent, regular certificate verification (trusted roots, valid timestamp, etc) will be used to validate the certificate.
    #[builder(into)]
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<String>,
    /// A network address of the form "host:port" that the WARP client will use to detect the presence of a TLS host.
    #[builder(into)]
    #[serde(rename = "tlsSockaddr")]
    pub r#tls_sockaddr: Box<String>,
}
