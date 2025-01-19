/// Manages a Mobile Network Packet Core Data Plane.
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
///       location: ${example.location}
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
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Packet Core Data Plane can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkPacketCoreDataPlane:NetworkPacketCoreDataPlane example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/packetCoreControlPlanes/packetCoreControlPlane1/packetCoreDataPlanes/packetCoreDataPlane1
/// ```
///
pub mod network_packet_core_data_plane {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkPacketCoreDataPlaneArgs {
        /// Specifies the Azure Region where the Mobile Network Packet Core Data Plane should exist. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        #[builder(into)]
        pub mobile_network_packet_core_control_plane_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// Specifies the name which should be used for this Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Data Plane.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IPv4 address for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The default IPv4 gateway for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_gateway: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 subnet for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_subnet: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the logical name for thie user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkPacketCoreDataPlaneResult {
        /// Specifies the Azure Region where the Mobile Network Packet Core Data Plane should exist. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        pub mobile_network_packet_core_control_plane_id: pulumi_wasm_rust::Output<
            String,
        >,
        /// Specifies the name which should be used for this Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Data Plane.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IPv4 address for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The default IPv4 gateway for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_gateway: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 subnet for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_subnet: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the logical name for thie user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkPacketCoreDataPlaneArgs,
    ) -> NetworkPacketCoreDataPlaneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let mobile_network_packet_core_control_plane_id_binding = args
            .mobile_network_packet_core_control_plane_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_plane_access_ipv4_address_binding = args
            .user_plane_access_ipv4_address
            .get_inner();
        let user_plane_access_ipv4_gateway_binding = args
            .user_plane_access_ipv4_gateway
            .get_inner();
        let user_plane_access_ipv4_subnet_binding = args
            .user_plane_access_ipv4_subnet
            .get_inner();
        let user_plane_access_name_binding = args.user_plane_access_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkPacketCoreDataPlane:NetworkPacketCoreDataPlane"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkPacketCoreControlPlaneId".into(),
                    value: &mobile_network_packet_core_control_plane_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkPacketCoreControlPlaneId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkPacketCoreDataPlaneResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_packet_core_control_plane_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkPacketCoreControlPlaneId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
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
