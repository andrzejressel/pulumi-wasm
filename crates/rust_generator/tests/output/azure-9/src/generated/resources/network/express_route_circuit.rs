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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod express_route_circuit {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitArgs {
        /// Allow the circuit to interact with classic (RDFE) resources. Defaults to `false`.
        #[builder(into, default)]
        pub allow_classic_operations: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The authorization key. This can be used to set up an ExpressRoute Circuit with an ExpressRoute Port from another subscription.
        #[builder(into, default)]
        pub authorization_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The bandwidth in Gbps of the circuit being created on the Express Route Port.
        ///
        /// > **NOTE:** The `express_route_port_id` and the `bandwidth_in_gbps` should be set together and they conflict with `service_provider_name`, `peering_location` and `bandwidth_in_mbps`.
        #[builder(into, default)]
        pub bandwidth_in_gbps: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The bandwidth in Mbps of the circuit being created on the Service Provider.
        ///
        /// > **NOTE:** Once you increase your bandwidth, you will not be able to decrease it to its previous value.
        ///
        /// > **NOTE:** The `service_provider_name`, the `peering_location` and the `bandwidth_in_mbps` should be set together and they conflict with `express_route_port_id` and `bandwidth_in_gbps`.
        #[builder(into, default)]
        pub bandwidth_in_mbps: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Express Route Port this Express Route Circuit is based on. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub express_route_port_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the peering location and **not** the Azure resource location. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub peering_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the ExpressRoute Service Provider. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub service_provider_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `sku` block for the ExpressRoute circuit as documented below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::network::ExpressRouteCircuitSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitResult {
        /// Allow the circuit to interact with classic (RDFE) resources. Defaults to `false`.
        pub allow_classic_operations: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The authorization key. This can be used to set up an ExpressRoute Circuit with an ExpressRoute Port from another subscription.
        pub authorization_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The bandwidth in Gbps of the circuit being created on the Express Route Port.
        ///
        /// > **NOTE:** The `express_route_port_id` and the `bandwidth_in_gbps` should be set together and they conflict with `service_provider_name`, `peering_location` and `bandwidth_in_mbps`.
        pub bandwidth_in_gbps: pulumi_gestalt_rust::Output<Option<f64>>,
        /// The bandwidth in Mbps of the circuit being created on the Service Provider.
        ///
        /// > **NOTE:** Once you increase your bandwidth, you will not be able to decrease it to its previous value.
        ///
        /// > **NOTE:** The `service_provider_name`, the `peering_location` and the `bandwidth_in_mbps` should be set together and they conflict with `express_route_port_id` and `bandwidth_in_gbps`.
        pub bandwidth_in_mbps: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Express Route Port this Express Route Circuit is based on. Changing this forces a new resource to be created.
        pub express_route_port_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the peering location and **not** the Azure resource location. Changing this forces a new resource to be created.
        pub peering_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The string needed by the service provider to provision the ExpressRoute circuit.
        pub service_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the ExpressRoute Service Provider. Changing this forces a new resource to be created.
        pub service_provider_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ExpressRoute circuit provisioning state from your chosen service provider. Possible values are `NotProvisioned`, `Provisioning`, `Provisioned`, and `Deprovisioning`.
        pub service_provider_provisioning_state: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block for the ExpressRoute circuit as documented below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::types::network::ExpressRouteCircuitSku,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExpressRouteCircuitArgs,
    ) -> ExpressRouteCircuitResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_classic_operations_binding = args
            .allow_classic_operations
            .get_output(context);
        let authorization_key_binding = args.authorization_key.get_output(context);
        let bandwidth_in_gbps_binding = args.bandwidth_in_gbps.get_output(context);
        let bandwidth_in_mbps_binding = args.bandwidth_in_mbps.get_output(context);
        let express_route_port_id_binding = args
            .express_route_port_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let peering_location_binding = args.peering_location.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_provider_name_binding = args
            .service_provider_name
            .get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuit:ExpressRouteCircuit".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowClassicOperations".into(),
                    value: allow_classic_operations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationKey".into(),
                    value: authorization_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bandwidthInGbps".into(),
                    value: bandwidth_in_gbps_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bandwidthInMbps".into(),
                    value: bandwidth_in_mbps_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressRoutePortId".into(),
                    value: express_route_port_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peeringLocation".into(),
                    value: peering_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceProviderName".into(),
                    value: service_provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExpressRouteCircuitResult {
            allow_classic_operations: o.get_field("allowClassicOperations"),
            authorization_key: o.get_field("authorizationKey"),
            bandwidth_in_gbps: o.get_field("bandwidthInGbps"),
            bandwidth_in_mbps: o.get_field("bandwidthInMbps"),
            express_route_port_id: o.get_field("expressRoutePortId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            peering_location: o.get_field("peeringLocation"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_key: o.get_field("serviceKey"),
            service_provider_name: o.get_field("serviceProviderName"),
            service_provider_provisioning_state: o
                .get_field("serviceProviderProvisioningState"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
