/// Manages an Azure Stack HCI Logical Network.
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
///         vlanId: 123
///         route:
///           addressPrefix: 0.0.0.0/0
///           nextHopIpAddress: 10.0.0.1
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Azure Stack HCI Logical Networks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciLogicalNetwork:HciLogicalNetwork example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AzureStackHCI/logicalNetworks/ln1
/// ```
///
pub mod hci_logical_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciLogicalNetworkArgs {
        /// The ID of Custom Location where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// A list of IPv4 addresses of DNS servers available to VMs deployed in the Logical Networks. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Azure Region where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Stack HCI Logical Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `subnet` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet: pulumi_wasm_rust::Output<
            super::super::types::stack::HciLogicalNetworkSubnet,
        >,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Logical Network.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the virtual switch on the cluster used to associate with the Azure Stack HCI Logical Network. Possible switch names can be retrieved by following this [Azure guide](https://learn.microsoft.com/azure-stack/hci/manage/create-logical-networks?tabs=azurecli#prerequisites). Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_switch_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct HciLogicalNetworkResult {
        /// The ID of Custom Location where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// A list of IPv4 addresses of DNS servers available to VMs deployed in the Logical Networks. Changing this forces a new resource to be created.
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Azure Region where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure Stack HCI Logical Network. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Stack HCI Logical Network should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `subnet` block as defined below. Changing this forces a new resource to be created.
        pub subnet: pulumi_wasm_rust::Output<
            super::super::types::stack::HciLogicalNetworkSubnet,
        >,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Logical Network.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the virtual switch on the cluster used to associate with the Azure Stack HCI Logical Network. Possible switch names can be retrieved by following this [Azure guide](https://learn.microsoft.com/azure-stack/hci/manage/create-logical-networks?tabs=azurecli#prerequisites). Changing this forces a new resource to be created.
        pub virtual_switch_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HciLogicalNetworkArgs) -> HciLogicalNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_location_id_binding = args.custom_location_id.get_inner();
        let dns_servers_binding = args.dns_servers.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subnet_binding = args.subnet.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_switch_name_binding = args.virtual_switch_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:stack/hciLogicalNetwork:HciLogicalNetwork".into(),
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
                    name: "subnet".into(),
                    value: &subnet_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualSwitchName".into(),
                    value: &virtual_switch_name_binding,
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
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subnet".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualSwitchName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HciLogicalNetworkResult {
            custom_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customLocationId").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subnet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnet").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_switch_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualSwitchName").unwrap(),
            ),
        }
    }
}
