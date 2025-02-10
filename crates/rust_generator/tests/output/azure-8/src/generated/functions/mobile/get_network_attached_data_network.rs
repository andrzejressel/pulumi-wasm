#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_attached_data_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkAttachedDataNetworkArgs {
        /// The Name of the `azure.mobile.NetworkDataNetwork` this resource belongs to.
        #[builder(into)]
        pub mobile_network_data_network_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the `azure.mobile.NetworkPacketCoreDataPlane` which the Mobile Network Attached Data Network belongs to.
        #[builder(into)]
        pub mobile_network_packet_core_data_plane_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNetworkAttachedDataNetworkResult {
        /// The DNS servers to signal to UEs to use for this attached data network.
        pub dns_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Mobile Network Attached Data Network should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub mobile_network_data_network_name: pulumi_gestalt_rust::Output<String>,
        pub mobile_network_packet_core_data_plane_id: pulumi_gestalt_rust::Output<
            String,
        >,
        /// A `network_address_port_translation` block as defined below.
        pub network_address_port_translations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkAttachedDataNetworkNetworkAddressPortTranslation,
            >,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Attached Data Network.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub user_equipment_address_pool_prefixes: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        pub user_equipment_static_address_pool_prefixes: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// The IPv4 address for the user data plane interface.
        pub user_plane_access_ipv4_address: pulumi_gestalt_rust::Output<String>,
        /// The default IPv4 gateway for the user data plane interface.
        pub user_plane_access_ipv4_gateway: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 subnet for the user data plane interface.
        pub user_plane_access_ipv4_subnet: pulumi_gestalt_rust::Output<String>,
        /// The logical name for thie user data plane interface.
        pub user_plane_access_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkAttachedDataNetworkArgs,
    ) -> GetNetworkAttachedDataNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mobile_network_data_network_name_binding = args
            .mobile_network_data_network_name
            .get_output(context);
        let mobile_network_packet_core_data_plane_id_binding = args
            .mobile_network_packet_core_data_plane_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetworkAttachedDataNetwork:getNetworkAttachedDataNetwork"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkDataNetworkName".into(),
                    value: mobile_network_data_network_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkPacketCoreDataPlaneId".into(),
                    value: mobile_network_packet_core_data_plane_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkAttachedDataNetworkResult {
            dns_addresses: o.get_field("dnsAddresses"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            mobile_network_data_network_name: o
                .get_field("mobileNetworkDataNetworkName"),
            mobile_network_packet_core_data_plane_id: o
                .get_field("mobileNetworkPacketCoreDataPlaneId"),
            network_address_port_translations: o
                .get_field("networkAddressPortTranslations"),
            tags: o.get_field("tags"),
            user_equipment_address_pool_prefixes: o
                .get_field("userEquipmentAddressPoolPrefixes"),
            user_equipment_static_address_pool_prefixes: o
                .get_field("userEquipmentStaticAddressPoolPrefixes"),
            user_plane_access_ipv4_address: o.get_field("userPlaneAccessIpv4Address"),
            user_plane_access_ipv4_gateway: o.get_field("userPlaneAccessIpv4Gateway"),
            user_plane_access_ipv4_subnet: o.get_field("userPlaneAccessIpv4Subnet"),
            user_plane_access_name: o.get_field("userPlaneAccessName"),
        }
    }
}
