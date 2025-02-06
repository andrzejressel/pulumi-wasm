#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMetastoreServiceNetworkConfig {
    /// The consumer-side network configuration for the Dataproc Metastore instance.
    #[builder(into)]
    #[serde(rename = "consumers")]
    pub r#consumers: Box<Vec<super::super::types::dataproc::GetMetastoreServiceNetworkConfigConsumer>>,
    /// Enables custom routes to be imported and exported for the Dataproc Metastore service's peered VPC network.
    #[builder(into)]
    #[serde(rename = "customRoutesEnabled")]
    pub r#custom_routes_enabled: Box<bool>,
}
