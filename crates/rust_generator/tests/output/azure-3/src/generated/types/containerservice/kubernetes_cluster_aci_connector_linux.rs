#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KubernetesClusterAciConnectorLinux {
    /// A `connector_identity` block is exported. The exported attributes are defined below.
    #[builder(into, default)]
    #[serde(rename = "connectorIdentities")]
    pub r#connector_identities: Box<Option<Vec<super::super::types::containerservice::KubernetesClusterAciConnectorLinuxConnectorIdentity>>>,
    /// The subnet name for the virtual nodes to run.
    /// 
    /// > **Note:** At this time ACI Connectors are not supported in Azure China.
    /// 
    /// > **Note:** AKS will add a delegation to the subnet named here. To prevent further runs from failing you should make sure that the subnet you create for virtual nodes has a delegation, like so.
    /// 
    /// ```yaml
    /// resources:
    ///   virtual:
    ///     type: azure:network:Subnet
    ///     properties:
    ///       delegations:
    ///         - name: aciDelegation
    ///           serviceDelegation:
    ///             name: Microsoft.ContainerInstance/containerGroups
    ///             actions:
    ///               - Microsoft.Network/virtualNetworks/subnets/action
    /// ```
    #[builder(into)]
    #[serde(rename = "subnetName")]
    pub r#subnet_name: Box<String>,
}
