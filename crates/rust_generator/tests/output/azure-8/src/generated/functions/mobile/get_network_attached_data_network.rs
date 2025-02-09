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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkAttachedDataNetworkArgs,
    ) -> GetNetworkAttachedDataNetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mobile_network_data_network_name_binding_1 = args
            .mobile_network_data_network_name
            .get_output(context);
        let mobile_network_data_network_name_binding = mobile_network_data_network_name_binding_1
            .get_inner();
        let mobile_network_packet_core_data_plane_id_binding_1 = args
            .mobile_network_packet_core_data_plane_id
            .get_output(context);
        let mobile_network_packet_core_data_plane_id_binding = mobile_network_packet_core_data_plane_id_binding_1
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkAttachedDataNetwork:getNetworkAttachedDataNetwork"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkAttachedDataNetworkResult {
            dns_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsAddresses"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mobile_network_data_network_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mobileNetworkDataNetworkName"),
            ),
            mobile_network_packet_core_data_plane_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mobileNetworkPacketCoreDataPlaneId"),
            ),
            network_address_port_translations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkAddressPortTranslations"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            user_equipment_address_pool_prefixes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userEquipmentAddressPoolPrefixes"),
            ),
            user_equipment_static_address_pool_prefixes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userEquipmentStaticAddressPoolPrefixes"),
            ),
            user_plane_access_ipv4_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPlaneAccessIpv4Address"),
            ),
            user_plane_access_ipv4_gateway: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPlaneAccessIpv4Gateway"),
            ),
            user_plane_access_ipv4_subnet: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPlaneAccessIpv4Subnet"),
            ),
            user_plane_access_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPlaneAccessName"),
            ),
        }
    }
}
