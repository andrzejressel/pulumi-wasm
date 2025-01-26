pub mod get_circuit_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCircuitPeeringArgs {
        /// The name of the ExpressRoute Circuit in which to create the Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_circuit_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`.
        #[builder(into)]
        pub peering_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the Express Route Circuit Peering. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCircuitPeeringResult {
        /// The ASN used by Azure for the peering.
        pub azure_asn: pulumi_wasm_rust::Output<i32>,
        pub express_route_circuit_name: pulumi_wasm_rust::Output<String>,
        pub gateway_manager_etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Indicates if IPv4 is enabled.
        pub ipv4_enabled: pulumi_wasm_rust::Output<bool>,
        pub peer_asn: pulumi_wasm_rust::Output<i32>,
        /// The type of the ExpressRoute Circuit Peering.
        pub peering_type: pulumi_wasm_rust::Output<String>,
        /// The primary port used by Azure for this peering.
        pub primary_azure_port: pulumi_wasm_rust::Output<String>,
        /// The primary peer address prefix.
        pub primary_peer_address_prefix: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub route_filter_id: pulumi_wasm_rust::Output<String>,
        /// The secondary port used by Azure for this peering.
        pub secondary_azure_port: pulumi_wasm_rust::Output<String>,
        /// The secondary peer address prefix.
        pub secondary_peer_address_prefix: pulumi_wasm_rust::Output<String>,
        pub shared_key: pulumi_wasm_rust::Output<String>,
        /// The VLAN ID used for this peering.
        pub vlan_id: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCircuitPeeringArgs,
    ) -> GetCircuitPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let express_route_circuit_name_binding = args
            .express_route_circuit_name
            .get_output(context)
            .get_inner();
        let peering_type_binding = args.peering_type.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:expressroute/getCircuitPeering:getCircuitPeering".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expressRouteCircuitName".into(),
                    value: &express_route_circuit_name_binding,
                },
                register_interface::ObjectField {
                    name: "peeringType".into(),
                    value: &peering_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "azureAsn".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteCircuitName".into(),
                },
                register_interface::ResultField {
                    name: "gatewayManagerEtag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipv4Enabled".into(),
                },
                register_interface::ResultField {
                    name: "peerAsn".into(),
                },
                register_interface::ResultField {
                    name: "peeringType".into(),
                },
                register_interface::ResultField {
                    name: "primaryAzurePort".into(),
                },
                register_interface::ResultField {
                    name: "primaryPeerAddressPrefix".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "routeFilterId".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAzurePort".into(),
                },
                register_interface::ResultField {
                    name: "secondaryPeerAddressPrefix".into(),
                },
                register_interface::ResultField {
                    name: "sharedKey".into(),
                },
                register_interface::ResultField {
                    name: "vlanId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCircuitPeeringResult {
            azure_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureAsn").unwrap(),
            ),
            express_route_circuit_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteCircuitName").unwrap(),
            ),
            gateway_manager_etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayManagerEtag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipv4_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4Enabled").unwrap(),
            ),
            peer_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAsn").unwrap(),
            ),
            peering_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peeringType").unwrap(),
            ),
            primary_azure_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAzurePort").unwrap(),
            ),
            primary_peer_address_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryPeerAddressPrefix").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            route_filter_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeFilterId").unwrap(),
            ),
            secondary_azure_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAzurePort").unwrap(),
            ),
            secondary_peer_address_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryPeerAddressPrefix").unwrap(),
            ),
            shared_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedKey").unwrap(),
            ),
            vlan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlanId").unwrap(),
            ),
        }
    }
}
