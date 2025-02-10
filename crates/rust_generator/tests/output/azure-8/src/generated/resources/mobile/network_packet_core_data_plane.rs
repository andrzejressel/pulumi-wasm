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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_packet_core_data_plane {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkPacketCoreDataPlaneArgs {
        /// Specifies the Azure Region where the Mobile Network Packet Core Data Plane should exist. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        #[builder(into)]
        pub mobile_network_packet_core_control_plane_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Specifies the name which should be used for this Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Data Plane.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IPv4 address for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_address: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The default IPv4 gateway for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_gateway: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IPv4 subnet for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_ipv4_subnet: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the logical name for thie user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub user_plane_access_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NetworkPacketCoreDataPlaneResult {
        /// Specifies the Azure Region where the Mobile Network Packet Core Data Plane should exist. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        pub mobile_network_packet_core_control_plane_id: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Specifies the name which should be used for this Mobile Network Packet Core Data Plane. Changing this forces a new Mobile Network Packet Core Data Plane to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Data Plane.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IPv4 address for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default IPv4 gateway for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_gateway: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 subnet for the user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_ipv4_subnet: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the logical name for thie user plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub user_plane_access_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkPacketCoreDataPlaneArgs,
    ) -> NetworkPacketCoreDataPlaneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let mobile_network_packet_core_control_plane_id_binding = args
            .mobile_network_packet_core_control_plane_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_plane_access_ipv4_address_binding = args
            .user_plane_access_ipv4_address
            .get_output(context);
        let user_plane_access_ipv4_gateway_binding = args
            .user_plane_access_ipv4_gateway
            .get_output(context);
        let user_plane_access_ipv4_subnet_binding = args
            .user_plane_access_ipv4_subnet
            .get_output(context);
        let user_plane_access_name_binding = args
            .user_plane_access_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mobile/networkPacketCoreDataPlane:NetworkPacketCoreDataPlane"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkPacketCoreControlPlaneId".into(),
                    value: mobile_network_packet_core_control_plane_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPlaneAccessIpv4Address".into(),
                    value: user_plane_access_ipv4_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPlaneAccessIpv4Gateway".into(),
                    value: user_plane_access_ipv4_gateway_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPlaneAccessIpv4Subnet".into(),
                    value: user_plane_access_ipv4_subnet_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPlaneAccessName".into(),
                    value: user_plane_access_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkPacketCoreDataPlaneResult {
            location: o.get_field("location"),
            mobile_network_packet_core_control_plane_id: o
                .get_field("mobileNetworkPacketCoreControlPlaneId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            user_plane_access_ipv4_address: o.get_field("userPlaneAccessIpv4Address"),
            user_plane_access_ipv4_gateway: o.get_field("userPlaneAccessIpv4Gateway"),
            user_plane_access_ipv4_subnet: o.get_field("userPlaneAccessIpv4Subnet"),
            user_plane_access_name: o.get_field("userPlaneAccessName"),
        }
    }
}
