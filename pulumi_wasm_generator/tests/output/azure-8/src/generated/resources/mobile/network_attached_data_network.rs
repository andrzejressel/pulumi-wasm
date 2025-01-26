/// Manages a Mobile Network Attached Data Network.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleDevice:
///     type: azure:databoxedge:Device
///     name: example
///     properties:
///       name: example-device
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: EdgeP_Base-Standard
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkPacketCoreControlPlane:
///     type: azure:mobile:NetworkPacketCoreControlPlane
///     name: example
///     properties:
///       name: example-mnpccp
///       resourceGroupName: ${example.name}
///       location: West Europe
///       sku: G0
///       mobileNetworkId: ${exampleNetwork.id}
///       controlPlaneAccessName: default-interface
///       controlPlaneAccessIpv4Address: 192.168.1.199
///       controlPlaneAccessIpv4Gateway: 192.168.1.1
///       controlPlaneAccessIpv4Subnet: 192.168.1.0/25
///       platform:
///         type: AKS-HCI
///         edgeDeviceId: ${exampleDevice.id}
///   exampleNetworkPacketCoreDataPlane:
///     type: azure:mobile:NetworkPacketCoreDataPlane
///     name: example
///     properties:
///       name: example-mnpcdp
///       mobileNetworkPacketCoreControlPlaneId: ${exampleNetworkPacketCoreControlPlane.id}
///       location: ${example.location}
///       userPlaneAccessName: default-interface
///       userPlaneAccessIpv4Address: 192.168.1.199
///       userPlaneAccessIpv4Gateway: 192.168.1.1
///       userPlaneAccessIpv4Subnet: 192.168.1.0/25
///   exampleNetworkDataNetwork:
///     type: azure:mobile:NetworkDataNetwork
///     name: example
///     properties:
///       name: example-data-network
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///   exampleNetworkAttachedDataNetwork:
///     type: azure:mobile:NetworkAttachedDataNetwork
///     name: example
///     properties:
///       mobileNetworkDataNetworkName: ${exampleNetworkDataNetwork.name}
///       mobileNetworkPacketCoreDataPlaneId: ${exampleNetworkPacketCoreDataPlane.id}
///       location: ${example.location}
///       dnsAddresses:
///         - 1.1.1.1
///       userEquipmentAddressPoolPrefixes:
///         - 2.4.1.0/24
///       userEquipmentStaticAddressPoolPrefixes:
///         - 2.4.2.0/24
///       userPlaneAccessName: test
///       userPlaneAccessIpv4Address: 10.204.141.4
///       userPlaneAccessIpv4Gateway: 10.204.141.1
///       userPlaneAccessIpv4Subnet: 10.204.141.0/24
///       networkAddressPortTranslation:
///         pinholeMaximumNumber: 65536
///         icmpPinholeTimeoutInSeconds: 30
///         tcpPinholeTimeoutInSeconds: 100
///         udpPinholeTimeoutInSeconds: 39
///         portRange:
///           maximum: 49999
///           minimum: 1024
///         tcpPortReuseMinimumHoldTimeInSeconds: 120
///         udpTcpPortReuseMinimumHoldTimeInSeconds: 60
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Attached Data Network can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkAttachedDataNetwork:NetworkAttachedDataNetwork example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/packetCoreControlPlanes/packetCoreControlPlane1/packetCoreDataPlanes/packetCoreDataPlane1/attachedDataNetworks/attachedDataNetwork1
/// ```
///
pub mod network_attached_data_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAttachedDataNetworkArgs {
        /// Specifies the DNS servers to signal to UEs to use for this attached data network.
        #[builder(into)]
        pub dns_addresses: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Specifies the Azure Region where the Mobile Network Attached Data Network should exist. Changing this forces a new Mobile Network Attached Data Network to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the `azure.mobile.NetworkDataNetwork` which the Attached Data Network belongs to, Changing this forces a new Mobile Network Attached Data Network to be created.
        #[builder(into)]
        pub mobile_network_data_network_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the ID of the `azure.mobile.NetworkPacketCoreDataPlane` which the Mobile Network Attached Data Network belongs to. Changing this forces a new Mobile Network Attached Data Network to be created.
        #[builder(into)]
        pub mobile_network_packet_core_data_plane_id: pulumi_wasm_rust::InputOrOutput<
            String,
        >,
        /// A `network_address_port_translation` block as defined below.
        #[builder(into, default)]
        pub network_address_port_translation: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::mobile::NetworkAttachedDataNetworkNetworkAddressPortTranslation,
            >,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Attached Data Network.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the user equipment (UE) address pool prefixes for the attached data network from which the packet core instance will dynamically assign IP addresses to UEs. The packet core instance assigns an IP address to a UE when the UE sets up a PDU session. At least one of `user_equipment_address_pool_prefixes` and `user_equipment_static_address_pool_prefix`. If you define both, they must be of the same size.
        #[builder(into, default)]
        pub user_equipment_address_pool_prefixes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the user equipment (UE) address pool prefixes for the attached data network from which the packet core instance will assign static IP addresses to UEs. The packet core instance assigns an IP address to a UE when the UE sets up a PDU session. The static IP address for a specific UE is set in StaticIPConfiguration on the corresponding SIM resource. At least one of `user_equipment_address_pool_prefix` and `user_equipment_static_address_pool_prefixes`. If you define both, they must be of the same size.
        ///
        /// > **Note:** At least one of `user_equipment_address_pool_prefixes` and `user_equipment_static_address_pool_prefixes` must be specified.
        #[builder(into, default)]
        pub user_equipment_static_address_pool_prefixes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The IPv4 address for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_address: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The default IPv4 gateway for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_gateway: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IPv4 subnet for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_subnet: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the logical name for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkAttachedDataNetworkResult {
        /// Specifies the DNS servers to signal to UEs to use for this attached data network.
        pub dns_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the Azure Region where the Mobile Network Attached Data Network should exist. Changing this forces a new Mobile Network Attached Data Network to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the `azure.mobile.NetworkDataNetwork` which the Attached Data Network belongs to, Changing this forces a new Mobile Network Attached Data Network to be created.
        pub mobile_network_data_network_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the `azure.mobile.NetworkPacketCoreDataPlane` which the Mobile Network Attached Data Network belongs to. Changing this forces a new Mobile Network Attached Data Network to be created.
        pub mobile_network_packet_core_data_plane_id: pulumi_wasm_rust::Output<String>,
        /// A `network_address_port_translation` block as defined below.
        pub network_address_port_translation: pulumi_wasm_rust::Output<
            Option<
                super::super::types::mobile::NetworkAttachedDataNetworkNetworkAddressPortTranslation,
            >,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Attached Data Network.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the user equipment (UE) address pool prefixes for the attached data network from which the packet core instance will dynamically assign IP addresses to UEs. The packet core instance assigns an IP address to a UE when the UE sets up a PDU session. At least one of `user_equipment_address_pool_prefixes` and `user_equipment_static_address_pool_prefix`. If you define both, they must be of the same size.
        pub user_equipment_address_pool_prefixes: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// Specifies the user equipment (UE) address pool prefixes for the attached data network from which the packet core instance will assign static IP addresses to UEs. The packet core instance assigns an IP address to a UE when the UE sets up a PDU session. The static IP address for a specific UE is set in StaticIPConfiguration on the corresponding SIM resource. At least one of `user_equipment_address_pool_prefix` and `user_equipment_static_address_pool_prefixes`. If you define both, they must be of the same size.
        ///
        /// > **Note:** At least one of `user_equipment_address_pool_prefixes` and `user_equipment_static_address_pool_prefixes` must be specified.
        pub user_equipment_static_address_pool_prefixes: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The IPv4 address for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The default IPv4 gateway for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_gateway: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 subnet for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_subnet: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the logical name for the user data plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NetworkAttachedDataNetworkArgs,
    ) -> NetworkAttachedDataNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_addresses_binding = args.dns_addresses.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mobile_network_data_network_name_binding = args
            .mobile_network_data_network_name
            .get_output(context)
            .get_inner();
        let mobile_network_packet_core_data_plane_id_binding = args
            .mobile_network_packet_core_data_plane_id
            .get_output(context)
            .get_inner();
        let network_address_port_translation_binding = args
            .network_address_port_translation
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_equipment_address_pool_prefixes_binding = args
            .user_equipment_address_pool_prefixes
            .get_output(context)
            .get_inner();
        let user_equipment_static_address_pool_prefixes_binding = args
            .user_equipment_static_address_pool_prefixes
            .get_output(context)
            .get_inner();
        let user_plane_access_ipv4_address_binding = args
            .user_plane_access_ipv4_address
            .get_output(context)
            .get_inner();
        let user_plane_access_ipv4_gateway_binding = args
            .user_plane_access_ipv4_gateway
            .get_output(context)
            .get_inner();
        let user_plane_access_ipv4_subnet_binding = args
            .user_plane_access_ipv4_subnet
            .get_output(context)
            .get_inner();
        let user_plane_access_name_binding = args
            .user_plane_access_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkAttachedDataNetwork:NetworkAttachedDataNetwork"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsAddresses".into(),
                    value: &dns_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkDataNetworkName".into(),
                    value: &mobile_network_data_network_name_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkPacketCoreDataPlaneId".into(),
                    value: &mobile_network_packet_core_data_plane_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkAddressPortTranslation".into(),
                    value: &network_address_port_translation_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userEquipmentAddressPoolPrefixes".into(),
                    value: &user_equipment_address_pool_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "userEquipmentStaticAddressPoolPrefixes".into(),
                    value: &user_equipment_static_address_pool_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "userPlaneAccessIpv4Address".into(),
                    value: &user_plane_access_ipv4_address_binding,
                },
                register_interface::ObjectField {
                    name: "userPlaneAccessIpv4Gateway".into(),
                    value: &user_plane_access_ipv4_gateway_binding,
                },
                register_interface::ObjectField {
                    name: "userPlaneAccessIpv4Subnet".into(),
                    value: &user_plane_access_ipv4_subnet_binding,
                },
                register_interface::ObjectField {
                    name: "userPlaneAccessName".into(),
                    value: &user_plane_access_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnsAddresses".into(),
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
                    name: "networkAddressPortTranslation".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkAttachedDataNetworkResult {
            dns_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsAddresses").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_data_network_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkDataNetworkName").unwrap(),
            ),
            mobile_network_packet_core_data_plane_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkPacketCoreDataPlaneId").unwrap(),
            ),
            network_address_port_translation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAddressPortTranslation").unwrap(),
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
