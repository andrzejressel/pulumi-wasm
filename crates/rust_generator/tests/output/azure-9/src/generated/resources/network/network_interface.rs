/// Manages a Network Interface.
///
/// ## Example Usage
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
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("internal")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Network Interfaces can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkInterface:NetworkInterface example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkInterfaces/nic1
/// ```
///
pub mod network_interface {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceArgs {
        /// Should Accelerated Networking be enabled? Defaults to `false`.
        ///
        /// > **Note:** Only certain Virtual Machine sizes are supported for Accelerated Networking - [more information can be found in this document](https://docs.microsoft.com/azure/virtual-network/create-vm-accelerated-networking-cli).
        ///
        /// > **Note:** To use Accelerated Networking in an Availability Set, the Availability Set must be deployed onto an Accelerated Networking enabled cluster.
        #[builder(into, default)]
        pub accelerated_networking_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the auxiliary mode used to enable network high-performance feature on Network Virtual Appliances (NVAs). This feature offers competitive performance in Connections Per Second (CPS) optimization, along with improvements to handling large amounts of simultaneous connections. Possible values are `AcceleratedConnections`, `Floating`, `MaxConnections` and `None`.
        ///
        /// > **Note:** `auxiliary_mode` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        #[builder(into, default)]
        pub auxiliary_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the SKU used for the network high-performance feature on Network Virtual Appliances (NVAs). Possible values are `A8`, `A4`, `A1`, `A2` and `None`.
        ///
        /// > **Note:** `auxiliary_sku` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        #[builder(into, default)]
        pub auxiliary_sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of IP Addresses defining the DNS Servers which should be used for this Network Interface.
        ///
        /// > **Note:** Configuring DNS Servers on the Network Interface will override the DNS Servers defined on the Virtual Network.
        #[builder(into, default)]
        pub dns_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the Edge Zone within the Azure Region where this Network Interface should exist. Changing this forces a new Network Interface to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The (relative) DNS Name used for internal communications between Virtual Machines in the same Virtual Network.
        #[builder(into, default)]
        pub internal_dns_name_label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `ip_configuration` blocks as defined below.
        #[builder(into)]
        pub ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::NetworkInterfaceIpConfiguration>,
        >,
        /// Should IP Forwarding be enabled? Defaults to `false`.
        #[builder(into, default)]
        pub ip_forwarding_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The location where the Network Interface should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Network Interface. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which to create the Network Interface. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceResult {
        /// Should Accelerated Networking be enabled? Defaults to `false`.
        ///
        /// > **Note:** Only certain Virtual Machine sizes are supported for Accelerated Networking - [more information can be found in this document](https://docs.microsoft.com/azure/virtual-network/create-vm-accelerated-networking-cli).
        ///
        /// > **Note:** To use Accelerated Networking in an Availability Set, the Availability Set must be deployed onto an Accelerated Networking enabled cluster.
        pub accelerated_networking_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If the Virtual Machine using this Network Interface is part of an Availability Set, then this list will have the union of all DNS servers from all Network Interfaces that are part of the Availability Set.
        pub applied_dns_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the auxiliary mode used to enable network high-performance feature on Network Virtual Appliances (NVAs). This feature offers competitive performance in Connections Per Second (CPS) optimization, along with improvements to handling large amounts of simultaneous connections. Possible values are `AcceleratedConnections`, `Floating`, `MaxConnections` and `None`.
        ///
        /// > **Note:** `auxiliary_mode` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        pub auxiliary_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the SKU used for the network high-performance feature on Network Virtual Appliances (NVAs). Possible values are `A8`, `A4`, `A1`, `A2` and `None`.
        ///
        /// > **Note:** `auxiliary_sku` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        pub auxiliary_sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of IP Addresses defining the DNS Servers which should be used for this Network Interface.
        ///
        /// > **Note:** Configuring DNS Servers on the Network Interface will override the DNS Servers defined on the Virtual Network.
        pub dns_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the Edge Zone within the Azure Region where this Network Interface should exist. Changing this forces a new Network Interface to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The (relative) DNS Name used for internal communications between Virtual Machines in the same Virtual Network.
        pub internal_dns_name_label: pulumi_gestalt_rust::Output<Option<String>>,
        /// Even if `internal_dns_name_label` is not specified, a DNS entry is created for the primary NIC of the VM. This DNS name can be constructed by concatenating the VM name with the value of `internal_domain_name_suffix`.
        pub internal_domain_name_suffix: pulumi_gestalt_rust::Output<String>,
        /// One or more `ip_configuration` blocks as defined below.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::NetworkInterfaceIpConfiguration>,
        >,
        /// Should IP Forwarding be enabled? Defaults to `false`.
        pub ip_forwarding_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The location where the Network Interface should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Media Access Control (MAC) Address of the Network Interface.
        pub mac_address: pulumi_gestalt_rust::Output<String>,
        /// The name of the Network Interface. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The first private IP address of the network interface.
        pub private_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The private IP addresses of the network interface.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the Resource Group in which to create the Network Interface. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Machine which this Network Interface is connected to.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NetworkInterfaceArgs,
    ) -> NetworkInterfaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accelerated_networking_enabled_binding = args
            .accelerated_networking_enabled
            .get_output(context)
            .get_inner();
        let auxiliary_mode_binding = args.auxiliary_mode.get_output(context).get_inner();
        let auxiliary_sku_binding = args.auxiliary_sku.get_output(context).get_inner();
        let dns_servers_binding = args.dns_servers.get_output(context).get_inner();
        let edge_zone_binding = args.edge_zone.get_output(context).get_inner();
        let internal_dns_name_label_binding = args
            .internal_dns_name_label
            .get_output(context)
            .get_inner();
        let ip_configurations_binding = args
            .ip_configurations
            .get_output(context)
            .get_inner();
        let ip_forwarding_enabled_binding = args
            .ip_forwarding_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkInterface:NetworkInterface".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratedNetworkingEnabled".into(),
                    value: &accelerated_networking_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "auxiliaryMode".into(),
                    value: &auxiliary_mode_binding,
                },
                register_interface::ObjectField {
                    name: "auxiliarySku".into(),
                    value: &auxiliary_sku_binding,
                },
                register_interface::ObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding,
                },
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "internalDnsNameLabel".into(),
                    value: &internal_dns_name_label_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfigurations".into(),
                    value: &ip_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "ipForwardingEnabled".into(),
                    value: &ip_forwarding_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkInterfaceResult {
            accelerated_networking_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratedNetworkingEnabled"),
            ),
            applied_dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appliedDnsServers"),
            ),
            auxiliary_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("auxiliaryMode"),
            ),
            auxiliary_sku: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("auxiliarySku"),
            ),
            dns_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsServers"),
            ),
            edge_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeZone"),
            ),
            internal_dns_name_label: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalDnsNameLabel"),
            ),
            internal_domain_name_suffix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("internalDomainNameSuffix"),
            ),
            ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipConfigurations"),
            ),
            ip_forwarding_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipForwardingEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mac_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("macAddress"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            private_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddresses"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            virtual_machine_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualMachineId"),
            ),
        }
    }
}
