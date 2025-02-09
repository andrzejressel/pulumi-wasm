/// Manages an Azure VMware Solution Private Cloud Netapp File Attachment.
///
/// ## Example Usage
///
/// > **NOTE :** For Azure Azure VMware Solution Private Cloud, normal `pulumi up` could ignore this note. Please disable correlation request id for continuous operations in one build (like acctest). The continuous operations like `update` or `delete` could not be triggered when it shares the same `correlation-id` with its previous operation.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let gatewaySubnet = subnet::create(
///         "gatewaySubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.6.1.0/24",])
///             .name("GatewaySubnet")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .virtual_network_name("${testVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let netappSubnet = subnet::create(
///         "netappSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.6.2.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("testdelegation")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/networkinterfaces/*",
///                     "Microsoft.Network/virtualNetworks/subnets/join/action",])
///                     .name("Microsoft.Netapp/volumes").build_struct()).build_struct(),
///                 ],
///             )
///             .name("example-Subnet")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .virtual_network_name("${testVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let test = public_ip::create(
///         "test",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-public-ip")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let testAccount = account::create(
///         "testAccount",
///         AccountArgs::builder()
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-NetAppAccount")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .build_struct(),
///     );
///     let testCluster = cluster::create(
///         "testCluster",
///         ClusterArgs::builder()
///             .cluster_node_count(3)
///             .name("example-vm-cluster")
///             .sku_name("av36")
///             .vmware_cloud_id("${testPrivateCloud.id}")
///             .build_struct(),
///     );
///     let testExpressRouteAuthorization = express_route_authorization::create(
///         "testExpressRouteAuthorization",
///         ExpressRouteAuthorizationArgs::builder()
///             .name("example-VmwareAuthorization")
///             .private_cloud_id("${testPrivateCloud.id}")
///             .build_struct(),
///     );
///     let testNetappVolumeAttachment = netapp_volume_attachment::create(
///         "testNetappVolumeAttachment",
///         NetappVolumeAttachmentArgs::builder()
///             .name("example-vmwareattachment")
///             .netapp_volume_id("${testVolume.id}")
///             .vmware_cluster_id("${testCluster.id}")
///             .build_struct(),
///     );
///     let testPool = pool::create(
///         "testPool",
///         PoolArgs::builder()
///             .account_name("${testAccount.name}")
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-NetAppPool")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .service_level("Standard")
///             .size_in_tb(4)
///             .build_struct(),
///     );
///     let testPrivateCloud = private_cloud::create(
///         "testPrivateCloud",
///         PrivateCloudArgs::builder()
///             .location("${testAzurermResourceGroup.location}")
///             .management_cluster(
///                 PrivateCloudManagementCluster::builder().size(3).build_struct(),
///             )
///             .name("example-PC")
///             .network_subnet_cidr("192.168.48.0/22")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .sku_name("av36")
///             .build_struct(),
///     );
///     let testVirtualNetwork = virtual_network::create(
///         "testVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.6.0.0/16",])
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-VirtualNetwork")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .build_struct(),
///     );
///     let testVirtualNetworkGateway = virtual_network_gateway::create(
///         "testVirtualNetworkGateway",
///         VirtualNetworkGatewayArgs::builder()
///             .ip_configurations(
///                 vec![
///                     VirtualNetworkGatewayIpConfiguration::builder()
///                     .name("vnetGatewayConfig").publicIpAddressId("${test.id}")
///                     .subnetId("${gatewaySubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-vnet-gateway")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .sku("Standard")
///             .type_("ExpressRoute")
///             .build_struct(),
///     );
///     let testVirtualNetworkGatewayConnection = virtual_network_gateway_connection::create(
///         "testVirtualNetworkGatewayConnection",
///         VirtualNetworkGatewayConnectionArgs::builder()
///             .authorization_key(
///                 "${testExpressRouteAuthorization.expressRouteAuthorizationKey}",
///             )
///             .express_route_circuit_id("${testPrivateCloud.circuits[0].expressRouteId}")
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-vnetgwconn")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .type_("ExpressRoute")
///             .virtual_network_gateway_id("${testVirtualNetworkGateway.id}")
///             .build_struct(),
///     );
///     let testVolume = volume::create(
///         "testVolume",
///         VolumeArgs::builder()
///             .account_name("${testAccount.name}")
///             .azure_vmware_data_store_enabled(true)
///             .export_policy_rules(
///                 vec![
///                     VolumeExportPolicyRule::builder().allowedClients(vec!["0.0.0.0/0",])
///                     .protocolsEnabled("NFSv3").rootAccessEnabled(true).ruleIndex(1)
///                     .unixReadOnly(false).unixReadWrite(true).build_struct(),
///                 ],
///             )
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-NetAppVolume")
///             .pool_name("${testPool.name}")
///             .protocols(vec!["NFSv3",])
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .service_level("Standard")
///             .storage_quota_in_gb(100)
///             .subnet_id("${netappSubnet.id}")
///             .volume_path("my-unique-file-path-%d")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure VMware Solution Private Cloud Netapp File Volume Attachments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:avs/netappVolumeAttachment:NetappVolumeAttachment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AVS/privateClouds/privateCloud1/clusters/Cluster1/dataStores/datastore1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod netapp_volume_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetappVolumeAttachmentArgs {
        /// The name which should be used for this Azure VMware Solution Private Cloud Netapp File Volume Attachment. Changing this forces a new Azure VMware Solution Private Cloud Netapp File Volume Attachment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The netapp file volume for this Azure VMware Solution Private Cloud Netapp File Volume Attachment to connect to. Changing this forces a new Azure VMware Solution Private Cloud Netapp File Volume Attachment to be created.
        #[builder(into)]
        pub netapp_volume_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The vmware cluster for this Azure VMware Solution Private Cloud Netapp File Volume Attachment to associated to. Changing this forces a new Azure VMware Solution Private Cloud Netapp File Volume Attachment to be created.
        ///
        /// > **NOTE :** please follow the prerequisites mentioned in this [article](https://learn.microsoft.com/en-us/azure/azure-vmware/attach-azure-netapp-files-to-azure-vmware-solution-hosts?tabs=azure-portal#prerequisites) before associating the netapp file volume to the Azure VMware Solution hosts.
        #[builder(into)]
        pub vmware_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetappVolumeAttachmentResult {
        /// The name which should be used for this Azure VMware Solution Private Cloud Netapp File Volume Attachment. Changing this forces a new Azure VMware Solution Private Cloud Netapp File Volume Attachment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The netapp file volume for this Azure VMware Solution Private Cloud Netapp File Volume Attachment to connect to. Changing this forces a new Azure VMware Solution Private Cloud Netapp File Volume Attachment to be created.
        pub netapp_volume_id: pulumi_gestalt_rust::Output<String>,
        /// The vmware cluster for this Azure VMware Solution Private Cloud Netapp File Volume Attachment to associated to. Changing this forces a new Azure VMware Solution Private Cloud Netapp File Volume Attachment to be created.
        ///
        /// > **NOTE :** please follow the prerequisites mentioned in this [article](https://learn.microsoft.com/en-us/azure/azure-vmware/attach-azure-netapp-files-to-azure-vmware-solution-hosts?tabs=azure-portal#prerequisites) before associating the netapp file volume to the Azure VMware Solution hosts.
        pub vmware_cluster_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetappVolumeAttachmentArgs,
    ) -> NetappVolumeAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let netapp_volume_id_binding_1 = args.netapp_volume_id.get_output(context);
        let netapp_volume_id_binding = netapp_volume_id_binding_1.get_inner();
        let vmware_cluster_id_binding_1 = args.vmware_cluster_id.get_output(context);
        let vmware_cluster_id_binding = vmware_cluster_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:avs/netappVolumeAttachment:NetappVolumeAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "netappVolumeId".into(),
                    value: &netapp_volume_id_binding,
                },
                register_interface::ObjectField {
                    name: "vmwareClusterId".into(),
                    value: &vmware_cluster_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetappVolumeAttachmentResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            netapp_volume_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("netappVolumeId"),
            ),
            vmware_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmwareClusterId"),
            ),
        }
    }
}
