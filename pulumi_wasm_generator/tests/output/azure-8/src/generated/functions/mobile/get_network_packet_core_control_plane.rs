pub mod get_network_packet_core_control_plane {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreControlPlaneArgs {
        /// The name of the Mobile Network Packet Core Control Plane.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Mobile Network Packet Core Control Plane exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPacketCoreControlPlaneResult {
        pub control_plane_access_ipv4_address: pulumi_wasm_rust::Output<String>,
        pub control_plane_access_ipv4_gateway: pulumi_wasm_rust::Output<String>,
        pub control_plane_access_ipv4_subnet: pulumi_wasm_rust::Output<String>,
        pub control_plane_access_name: pulumi_wasm_rust::Output<String>,
        /// The core network technology generation.
        pub core_network_technology: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkPacketCoreControlPlaneIdentity,
            >,
        >,
        /// Settings in JSON format to allow interoperability with third party components e.g. RANs and UEs.
        pub interoperability_settings_json: pulumi_wasm_rust::Output<String>,
        /// One or more `local_diagnostics_access` blocks as defined below. The Kubernetes ingress configuration that controls access to the packet core diagnostics through local APIs.
        pub local_diagnostics_accesses: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkPacketCoreControlPlaneLocalDiagnosticsAccess,
            >,
        >,
        /// The Azure Region where the Mobile Network Packet Core Control Plane exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The logical name for this interface.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `platform` block as defined below.
        pub platforms: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkPacketCoreControlPlanePlatform,
            >,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The list of Mobile Network Site IDs in which this packet core control plane is deployed.
        pub site_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The SKU defining the throughput and SIM allowances for this packet core control plane deployment.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// The version of the packet core software that is deployed.
        pub software_version: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Mobile Network Packet Core Control Plane.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The MTU in bytes that can be sent to the user equipment.
        pub user_equipment_mtu_in_bytes: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkPacketCoreControlPlaneArgs,
    ) -> GetNetworkPacketCoreControlPlaneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkPacketCoreControlPlane:getNetworkPacketCoreControlPlane"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "controlPlaneAccessIpv4Address".into(),
                },
                register_interface::ResultField {
                    name: "controlPlaneAccessIpv4Gateway".into(),
                },
                register_interface::ResultField {
                    name: "controlPlaneAccessIpv4Subnet".into(),
                },
                register_interface::ResultField {
                    name: "controlPlaneAccessName".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkTechnology".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "interoperabilitySettingsJson".into(),
                },
                register_interface::ResultField {
                    name: "localDiagnosticsAccesses".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platforms".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "siteIds".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "softwareVersion".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userEquipmentMtuInBytes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkPacketCoreControlPlaneResult {
            control_plane_access_ipv4_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneAccessIpv4Address").unwrap(),
            ),
            control_plane_access_ipv4_gateway: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneAccessIpv4Gateway").unwrap(),
            ),
            control_plane_access_ipv4_subnet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneAccessIpv4Subnet").unwrap(),
            ),
            control_plane_access_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneAccessName").unwrap(),
            ),
            core_network_technology: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkTechnology").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            interoperability_settings_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interoperabilitySettingsJson").unwrap(),
            ),
            local_diagnostics_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localDiagnosticsAccesses").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platforms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platforms").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            site_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("siteIds").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            software_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("softwareVersion").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_equipment_mtu_in_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userEquipmentMtuInBytes").unwrap(),
            ),
        }
    }
}
