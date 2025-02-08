#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetWorkloadIdentityPoolProviderX509TrustStoreTrustAnchor {
    /// PEM certificate of the PKI used for validation. Must only contain one
    /// ca certificate(either root or intermediate cert).
    #[builder(into)]
    #[serde(rename = "pemCertificate")]
    pub r#pem_certificate: Box<String>,
}
