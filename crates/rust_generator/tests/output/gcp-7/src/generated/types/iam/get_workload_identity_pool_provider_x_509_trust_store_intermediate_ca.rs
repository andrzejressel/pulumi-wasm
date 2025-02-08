#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWorkloadIdentityPoolProviderX509TrustStoreIntermediateCa {
    /// PEM certificate of the PKI used for validation. Must only contain one
    /// ca certificate(either root or intermediate cert).
    #[builder(into)]
    #[serde(rename = "pemCertificate")]
    pub r#pem_certificate: Box<String>,
}
