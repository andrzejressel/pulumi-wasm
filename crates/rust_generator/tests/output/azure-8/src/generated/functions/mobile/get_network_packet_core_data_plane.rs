#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_packet_core_data_plane {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreDataPlaneArgs {
        /// The ID of the Mobile Network Packet Core Data Plane.
        #[builder(into)]
        pub mobile_network_packet_core_control_plane_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The name of the Mobile Network Packet Core Data Plane.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreDataPlaneResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Mobile Network Packet Core Data Plane should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub mobile_network_packet_core_control_plane_id: pulumi_gestalt_rust::Output<
            String,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Data Plane.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The IPv4 address for the user plane interface.
        pub user_plane_access_ipv4_address: pulumi_gestalt_rust::Output<String>,
        /// The default IPv4 gateway for the user plane interface.
        pub user_plane_access_ipv4_gateway: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 subnet for the user plane interface.
        pub user_plane_access_ipv4_subnet: pulumi_gestalt_rust::Output<String>,
        /// The logical name for thie user plane interface.
        pub user_plane_access_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkPacketCoreDataPlaneArgs,
    ) -> GetNetworkPacketCoreDataPlaneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mobile_network_packet_core_control_plane_id_binding = args
            .mobile_network_packet_core_control_plane_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetworkPacketCoreDataPlane:getNetworkPacketCoreDataPlane"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkPacketCoreControlPlaneId".into(),
                    value: mobile_network_packet_core_control_plane_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkPacketCoreDataPlaneResult {
            id: o.get_field("id"),
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
