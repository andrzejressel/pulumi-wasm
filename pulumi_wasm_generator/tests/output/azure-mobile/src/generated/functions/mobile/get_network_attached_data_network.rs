pub mod get_network_attached_data_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkAttachedDataNetworkArgs {
        /// The Name of the `azure.mobile.NetworkDataNetwork` this resource belongs to.
        #[builder(into)]
        pub mobile_network_data_network_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the `azure.mobile.NetworkPacketCoreDataPlane` which the Mobile Network Attached Data Network belongs to.
        #[builder(into)]
        pub mobile_network_packet_core_data_plane_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkAttachedDataNetworkResult {
        /// The DNS servers to signal to UEs to use for this attached data network.
        pub dns_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Mobile Network Attached Data Network should exist.
        pub location: pulumi_wasm_rust::Output<String>,
        pub mobile_network_data_network_name: pulumi_wasm_rust::Output<String>,
        pub mobile_network_packet_core_data_plane_id: pulumi_wasm_rust::Output<String>,
        /// A `network_address_port_translation` block as defined below.
        pub network_address_port_translations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkAttachedDataNetworkNetworkAddressPortTranslation,
            >,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Attached Data Network.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub user_equipment_address_pool_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        pub user_equipment_static_address_pool_prefixes: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// The IPv4 address for the user data plane interface.
        pub user_plane_access_ipv4_address: pulumi_wasm_rust::Output<String>,
        /// The default IPv4 gateway for the user data plane interface.
        pub user_plane_access_ipv4_gateway: pulumi_wasm_rust::Output<String>,
        /// The IPv4 subnet for the user data plane interface.
        pub user_plane_access_ipv4_subnet: pulumi_wasm_rust::Output<String>,
        /// The logical name for thie user data plane interface.
        pub user_plane_access_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetNetworkAttachedDataNetworkArgs,
    ) -> GetNetworkAttachedDataNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mobile_network_data_network_name_binding = args
            .mobile_network_data_network_name
            .get_inner();
        let mobile_network_packet_core_data_plane_id_binding = args
            .mobile_network_packet_core_data_plane_id
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkAttachedDataNetwork:getNetworkAttachedDataNetwork"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mobileNetworkDataNetworkName".into(),
                    value: &mobile_network_data_network_name_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkPacketCoreDataPlaneId".into(),
                    value: &mobile_network_packet_core_data_plane_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnsAddresses".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkDataNetworkName".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkPacketCoreDataPlaneId".into(),
                },
                register_interface::ResultField {
                    name: "networkAddressPortTranslations".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userEquipmentAddressPoolPrefixes".into(),
                },
                register_interface::ResultField {
                    name: "userEquipmentStaticAddressPoolPrefixes".into(),
                },
                register_interface::ResultField {
                    name: "userPlaneAccessIpv4Address".into(),
                },
                register_interface::ResultField {
                    name: "userPlaneAccessIpv4Gateway".into(),
                },
                register_interface::ResultField {
                    name: "userPlaneAccessIpv4Subnet".into(),
                },
                register_interface::ResultField {
                    name: "userPlaneAccessName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkAttachedDataNetworkResult {
            dns_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsAddresses").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_data_network_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkDataNetworkName").unwrap(),
            ),
            mobile_network_packet_core_data_plane_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkPacketCoreDataPlaneId").unwrap(),
            ),
            network_address_port_translations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAddressPortTranslations").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_equipment_address_pool_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userEquipmentAddressPoolPrefixes").unwrap(),
            ),
            user_equipment_static_address_pool_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userEquipmentStaticAddressPoolPrefixes").unwrap(),
            ),
            user_plane_access_ipv4_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPlaneAccessIpv4Address").unwrap(),
            ),
            user_plane_access_ipv4_gateway: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPlaneAccessIpv4Gateway").unwrap(),
            ),
            user_plane_access_ipv4_subnet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPlaneAccessIpv4Subnet").unwrap(),
            ),
            user_plane_access_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPlaneAccessName").unwrap(),
            ),
        }
    }
}
