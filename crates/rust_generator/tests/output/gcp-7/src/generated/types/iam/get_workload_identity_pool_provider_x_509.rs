#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWorkloadIdentityPoolProviderX509 {
    /// A Trust store, use this trust store as a wrapper to config the trust
    /// anchor and optional intermediate cas to help build the trust chain for
    /// the incoming end entity certificate. Follow the x509 guidelines to
    /// define those PEM encoded certs. Only 1 trust store is currently
    /// supported.
    #[builder(into)]
    #[serde(rename = "trustStores")]
    pub r#trust_stores: Box<Vec<super::super::types::iam::GetWorkloadIdentityPoolProviderX509TrustStore>>,
}
