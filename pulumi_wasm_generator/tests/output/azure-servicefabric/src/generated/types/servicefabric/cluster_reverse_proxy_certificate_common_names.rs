#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterReverseProxyCertificateCommonNames {
    /// A `common_names` block as defined below.
    #[builder(into)]
    #[serde(rename = "commonNames")]
    pub r#common_names: Box<Vec<super::super::types::servicefabric::ClusterReverseProxyCertificateCommonNamesCommonName>>,
    /// The X509 Store where the Certificate Exists, such as `My`.
    #[builder(into)]
    #[serde(rename = "x509StoreName")]
    pub r#x_509_store_name: Box<String>,
}