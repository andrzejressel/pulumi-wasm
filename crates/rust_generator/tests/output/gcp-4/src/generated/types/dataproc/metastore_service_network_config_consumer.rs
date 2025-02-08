#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MetastoreServiceNetworkConfigConsumer {
    /// (Output)
    /// The URI of the endpoint used to access the metastore service.
    #[builder(into, default)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: Box<Option<String>>,
    /// The subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint.
    /// It is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network.
    /// There must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:
    /// `projects/{projectNumber}/regions/{region_id}/subnetworks/{subnetwork_id}
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<String>,
}
