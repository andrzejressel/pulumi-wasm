#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterConfidentialComputing {
    /// Should the SGX quote helper be enabled?
    #[builder(into)]
    #[serde(rename = "sgxQuoteHelperEnabled")]
    pub r#sgx_quote_helper_enabled: Box<bool>,
}