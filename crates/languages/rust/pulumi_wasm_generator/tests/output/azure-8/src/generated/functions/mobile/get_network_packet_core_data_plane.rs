pub mod get_network_packet_core_data_plane {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreDataPlaneArgs {
        /// The ID of the Mobile Network Packet Core Data Plane.
        #[builder(into)]
        pub mobile_network_packet_core_control_plane_id: pulumi_wasm_rust::InputOrOutput<
            String,
        >,
        /// The name of the Mobile Network Packet Core Data Plane.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreDataPlaneResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Mobile Network Packet Core Data Plane should exist.
        pub location: pulumi_wasm_rust::Output<String>,
        pub mobile_network_packet_core_control_plane_id: pulumi_wasm_rust::Output<
            String,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Data Plane.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The IPv4 address for the user plane interface.
        pub user_plane_access_ipv4_address: pulumi_wasm_rust::Output<String>,
        /// The default IPv4 gateway for the user plane interface.
        pub user_plane_access_ipv4_gateway: pulumi_wasm_rust::Output<String>,
        /// The IPv4 subnet for the user plane interface.
        pub user_plane_access_ipv4_subnet: pulumi_wasm_rust::Output<String>,
        /// The logical name for thie user plane interface.
        pub user_plane_access_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkPacketCoreDataPlaneArgs,
    ) -> GetNetworkPacketCoreDataPlaneResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mobile_network_packet_core_control_plane_id_binding = args
            .mobile_network_packet_core_control_plane_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkPacketCoreDataPlane:getNetworkPacketCoreDataPlane"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mobileNetworkPacketCoreControlPlaneId".into(),
                    value: &mobile_network_packet_core_control_plane_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkPacketCoreDataPlaneResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mobile_network_packet_core_control_plane_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mobileNetworkPacketCoreControlPlaneId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            user_plane_access_ipv4_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPlaneAccessIpv4Address"),
            ),
            user_plane_access_ipv4_gateway: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPlaneAccessIpv4Gateway"),
            ),
            user_plane_access_ipv4_subnet: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPlaneAccessIpv4Subnet"),
            ),
            user_plane_access_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPlaneAccessName"),
            ),
        }
    }
}
