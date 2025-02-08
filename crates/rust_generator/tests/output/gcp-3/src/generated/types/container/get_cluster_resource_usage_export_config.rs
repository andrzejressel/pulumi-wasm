#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterResourceUsageExportConfig {
    /// Parameters for using BigQuery as the destination of resource usage export.
    #[builder(into)]
    #[serde(rename = "bigqueryDestinations")]
    pub r#bigquery_destinations: Box<Vec<super::super::types::container::GetClusterResourceUsageExportConfigBigqueryDestination>>,
    /// Whether to enable network egress metering for this cluster. If enabled, a daemonset will be created in the cluster to meter network egress traffic.
    #[builder(into)]
    #[serde(rename = "enableNetworkEgressMetering")]
    pub r#enable_network_egress_metering: Box<bool>,
    /// Whether to enable resource consumption metering on this cluster. When enabled, a table will be created in the resource export BigQuery dataset to store resource consumption data. The resulting table can be joined with the resource usage table or with BigQuery billing export. Defaults to true.
    #[builder(into)]
    #[serde(rename = "enableResourceConsumptionMetering")]
    pub r#enable_resource_consumption_metering: Box<bool>,
}
