#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterGcpConfigAccessConfigNetworkConfig {
    /// Name of the VPC subnet from which the cluster is accessible. Both broker and bootstrap server IP addresses and DNS entries are automatically created in the subnet. The subnet must be located in the same region as the cluster. The project may differ. The name of the subnet must be in the format `projects/PROJECT_ID/regions/REGION/subnetworks/SUBNET`.
    #[builder(into)]
    #[serde(rename = "subnet")]
    pub r#subnet: Box<String>,
}
