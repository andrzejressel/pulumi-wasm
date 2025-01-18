/// Manages a Network Interface.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceArgs {
        /// Should Accelerated Networking be enabled? Defaults to `false`.
        ///
        /// > **Note:** Only certain Virtual Machine sizes are supported for Accelerated Networking - [more information can be found in this document](https://docs.microsoft.com/azure/virtual-network/create-vm-accelerated-networking-cli).
        ///
        /// > **Note:** To use Accelerated Networking in an Availability Set, the Availability Set must be deployed onto an Accelerated Networking enabled cluster.
        #[builder(into, default)]
        pub accelerated_networking_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the auxiliary mode used to enable network high-performance feature on Network Virtual Appliances (NVAs). This feature offers competitive performance in Connections Per Second (CPS) optimization, along with improvements to handling large amounts of simultaneous connections. Possible values are `AcceleratedConnections`, `Floating`, `MaxConnections` and `None`.
        ///
        /// > **Note:** `auxiliary_mode` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        #[builder(into, default)]
        pub auxiliary_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the SKU used for the network high-performance feature on Network Virtual Appliances (NVAs). Possible values are `A8`, `A4`, `A1`, `A2` and `None`.
        ///
        /// > **Note:** `auxiliary_sku` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        #[builder(into, default)]
        pub auxiliary_sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IP Addresses defining the DNS Servers which should be used for this Network Interface.
        ///
        /// > **Note:** Configuring DNS Servers on the Network Interface will override the DNS Servers defined on the Virtual Network.
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the Edge Zone within the Azure Region where this Network Interface should exist. Changing this forces a new Network Interface to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The (relative) DNS Name used for internal communications between Virtual Machines in the same Virtual Network.
        #[builder(into, default)]
        pub internal_dns_name_label: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `ip_configuration` blocks as defined below.
        #[builder(into)]
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkInterfaceIpConfiguration>,
        >,
        /// Should IP Forwarding be enabled? Defaults to `false`.
        #[builder(into, default)]
        pub ip_forwarding_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The location where the Network Interface should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Network Interface. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which to create the Network Interface. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
        pub accelerated_networking_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// If the Virtual Machine using this Network Interface is part of an Availability Set, then this list will have the union of all DNS servers from all Network Interfaces that are part of the Availability Set.
        pub applied_dns_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the auxiliary mode used to enable network high-performance feature on Network Virtual Appliances (NVAs). This feature offers competitive performance in Connections Per Second (CPS) optimization, along with improvements to handling large amounts of simultaneous connections. Possible values are `AcceleratedConnections`, `Floating`, `MaxConnections` and `None`.
        ///
        /// > **Note:** `auxiliary_mode` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        pub auxiliary_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the SKU used for the network high-performance feature on Network Virtual Appliances (NVAs). Possible values are `A8`, `A4`, `A1`, `A2` and `None`.
        ///
        /// > **Note:** `auxiliary_sku` is in **Preview** and requires that the preview is enabled - [more information can be found in the Azure documentation](https://learn.microsoft.com/azure/networking/nva-accelerated-connections#prerequisites).
        pub auxiliary_sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IP Addresses defining the DNS Servers which should be used for this Network Interface.
        ///
        /// > **Note:** Configuring DNS Servers on the Network Interface will override the DNS Servers defined on the Virtual Network.
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the Edge Zone within the Azure Region where this Network Interface should exist. Changing this forces a new Network Interface to be created.
        pub edge_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The (relative) DNS Name used for internal communications between Virtual Machines in the same Virtual Network.
        pub internal_dns_name_label: pulumi_wasm_rust::Output<Option<String>>,
        /// Even if `internal_dns_name_label` is not specified, a DNS entry is created for the primary NIC of the VM. This DNS name can be constructed by concatenating the VM name with the value of `internal_domain_name_suffix`.
        pub internal_domain_name_suffix: pulumi_wasm_rust::Output<String>,
        /// One or more `ip_configuration` blocks as defined below.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkInterfaceIpConfiguration>,
        >,
        /// Should IP Forwarding be enabled? Defaults to `false`.
        pub ip_forwarding_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The location where the Network Interface should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Media Access Control (MAC) Address of the Network Interface.
        pub mac_address: pulumi_wasm_rust::Output<String>,
        /// The name of the Network Interface. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The first private IP address of the network interface.
        pub private_ip_address: pulumi_wasm_rust::Output<String>,
        /// The private IP addresses of the network interface.
        pub private_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the Resource Group in which to create the Network Interface. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Machine which this Network Interface is connected to.
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkInterfaceArgs) -> NetworkInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accelerated_networking_enabled_binding = args
            .accelerated_networking_enabled
            .get_inner();
        let auxiliary_mode_binding = args.auxiliary_mode.get_inner();
        let auxiliary_sku_binding = args.auxiliary_sku.get_inner();
        let dns_servers_binding = args.dns_servers.get_inner();
        let edge_zone_binding = args.edge_zone.get_inner();
        let internal_dns_name_label_binding = args.internal_dns_name_label.get_inner();
        let ip_configurations_binding = args.ip_configurations.get_inner();
        let ip_forwarding_enabled_binding = args.ip_forwarding_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceleratedNetworkingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "appliedDnsServers".into(),
                },
                register_interface::ResultField {
                    name: "auxiliaryMode".into(),
                },
                register_interface::ResultField {
                    name: "auxiliarySku".into(),
                },
                register_interface::ResultField {
                    name: "dnsServers".into(),
                },
                register_interface::ResultField {
                    name: "edgeZone".into(),
                },
                register_interface::ResultField {
                    name: "internalDnsNameLabel".into(),
                },
                register_interface::ResultField {
                    name: "internalDomainNameSuffix".into(),
                },
                register_interface::ResultField {
                    name: "ipConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "ipForwardingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "macAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkInterfaceResult {
            accelerated_networking_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceleratedNetworkingEnabled").unwrap(),
            ),
            applied_dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appliedDnsServers").unwrap(),
            ),
            auxiliary_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auxiliaryMode").unwrap(),
            ),
            auxiliary_sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auxiliarySku").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            edge_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeZone").unwrap(),
            ),
            internal_dns_name_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalDnsNameLabel").unwrap(),
            ),
            internal_domain_name_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalDomainNameSuffix").unwrap(),
            ),
            ip_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfigurations").unwrap(),
            ),
            ip_forwarding_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipForwardingEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mac_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("macAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddress").unwrap(),
            ),
            private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddresses").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineId").unwrap(),
            ),
        }
    }
}
