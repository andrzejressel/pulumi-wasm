pub mod get_network_interface {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceArgs {
        /// Specifies the name of the Network Interface.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Network Interface is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInterfaceResult {
        /// Indicates if accelerated networking is set on the specified Network Interface.
        pub accelerated_networking_enabled: pulumi_wasm_rust::Output<bool>,
        /// List of DNS servers applied to the specified Network Interface.
        pub applied_dns_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The list of DNS servers used by the specified Network Interface.
        pub dns_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The internal DNS name label of the specified Network Interface.
        pub internal_dns_name_label: pulumi_wasm_rust::Output<String>,
        /// One or more `ip_configuration` blocks as defined below.
        pub ip_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetNetworkInterfaceIpConfiguration>,
        >,
        /// Indicate if IP forwarding is set on the specified Network Interface.
        pub ip_forwarding_enabled: pulumi_wasm_rust::Output<bool>,
        /// The location of the specified Network Interface.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The MAC address used by the specified Network Interface.
        pub mac_address: pulumi_wasm_rust::Output<String>,
        /// The name of the IP Configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the network security group associated to the specified Network Interface.
        pub network_security_group_id: pulumi_wasm_rust::Output<String>,
        /// The Private IP Address assigned to this Network Interface.
        pub private_ip_address: pulumi_wasm_rust::Output<String>,
        /// The list of private IP addresses associates to the specified Network Interface.
        pub private_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// List the tags associated to the specified Network Interface.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the virtual machine that the specified Network Interface is attached to.
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNetworkInterfaceArgs) -> GetNetworkInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getNetworkInterface:getNetworkInterface".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "dnsServers".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "internalDnsNameLabel".into(),
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
                    name: "networkSecurityGroupId".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkInterfaceResult {
            accelerated_networking_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceleratedNetworkingEnabled").unwrap(),
            ),
            applied_dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appliedDnsServers").unwrap(),
            ),
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsServers").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            internal_dns_name_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internalDnsNameLabel").unwrap(),
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
            network_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkSecurityGroupId").unwrap(),
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
