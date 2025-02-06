#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetastoreServiceNetworkConfig {
    /// The consumer-side network configuration for the Dataproc Metastore instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "consumers")]
    pub r#consumers: Box<Vec<super::super::types::dataproc::MetastoreServiceNetworkConfigConsumer>>,
    /// Enables custom routes to be imported and exported for the Dataproc Metastore service's peered VPC network.
    #[builder(into, default)]
    #[serde(rename = "customRoutesEnabled")]
    pub r#custom_routes_enabled: Box<Option<bool>>,
}
