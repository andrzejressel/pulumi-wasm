#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceParametersAmazonElasticsearch {
    /// The OpenSearch domain.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Box<String>,
}