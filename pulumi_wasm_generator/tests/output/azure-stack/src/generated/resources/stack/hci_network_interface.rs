/// Manages an Azure Stack HCI Network Interface.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleHciLogicalNetwork:
///     type: azure:stack:HciLogicalNetwork
///     name: example
///     properties:
///       name: example-hci-ln
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       customLocationId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ExtendedLocation/customLocations/cl1
///       virtualSwitchName: ConvergedSwitch(managementcompute)
///       dnsServers:
///         - 10.0.0.7
///         - 10.0.0.8
///       subnet:
///         ipAllocationMethod: Static
///         addressPrefix: 10.0.0.0/24
///         route:
///           name: example-route
///           addressPrefix: 0.0.0.0/0
///           nextHopIpAddress: 10.0.20.1
///         vlanId: 123
///       tags:
///         foo: bar
///   exampleHciNetworkInterface:
///     type: azure:stack:HciNetworkInterface
///     name: example
///     properties:
///       name: example-ni
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       customLocationId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ExtendedLocation/customLocations/cl1
///       dnsServers:
///         - 10.0.0.8
///       ipConfiguration:
///         privateIpAddress: 10.0.0.2
///         subnetId: ${test.id}
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Azure Stack HCI Network Interfaces can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciNetworkInterface:HciNetworkInterface example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AzureStackHCI/networkInterfaces/ni1
/// ```
///
pub mod hci_network_interface {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciNetworkInterfaceArgs {
        /// The ID of the Custom Location where the Azure Stack HCI Network Interface should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// A list of IPv4 addresses of DNS servers available to VMs deployed in the Network Interface. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `ip_configuration` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ip_configuration: pulumi_wasm_rust::Output<
            super::super::types::stack::HciNetworkInterfaceIpConfiguration,
        >,
        /// The Azure Region where the Azure Stack HCI Network Interface should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The MAC address of the Network Interface. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `mac_address` is not specified, it will be assigned by the server. If you experience a diff you may need to add this to `ignore_changes`.
        #[builder(into, default)]
        pub mac_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Stack HCI Network Interface. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Azure Stack HCI Network Interface should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Network Interface.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HciNetworkInterfaceResult {
        /// The ID of the Custom Location where the Azure Stack HCI Network Interface should exist. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// A list of IPv4 addresses of DNS servers available to VMs deployed in the Network Interface. Changing this forces a new resource to be created.
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `ip_configuration` block as defined below. Changing this forces a new resource to be created.
        pub ip_configuration: pulumi_wasm_rust::Output<
            super::super::types::stack::HciNetworkInterfaceIpConfiguration,
        >,
        /// The Azure Region where the Azure Stack HCI Network Interface should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The MAC address of the Network Interface. Changing this forces a new resource to be created.
        ///
        /// > **Note:** If `mac_address` is not specified, it will be assigned by the server. If you experience a diff you may need to add this to `ignore_changes`.
        pub mac_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Stack HCI Network Interface. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Stack HCI Network Interface should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Network Interface.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: HciNetworkInterfaceArgs,
    ) -> HciNetworkInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_location_id_binding = args.custom_location_id.get_inner();
        let dns_servers_binding = args.dns_servers.get_inner();
        let ip_configuration_binding = args.ip_configuration.get_inner();
        let location_binding = args.location.get_inner();
        let mac_address_binding = args.mac_address.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:stack/hciNetworkInterface:HciNetworkInterface".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customLocationId".into(),
                    value: &custom_location_id_binding,
                },
                register_interface::ObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfiguration".into(),
                    value: &ip_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "macAddress".into(),
                    value: &mac_address_binding,
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
                    name: "customLocationId".into(),
                },
                register_interface::ResultField {
                    name: "dnsServers".into(),
                },
                register_interface::ResultField {
                    name: "ipConfiguration".into(),
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HciNetworkInterfaceResult {
            custom_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customLocationId").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            ip_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfiguration").unwrap(),
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
