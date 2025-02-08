#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkspaceCustomParameters {
    /// The ID of a Azure Machine Learning workspace to link with Databricks workspace. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "machineLearningWorkspaceId")]
    pub r#machine_learning_workspace_id: Box<Option<String>>,
    /// Name of the NAT gateway for Secure Cluster Connectivity (No Public IP) workspace subnets (only for workspace with managed virtual network). Defaults to `nat-gateway`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "natGatewayName")]
    pub r#nat_gateway_name: Box<Option<String>>,
    /// Are public IP Addresses not allowed? Possible values are `true` or `false`. Defaults to `true`.
    /// 
    /// > **Note:** Updating `no_public_ip` parameter is only allowed if the value is changing from `false` to `true` and only for VNet-injected workspaces.
    /// 
    /// > **Note:** In `v3.104.0` and higher of the provider the `no_public_ip` parameter will now default to `true` instead of `false`.
    #[builder(into, default)]
    #[serde(rename = "noPublicIp")]
    pub r#no_public_ip: Box<Option<bool>>,
    /// The name of the Private Subnet within the Virtual Network. Required if `virtual_network_id` is set. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "privateSubnetName")]
    pub r#private_subnet_name: Box<Option<String>>,
    /// The resource ID of the `azure.network.SubnetNetworkSecurityGroupAssociation` resource which is referred to by the `private_subnet_name` field. This is the same as the ID of the subnet referred to by the `private_subnet_name` field. Required if `virtual_network_id` is set.
    #[builder(into, default)]
    #[serde(rename = "privateSubnetNetworkSecurityGroupAssociationId")]
    pub r#private_subnet_network_security_group_association_id: Box<Option<String>>,
    /// Name of the Public IP for No Public IP workspace with managed virtual network. Defaults to `nat-gw-public-ip`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "publicIpName")]
    pub r#public_ip_name: Box<Option<String>>,
    /// The name of the Public Subnet within the Virtual Network. Required if `virtual_network_id` is set. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "publicSubnetName")]
    pub r#public_subnet_name: Box<Option<String>>,
    /// The resource ID of the `azure.network.SubnetNetworkSecurityGroupAssociation` resource which is referred to by the `public_subnet_name` field. This is the same as the ID of the subnet referred to by the `public_subnet_name` field. Required if `virtual_network_id` is set.
    #[builder(into, default)]
    #[serde(rename = "publicSubnetNetworkSecurityGroupAssociationId")]
    pub r#public_subnet_network_security_group_association_id: Box<Option<String>>,
    /// Default Databricks File Storage account name. Defaults to a randomized name(e.g. `dbstoragel6mfeghoe5kxu`). Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "storageAccountName")]
    pub r#storage_account_name: Box<Option<String>>,
    /// Storage account SKU name. Possible values include `Standard_LRS`, `Standard_GRS`, `Standard_RAGRS`, `Standard_GZRS`, `Standard_RAGZRS`, `Standard_ZRS`, `Premium_LRS` or `Premium_ZRS`. Defaults to `Standard_GRS`.
    #[builder(into, default)]
    #[serde(rename = "storageAccountSkuName")]
    pub r#storage_account_sku_name: Box<Option<String>>,
    /// The ID of a Virtual Network where this Databricks Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Box<Option<String>>,
    /// Address prefix for Managed virtual network. Defaults to `10.139`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Databricks requires that a network security group is associated with the `public` and `private` subnets when a `virtual_network_id` has been defined. Both `public` and `private` subnets must be delegated to `Microsoft.Databricks/workspaces`. For more information about subnet delegation see the [product documentation](https://docs.microsoft.com/azure/virtual-network/subnet-delegation-overview).
    #[builder(into, default)]
    #[serde(rename = "vnetAddressPrefix")]
    pub r#vnet_address_prefix: Box<Option<String>>,
}
