#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetWorkloadIdentityPoolProviderX509TrustStore {
    /// Set of intermediate CA certificates used for building the trust chain to
    /// trust anchor.
    /// IMPORTANT: Intermediate CAs are only supported when configuring x509 federation.
    #[builder(into)]
    #[serde(rename = "intermediateCas")]
    pub r#intermediate_cas: Box<Vec<super::super::types::iam::GetWorkloadIdentityPoolProviderX509TrustStoreIntermediateCa>>,
    /// List of Trust Anchors to be used while performing validation
    /// against a given TrustStore. The incoming end entity's certificate
    /// must be chained up to one of the trust anchors here.
    #[builder(into)]
    #[serde(rename = "trustAnchors")]
    pub r#trust_anchors: Box<Vec<super::super::types::iam::GetWorkloadIdentityPoolProviderX509TrustStoreTrustAnchor>>,
}
