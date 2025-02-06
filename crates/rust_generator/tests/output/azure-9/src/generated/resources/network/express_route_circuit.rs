/// Manages an ExpressRoute circuit.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: exprtTest
///       location: West Europe
///   exampleExpressRouteCircuit:
///     type: azure:network:ExpressRouteCircuit
///     name: example
///     properties:
///       name: expressRoute1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       serviceProviderName: Equinix
///       peeringLocation: Silicon Valley
///       bandwidthInMbps: 50
///       sku:
///         tier: Standard
///         family: MeteredData
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// ExpressRoute circuits can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRouteCircuit:ExpressRouteCircuit myExpressRoute /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/expressRouteCircuits/myExpressRoute
/// ```
///
pub mod express_route_circuit {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitArgs {
        /// Allow the circuit to interact with classic (RDFE) resources. Defaults to `false`.
        #[builder(into, default)]
        pub allow_classic_operations: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The authorization key. This can be used to set up an ExpressRoute Circuit with an ExpressRoute Port from another subscription.
        #[builder(into, default)]
        pub authorization_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The bandwidth in Gbps of the circuit being created on the Express Route Port.
        ///
        /// > **NOTE:** The `express_route_port_id` and the `bandwidth_in_gbps` should be set together and they conflict with `service_provider_name`, `peering_location` and `bandwidth_in_mbps`.
        #[builder(into, default)]
        pub bandwidth_in_gbps: pulumi_wasm_rust::InputOrOutput<Option<f64>>,
        /// The bandwidth in Mbps of the circuit being created on the Service Provider.
        ///
        /// > **NOTE:** Once you increase your bandwidth, you will not be able to decrease it to its previous value.
        ///
        /// > **NOTE:** The `service_provider_name`, the `peering_location` and the `bandwidth_in_mbps` should be set together and they conflict with `express_route_port_id` and `bandwidth_in_gbps`.
        #[builder(into, default)]
        pub bandwidth_in_mbps: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Express Route Port this Express Route Circuit is based on. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub express_route_port_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the peering location and **not** the Azure resource location. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub peering_location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the ExpressRoute Service Provider. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub service_provider_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `sku` block for the ExpressRoute circuit as documented below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::InputOrOutput<
            super::super::types::network::ExpressRouteCircuitSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitResult {
        /// Allow the circuit to interact with classic (RDFE) resources. Defaults to `false`.
        pub allow_classic_operations: pulumi_wasm_rust::Output<Option<bool>>,
        /// The authorization key. This can be used to set up an ExpressRoute Circuit with an ExpressRoute Port from another subscription.
        pub authorization_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The bandwidth in Gbps of the circuit being created on the Express Route Port.
        ///
        /// > **NOTE:** The `express_route_port_id` and the `bandwidth_in_gbps` should be set together and they conflict with `service_provider_name`, `peering_location` and `bandwidth_in_mbps`.
        pub bandwidth_in_gbps: pulumi_wasm_rust::Output<Option<f64>>,
        /// The bandwidth in Mbps of the circuit being created on the Service Provider.
        ///
        /// > **NOTE:** Once you increase your bandwidth, you will not be able to decrease it to its previous value.
        ///
        /// > **NOTE:** The `service_provider_name`, the `peering_location` and the `bandwidth_in_mbps` should be set together and they conflict with `express_route_port_id` and `bandwidth_in_gbps`.
        pub bandwidth_in_mbps: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the Express Route Port this Express Route Circuit is based on. Changing this forces a new resource to be created.
        pub express_route_port_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the peering location and **not** the Azure resource location. Changing this forces a new resource to be created.
        pub peering_location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The string needed by the service provider to provision the ExpressRoute circuit.
        pub service_key: pulumi_wasm_rust::Output<String>,
        /// The name of the ExpressRoute Service Provider. Changing this forces a new resource to be created.
        pub service_provider_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ExpressRoute circuit provisioning state from your chosen service provider. Possible values are `NotProvisioned`, `Provisioning`, `Provisioned`, and `Deprovisioning`.
        pub service_provider_provisioning_state: pulumi_wasm_rust::Output<String>,
        /// A `sku` block for the ExpressRoute circuit as documented below.
        pub sku: pulumi_wasm_rust::Output<
            super::super::types::network::ExpressRouteCircuitSku,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExpressRouteCircuitArgs,
    ) -> ExpressRouteCircuitResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_classic_operations_binding = args
            .allow_classic_operations
            .get_output(context)
            .get_inner();
        let authorization_key_binding = args
            .authorization_key
            .get_output(context)
            .get_inner();
        let bandwidth_in_gbps_binding = args
            .bandwidth_in_gbps
            .get_output(context)
            .get_inner();
        let bandwidth_in_mbps_binding = args
            .bandwidth_in_mbps
            .get_output(context)
            .get_inner();
        let express_route_port_id_binding = args
            .express_route_port_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let peering_location_binding = args
            .peering_location
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let service_provider_name_binding = args
            .service_provider_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuit:ExpressRouteCircuit".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowClassicOperations".into(),
                    value: &allow_classic_operations_binding,
                },
                register_interface::ObjectField {
                    name: "authorizationKey".into(),
                    value: &authorization_key_binding,
                },
                register_interface::ObjectField {
                    name: "bandwidthInGbps".into(),
                    value: &bandwidth_in_gbps_binding,
                },
                register_interface::ObjectField {
                    name: "bandwidthInMbps".into(),
                    value: &bandwidth_in_mbps_binding,
                },
                register_interface::ObjectField {
                    name: "expressRoutePortId".into(),
                    value: &express_route_port_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "peeringLocation".into(),
                    value: &peering_location_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceProviderName".into(),
                    value: &service_provider_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExpressRouteCircuitResult {
            allow_classic_operations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowClassicOperations"),
            ),
            authorization_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizationKey"),
            ),
            bandwidth_in_gbps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bandwidthInGbps"),
            ),
            bandwidth_in_mbps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bandwidthInMbps"),
            ),
            express_route_port_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expressRoutePortId"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            peering_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peeringLocation"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceKey"),
            ),
            service_provider_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceProviderName"),
            ),
            service_provider_provisioning_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceProviderProvisioningState"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
