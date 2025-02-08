#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KubernetesClusterConfidentialComputing {
    /// Should the SGX quote helper be enabled?
    #[builder(into)]
    #[serde(rename = "sgxQuoteHelperEnabled")]
    pub r#sgx_quote_helper_enabled: Box<bool>,
}
