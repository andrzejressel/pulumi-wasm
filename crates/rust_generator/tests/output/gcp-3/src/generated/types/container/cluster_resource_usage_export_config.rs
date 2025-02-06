#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterResourceUsageExportConfig {
    /// Parameters for using BigQuery as the destination of resource usage export.
    /// 
    /// * `bigquery_destination.dataset_id` (Required) - The ID of a BigQuery Dataset. For Example:
    /// 
    #[builder(into)]
    #[serde(rename = "bigqueryDestination")]
    pub r#bigquery_destination: Box<super::super::types::container::ClusterResourceUsageExportConfigBigqueryDestination>,
    /// Whether to enable network egress metering for this cluster. If enabled, a daemonset will be created
    /// in the cluster to meter network egress traffic.
    #[builder(into, default)]
    #[serde(rename = "enableNetworkEgressMetering")]
    pub r#enable_network_egress_metering: Box<Option<bool>>,
    /// Whether to enable resource
    /// consumption metering on this cluster. When enabled, a table will be created in
    /// the resource export BigQuery dataset to store resource consumption data. The
    /// resulting table can be joined with the resource usage table or with BigQuery
    /// billing export. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enableResourceConsumptionMetering")]
    pub r#enable_resource_consumption_metering: Box<Option<bool>>,
}
