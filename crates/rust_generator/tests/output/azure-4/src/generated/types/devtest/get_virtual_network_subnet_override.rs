#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNetworkSubnetOverride {
    /// The name of the subnet.
    #[builder(into)]
    #[serde(rename = "labSubnetName")]
    pub r#lab_subnet_name: Box<String>,
    /// The resource identifier for the subnet.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
    /// Indicates if the subnet can be used for VM creation.  Possible values are `Allow`, `Default` and `Deny`.
    #[builder(into)]
    #[serde(rename = "useInVmCreationPermission")]
    pub r#use_in_vm_creation_permission: Box<String>,
    #[builder(into)]
    #[serde(rename = "usePublicIpAddressPermission")]
    pub r#use_public_ip_address_permission: Box<String>,
    /// The virtual network pool associated with this subnet.
    #[builder(into)]
    #[serde(rename = "virtualNetworkPoolName")]
    pub r#virtual_network_pool_name: Box<String>,
}
