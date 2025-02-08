#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterMaintenanceConfig {
    /// All IPv4 address from these ranges will be placed into maintenance mode.
    /// Nodes in maintenance mode will be cordoned and drained. When both of these
    /// are true, the "baremetal.cluster.gke.io/maintenance" annotation will be set
    /// on the node resource.
    #[builder(into)]
    #[serde(rename = "maintenanceAddressCidrBlocks")]
    pub r#maintenance_address_cidr_blocks: Box<Vec<String>>,
}
