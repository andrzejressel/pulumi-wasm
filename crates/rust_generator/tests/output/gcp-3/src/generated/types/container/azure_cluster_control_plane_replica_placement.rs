#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AzureClusterControlPlaneReplicaPlacement {
    /// For a given replica, the Azure availability zone where to provision the control plane VM and the ETCD disk.
    #[builder(into)]
    #[serde(rename = "azureAvailabilityZone")]
    pub r#azure_availability_zone: Box<String>,
    /// For a given replica, the ARM ID of the subnet where the control plane VM is deployed. Make sure it's a subnet under the virtual network in the cluster configuration.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
