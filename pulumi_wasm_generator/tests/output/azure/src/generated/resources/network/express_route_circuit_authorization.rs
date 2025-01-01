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
pub mod express_route_circuit_authorization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitAuthorizationArgs {
        /// The name of the Express Route Circuit in which to create the Authorization. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_circuit_name: pulumi_wasm_rust::Output<String>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteCircuitAuthorizationResult {
        /// The Authorization Key.
        pub authorization_key: pulumi_wasm_rust::Output<String>,
        /// The authorization use status.
        pub authorization_use_status: pulumi_wasm_rust::Output<String>,
        /// The name of the Express Route Circuit in which to create the Authorization. Changing this forces a new resource to be created.
        pub express_route_circuit_name: pulumi_wasm_rust::Output<String>,
        /// The name of the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the ExpressRoute circuit. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ExpressRouteCircuitAuthorizationArgs,
    ) -> ExpressRouteCircuitAuthorizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let express_route_circuit_name_binding = args
            .express_route_circuit_name
            .get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRouteCircuitAuthorization:ExpressRouteCircuitAuthorization"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizationKey".into(),
                },
                register_interface::ResultField {
                    name: "authorizationUseStatus".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteCircuitName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExpressRouteCircuitAuthorizationResult {
            authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationKey").unwrap(),
            ),
            authorization_use_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationUseStatus").unwrap(),
            ),
            express_route_circuit_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteCircuitName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
