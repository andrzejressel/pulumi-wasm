#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_packet_core_control_plane {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreControlPlaneArgs {
        /// The name of the Mobile Network Packet Core Control Plane.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Mobile Network Packet Core Control Plane exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreControlPlaneResult {
        pub control_plane_access_ipv4_address: pulumi_gestalt_rust::Output<String>,
        pub control_plane_access_ipv4_gateway: pulumi_gestalt_rust::Output<String>,
        pub control_plane_access_ipv4_subnet: pulumi_gestalt_rust::Output<String>,
        pub control_plane_access_name: pulumi_gestalt_rust::Output<String>,
        /// The core network technology generation.
        pub core_network_technology: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkPacketCoreControlPlaneIdentity,
            >,
        >,
        /// Settings in JSON format to allow interoperability with third party components e.g. RANs and UEs.
        pub interoperability_settings_json: pulumi_gestalt_rust::Output<String>,
        /// One or more `local_diagnostics_access` blocks as defined below. The Kubernetes ingress configuration that controls access to the packet core diagnostics through local APIs.
        pub local_diagnostics_accesses: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkPacketCoreControlPlaneLocalDiagnosticsAccess,
            >,
        >,
        /// The Azure Region where the Mobile Network Packet Core Control Plane exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The logical name for this interface.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `platform` block as defined below.
        pub platforms: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkPacketCoreControlPlanePlatform,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The list of Mobile Network Site IDs in which this packet core control plane is deployed.
        pub site_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The SKU defining the throughput and SIM allowances for this packet core control plane deployment.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// The version of the packet core software that is deployed.
        pub software_version: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Mobile Network Packet Core Control Plane.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The MTU in bytes that can be sent to the user equipment.
        pub user_equipment_mtu_in_bytes: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkPacketCoreControlPlaneArgs,
    ) -> GetNetworkPacketCoreControlPlaneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetworkPacketCoreControlPlane:getNetworkPacketCoreControlPlane"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkPacketCoreControlPlaneResult {
            control_plane_access_ipv4_address: o
                .get_field("controlPlaneAccessIpv4Address"),
            control_plane_access_ipv4_gateway: o
                .get_field("controlPlaneAccessIpv4Gateway"),
            control_plane_access_ipv4_subnet: o
                .get_field("controlPlaneAccessIpv4Subnet"),
            control_plane_access_name: o.get_field("controlPlaneAccessName"),
            core_network_technology: o.get_field("coreNetworkTechnology"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            interoperability_settings_json: o.get_field("interoperabilitySettingsJson"),
            local_diagnostics_accesses: o.get_field("localDiagnosticsAccesses"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            platforms: o.get_field("platforms"),
            resource_group_name: o.get_field("resourceGroupName"),
            site_ids: o.get_field("siteIds"),
            sku: o.get_field("sku"),
            software_version: o.get_field("softwareVersion"),
            tags: o.get_field("tags"),
            user_equipment_mtu_in_bytes: o.get_field("userEquipmentMtuInBytes"),
        }
    }
}
