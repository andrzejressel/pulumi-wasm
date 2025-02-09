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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ExpressRouteCircuitAuthorizationArgs,
    ) -> ExpressRouteCircuitAuthorizationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let express_route_circuit_name_binding_1 = args
            .express_route_circuit_name
            .get_output(context);
        let express_route_circuit_name_binding = express_route_circuit_name_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuitAuthorization:ExpressRouteCircuitAuthorization"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expressRouteCircuitName".into(),
                    value: &express_route_circuit_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExpressRouteCircuitAuthorizationResult {
            authorization_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizationKey"),
            ),
            authorization_use_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizationUseStatus"),
            ),
            express_route_circuit_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expressRouteCircuitName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
