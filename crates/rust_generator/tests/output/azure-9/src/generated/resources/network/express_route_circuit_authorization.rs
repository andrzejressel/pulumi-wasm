/// Manages an ExpressRoute Circuit Authorization.
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
///       allowClassicOperations: false
///       tags:
///         environment: Production
///   exampleExpressRouteCircuitAuthorization:
///     type: azure:network:ExpressRouteCircuitAuthorization
///     name: example
///     properties:
///       name: exampleERCAuth
///       expressRouteCircuitName: ${exampleExpressRouteCircuit.name}
///       resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// ExpressRoute Circuit Authorizations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRouteCircuitAuthorization:ExpressRouteCircuitAuthorization auth1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/expressRouteCircuits/myExpressRoute/authorizations/auth1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod express_route_circuit_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitAuthorizationArgs {
        /// The name of the Express Route Circuit in which to create the Authorization. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_circuit_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitAuthorizationResult {
        /// The Authorization Key.
        pub authorization_key: pulumi_gestalt_rust::Output<String>,
        /// The authorization use status.
        pub authorization_use_status: pulumi_gestalt_rust::Output<String>,
        /// The name of the Express Route Circuit in which to create the Authorization. Changing this forces a new resource to be created.
        pub express_route_circuit_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExpressRouteCircuitAuthorizationArgs,
    ) -> ExpressRouteCircuitAuthorizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let express_route_circuit_name_binding = args
            .express_route_circuit_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuitAuthorization:ExpressRouteCircuitAuthorization"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressRouteCircuitName".into(),
                    value: express_route_circuit_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExpressRouteCircuitAuthorizationResult {
            authorization_key: o.get_field("authorizationKey"),
            authorization_use_status: o.get_field("authorizationUseStatus"),
            express_route_circuit_name: o.get_field("expressRouteCircuitName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
