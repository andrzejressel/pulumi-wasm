/// Manages a Mobile Network Packet Core Control Plane.
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
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkSite:
///     type: azure:mobile:NetworkSite
///     name: example
///     properties:
///       name: example-mns
///       mobileNetworkId: ${test.id}
///       location: ${example.location}
///   exampleDevice:
///     type: azure:databoxedge:Device
///     name: example
///     properties:
///       name: example-device
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: EdgeP_Base-Standard
///   exampleNetworkPacketCoreControlPlane:
///     type: azure:mobile:NetworkPacketCoreControlPlane
///     name: example
///     properties:
///       name: example-mnpccp
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: G0
///       controlPlaneAccessName: default-interface
///       controlPlaneAccessIpv4Address: 192.168.1.199
///       controlPlaneAccessIpv4Gateway: 192.168.1.1
///       controlPlaneAccessIpv4Subnet: 192.168.1.0/25
///       siteIds:
///         - ${exampleNetworkSite.id}
///       localDiagnosticsAccess:
///         authenticationType: AAD
///       platform:
///         type: AKS-HCI
///         edgeDeviceId: ${exampleDevice.id}
///       interoperabilitySettingsJson:
///         fn::toJSON:
///           key: value
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Packet Core Control Plane can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkPacketCoreControlPlane:NetworkPacketCoreControlPlane example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/packetCoreControlPlanes/packetCoreControlPlane1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_packet_core_control_plane {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkPacketCoreControlPlaneArgs {
        /// The IPv4 address for the control plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub control_plane_access_ipv4_address: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The default IPv4 gateway for the control plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub control_plane_access_ipv4_gateway: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The IPv4 subnet for the control plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub control_plane_access_ipv4_subnet: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the logical name for this interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        #[builder(into, default)]
        pub control_plane_access_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The core network technology generation. Possible values are `5GC` and `EPC`.
        #[builder(into, default)]
        pub core_network_technology: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mobile::NetworkPacketCoreControlPlaneIdentity>,
        >,
        /// Settings in JSON format to allow interoperability with third party components e.g. RANs and UEs.
        #[builder(into, default)]
        pub interoperability_settings_json: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// One or more `local_diagnostics_access` blocks as defined below. Specifies the Kubernetes ingress configuration that controls access to the packet core diagnostics through local APIs.
        #[builder(into)]
        pub local_diagnostics_access: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mobile::NetworkPacketCoreControlPlaneLocalDiagnosticsAccess,
        >,
        /// Specifies the Azure Region where the Mobile Network Packet Core Control Plane should exist. Changing this forces a new Mobile Network Packet Core Control Plane to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies The name of the Mobile Network Packet Core Control Plane. Changing this forces a new Mobile Network Packet Core Control Plane to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `platform` block as defined below.
        #[builder(into, default)]
        pub platform: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mobile::NetworkPacketCoreControlPlanePlatform>,
        >,
        /// Specifies the name of the Resource Group where the Mobile Network Packet Core Control Plane should exist. Changing this forces a new Mobile Network Packet Core Control Plane to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of Mobile Network Site IDs in which this packet core control plane should be deployed. The Sites must be in the same location as the packet core control plane.
        #[builder(into)]
        pub site_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The SKU defining the throughput and SIM allowances for this packet core control plane deployment. Possible values are `G0`, `G1`, `G2`, `G3`, `G4`, `G5` and `G10`.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the version of the packet core software that is deployed.
        #[builder(into, default)]
        pub software_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Control Plane.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the MTU in bytes that can be sent to the user equipment. The same MTU is set on the user plane data links for all data networks. The MTU set on the user plane access link will be 60 bytes greater than this value to allow for GTP encapsulation.
        #[builder(into, default)]
        pub user_equipment_mtu_in_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct NetworkPacketCoreControlPlaneResult {
        /// The IPv4 address for the control plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub control_plane_access_ipv4_address: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The default IPv4 gateway for the control plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub control_plane_access_ipv4_gateway: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The IPv4 subnet for the control plane interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub control_plane_access_ipv4_subnet: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the logical name for this interface. This should match one of the interfaces configured on your Azure Stack Edge device.
        pub control_plane_access_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The core network technology generation. Possible values are `5GC` and `EPC`.
        pub core_network_technology: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::mobile::NetworkPacketCoreControlPlaneIdentity>,
        >,
        /// Settings in JSON format to allow interoperability with third party components e.g. RANs and UEs.
        pub interoperability_settings_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `local_diagnostics_access` blocks as defined below. Specifies the Kubernetes ingress configuration that controls access to the packet core diagnostics through local APIs.
        pub local_diagnostics_access: pulumi_gestalt_rust::Output<
            super::super::types::mobile::NetworkPacketCoreControlPlaneLocalDiagnosticsAccess,
        >,
        /// Specifies the Azure Region where the Mobile Network Packet Core Control Plane should exist. Changing this forces a new Mobile Network Packet Core Control Plane to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies The name of the Mobile Network Packet Core Control Plane. Changing this forces a new Mobile Network Packet Core Control Plane to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `platform` block as defined below.
        pub platform: pulumi_gestalt_rust::Output<
            Option<super::super::types::mobile::NetworkPacketCoreControlPlanePlatform>,
        >,
        /// Specifies the name of the Resource Group where the Mobile Network Packet Core Control Plane should exist. Changing this forces a new Mobile Network Packet Core Control Plane to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of Mobile Network Site IDs in which this packet core control plane should be deployed. The Sites must be in the same location as the packet core control plane.
        pub site_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The SKU defining the throughput and SIM allowances for this packet core control plane deployment. Possible values are `G0`, `G1`, `G2`, `G3`, `G4`, `G5` and `G10`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of the packet core software that is deployed.
        pub software_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Mobile Network Packet Core Control Plane.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the MTU in bytes that can be sent to the user equipment. The same MTU is set on the user plane data links for all data networks. The MTU set on the user plane access link will be 60 bytes greater than this value to allow for GTP encapsulation.
        pub user_equipment_mtu_in_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkPacketCoreControlPlaneArgs,
    ) -> NetworkPacketCoreControlPlaneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let control_plane_access_ipv4_address_binding = args
            .control_plane_access_ipv4_address
            .get_output(context);
        let control_plane_access_ipv4_gateway_binding = args
            .control_plane_access_ipv4_gateway
            .get_output(context);
        let control_plane_access_ipv4_subnet_binding = args
            .control_plane_access_ipv4_subnet
            .get_output(context);
        let control_plane_access_name_binding = args
            .control_plane_access_name
            .get_output(context);
        let core_network_technology_binding = args
            .core_network_technology
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let interoperability_settings_json_binding = args
            .interoperability_settings_json
            .get_output(context);
        let local_diagnostics_access_binding = args
            .local_diagnostics_access
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let platform_binding = args.platform.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let site_ids_binding = args.site_ids.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let software_version_binding = args.software_version.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_equipment_mtu_in_bytes_binding = args
            .user_equipment_mtu_in_bytes
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mobile/networkPacketCoreControlPlane:NetworkPacketCoreControlPlane"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneAccessIpv4Address".into(),
                    value: &control_plane_access_ipv4_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneAccessIpv4Gateway".into(),
                    value: &control_plane_access_ipv4_gateway_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneAccessIpv4Subnet".into(),
                    value: &control_plane_access_ipv4_subnet_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneAccessName".into(),
                    value: &control_plane_access_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coreNetworkTechnology".into(),
                    value: &core_network_technology_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interoperabilitySettingsJson".into(),
                    value: &interoperability_settings_json_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localDiagnosticsAccess".into(),
                    value: &local_diagnostics_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platform".into(),
                    value: &platform_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteIds".into(),
                    value: &site_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "softwareVersion".into(),
                    value: &software_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userEquipmentMtuInBytes".into(),
                    value: &user_equipment_mtu_in_bytes_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkPacketCoreControlPlaneResult {
            control_plane_access_ipv4_address: o
                .get_field("controlPlaneAccessIpv4Address"),
            control_plane_access_ipv4_gateway: o
                .get_field("controlPlaneAccessIpv4Gateway"),
            control_plane_access_ipv4_subnet: o
                .get_field("controlPlaneAccessIpv4Subnet"),
            control_plane_access_name: o.get_field("controlPlaneAccessName"),
            core_network_technology: o.get_field("coreNetworkTechnology"),
            identity: o.get_field("identity"),
            interoperability_settings_json: o.get_field("interoperabilitySettingsJson"),
            local_diagnostics_access: o.get_field("localDiagnosticsAccess"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            platform: o.get_field("platform"),
            resource_group_name: o.get_field("resourceGroupName"),
            site_ids: o.get_field("siteIds"),
            sku: o.get_field("sku"),
            software_version: o.get_field("softwareVersion"),
            tags: o.get_field("tags"),
            user_equipment_mtu_in_bytes: o.get_field("userEquipmentMtuInBytes"),
        }
    }
}
