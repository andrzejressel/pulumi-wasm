/// Manages a Replica Set for an Active Directory Domain Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   primary:
///     type: azure:core:ResourceGroup
///     properties:
///       name: aadds-primary-rg
///       location: West Europe
///   primaryVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: primary
///     properties:
///       name: aadds-primary-vnet
///       location: ${primary.location}
///       resourceGroupName: ${primary.name}
///       addressSpaces:
///         - 10.0.1.0/16
///   primarySubnet:
///     type: azure:network:Subnet
///     name: primary
///     properties:
///       name: aadds-primary-subnet
///       resourceGroupName: ${primary.name}
///       virtualNetworkName: ${primaryVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   primaryNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: primary
///     properties:
///       name: aadds-primary-nsg
///       location: ${primary.location}
///       resourceGroupName: ${primary.name}
///       securityRules:
///         - name: AllowSyncWithAzureAD
///           priority: 101
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '443'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowRD
///           priority: 201
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '3389'
///           sourceAddressPrefix: CorpNetSaw
///           destinationAddressPrefix: '*'
///         - name: AllowPSRemoting
///           priority: 301
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '5986'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowLDAPS
///           priority: 401
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '636'
///           sourceAddressPrefix: '*'
///           destinationAddressPrefix: '*'
///   primarySubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: primary
///     properties:
///       subnetId: ${primarySubnet.id}
///       networkSecurityGroupId: ${primaryNetworkSecurityGroup.id}
///   dcAdmins:
///     type: azuread:Group
///     name: dc_admins
///     properties:
///       displayName: aad-dc-administrators
///       securityEnabled: true
///   admin:
///     type: azuread:User
///     properties:
///       userPrincipalName: dc-admin@hashicorp-example.net
///       displayName: DC Administrator
///       password: Pa55w0Rd!!1
///   adminGroupMember:
///     type: azuread:GroupMember
///     name: admin
///     properties:
///       groupObjectId: ${dcAdmins.objectId}
///       memberObjectId: ${admin.objectId}
///   example:
///     type: azuread:ServicePrincipal
///     properties:
///       applicationId: 2565bd9d-da50-47d4-8b85-4c97f669dc36
///   aadds:
///     type: azure:core:ResourceGroup
///     properties:
///       name: aadds-rg
///       location: westeurope
///   exampleService:
///     type: azure:domainservices:Service
///     name: example
///     properties:
///       name: example-aadds
///       location: ${aadds.location}
///       resourceGroupName: ${aadds.name}
///       domainName: widgetslogin.net
///       sku: Enterprise
///       filteredSyncEnabled: false
///       initialReplicaSet:
///         location: ${primaryVirtualNetwork.location}
///         subnetId: ${primarySubnet.id}
///       notifications:
///         additionalRecipients:
///           - notifyA@example.net
///           - notifyB@example.org
///         notifyDcAdmins: true
///         notifyGlobalAdmins: true
///       security:
///         syncKerberosPasswords: true
///         syncNtlmPasswords: true
///         syncOnPremPasswords: true
///       tags:
///         Environment: prod
///     options:
///       dependsOn:
///         - ${example}
///         - ${primarySubnetNetworkSecurityGroupAssociation}
///   replica:
///     type: azure:core:ResourceGroup
///     properties:
///       name: aadds-replica-rg
///       location: North Europe
///   replicaVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: replica
///     properties:
///       name: aadds-replica-vnet
///       location: ${replica.location}
///       resourceGroupName: ${replica.name}
///       addressSpaces:
///         - 10.20.0.0/16
///   aaddsReplica:
///     type: azure:network:Subnet
///     name: aadds_replica
///     properties:
///       name: aadds-replica-subnet
///       resourceGroupName: ${replica.name}
///       virtualNetworkName: ${replicaVirtualNetwork.name}
///       addressPrefixes:
///         - 10.20.0.0/24
///   aaddsReplicaNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: aadds_replica
///     properties:
///       name: aadds-replica-nsg
///       location: ${replica.location}
///       resourceGroupName: ${replica.name}
///       securityRules:
///         - name: AllowSyncWithAzureAD
///           priority: 101
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '443'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowRD
///           priority: 201
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '3389'
///           sourceAddressPrefix: CorpNetSaw
///           destinationAddressPrefix: '*'
///         - name: AllowPSRemoting
///           priority: 301
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '5986'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowLDAPS
///           priority: 401
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '636'
///           sourceAddressPrefix: '*'
///           destinationAddressPrefix: '*'
///   replicaSubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: replica
///     properties:
///       subnetId: ${aaddsReplica.id}
///       networkSecurityGroupId: ${aaddsReplicaNetworkSecurityGroup.id}
///   primaryReplica:
///     type: azure:network:VirtualNetworkPeering
///     name: primary_replica
///     properties:
///       name: aadds-primary-replica
///       resourceGroupName: ${primaryVirtualNetwork.resourceGroupName}
///       virtualNetworkName: ${primaryVirtualNetwork.name}
///       remoteVirtualNetworkId: ${replicaVirtualNetwork.id}
///       allowForwardedTraffic: true
///       allowGatewayTransit: false
///       allowVirtualNetworkAccess: true
///       useRemoteGateways: false
///   replicaPrimary:
///     type: azure:network:VirtualNetworkPeering
///     name: replica_primary
///     properties:
///       name: aadds-replica-primary
///       resourceGroupName: ${replicaVirtualNetwork.resourceGroupName}
///       virtualNetworkName: ${replicaVirtualNetwork.name}
///       remoteVirtualNetworkId: ${primaryVirtualNetwork.id}
///       allowForwardedTraffic: true
///       allowGatewayTransit: false
///       allowVirtualNetworkAccess: true
///       useRemoteGateways: false
///   replicaVirtualNetworkDnsServers:
///     type: azure:network:VirtualNetworkDnsServers
///     name: replica
///     properties:
///       virtualNetworkId: ${replicaVirtualNetwork.id}
///       dnsServers: ${exampleService.initialReplicaSet.domainControllerIpAddresses}
///   replicaReplicaSet:
///     type: azure:domainservices:ReplicaSet
///     name: replica
///     properties:
///       domainServiceId: ${exampleService.id}
///       location: ${replica.location}
///       subnetId: ${aaddsReplica.id}
///     options:
///       dependsOn:
///         - ${replicaSubnetNetworkSecurityGroupAssociation}
///         - ${primaryReplica}
///         - ${replicaPrimary}
/// ```
///
/// ## Import
///
/// Domain Service Replica Sets can be imported using the resource ID of the parent Domain Service and the Replica Set ID, e.g.
///
/// ```sh
/// $ pulumi import azure:domainservices/replicaSet:ReplicaSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AAD/domainServices/instance1/replicaSets/00000000-0000-0000-0000-000000000000
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replica_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicaSetArgs {
        /// The ID of the Domain Service for which to create this Replica Set. Changing this forces a new resource to be created.
        #[builder(into)]
        pub domain_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure location where this Replica Set should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the subnet in which to place this Replica Set. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReplicaSetResult {
        /// A list of subnet IP addresses for the domain controllers in this Replica Set, typically two.
        pub domain_controller_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the Domain Service for which to create this Replica Set. Changing this forces a new resource to be created.
        pub domain_service_id: pulumi_gestalt_rust::Output<String>,
        /// The publicly routable IP address for the domain controllers in this Replica Set.
        pub external_access_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where this Replica Set should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The current service status for the replica set.
        pub service_status: pulumi_gestalt_rust::Output<String>,
        /// The ID of the subnet in which to place this Replica Set. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicaSetArgs,
    ) -> ReplicaSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_service_id_binding = args.domain_service_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:domainservices/replicaSet:ReplicaSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainServiceId".into(),
                    value: domain_service_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicaSetResult {
            domain_controller_ip_addresses: o.get_field("domainControllerIpAddresses"),
            domain_service_id: o.get_field("domainServiceId"),
            external_access_ip_address: o.get_field("externalAccessIpAddress"),
            location: o.get_field("location"),
            service_status: o.get_field("serviceStatus"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
