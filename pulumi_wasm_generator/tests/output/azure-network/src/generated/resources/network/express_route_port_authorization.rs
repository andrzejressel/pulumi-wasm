/// Manages an ExpressRoute Port Authorization.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("exprtTest")
///             .build_struct(),
///     );
///     let exampleExpressRoutePort = express_route_port::create(
///         "exampleExpressRoutePort",
///         ExpressRoutePortArgs::builder()
///             .bandwidth_in_gbps(10)
///             .encapsulation("Dot1Q")
///             .location("${example.location}")
///             .name("port1")
///             .peering_location("Airtel-Chennai-CLS")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleExpressRoutePortAuthorization = express_route_port_authorization::create(
///         "exampleExpressRoutePortAuthorization",
///         ExpressRoutePortAuthorizationArgs::builder()
///             .express_route_port_name("${exampleExpressRoutePort.name}")
///             .name("exampleERCAuth")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ExpressRoute Port Authorizations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRoutePortAuthorization:ExpressRoutePortAuthorization auth1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/expressRoutePorts/myExpressPort/authorizations/auth1
/// ```
///
pub mod express_route_port_authorization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRoutePortAuthorizationArgs {
        /// The name of the Express Route Port in which to create the Authorization. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_port_name: pulumi_wasm_rust::Output<String>,
        /// The name of the ExpressRoute Port. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute Port. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRoutePortAuthorizationResult {
        /// The Authorization Key.
        pub authorization_key: pulumi_wasm_rust::Output<String>,
        /// The authorization use status.
        pub authorization_use_status: pulumi_wasm_rust::Output<String>,
        /// The name of the Express Route Port in which to create the Authorization. Changing this forces a new resource to be created.
        pub express_route_port_name: pulumi_wasm_rust::Output<String>,
        /// The name of the ExpressRoute Port. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the ExpressRoute Port. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ExpressRoutePortAuthorizationArgs,
    ) -> ExpressRoutePortAuthorizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let express_route_port_name_binding = args.express_route_port_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRoutePortAuthorization:ExpressRoutePortAuthorization"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expressRoutePortName".into(),
                    value: &express_route_port_name_binding,
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
                    name: "expressRoutePortName".into(),
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
        ExpressRoutePortAuthorizationResult {
            authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationKey").unwrap(),
            ),
            authorization_use_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationUseStatus").unwrap(),
            ),
            express_route_port_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRoutePortName").unwrap(),
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
