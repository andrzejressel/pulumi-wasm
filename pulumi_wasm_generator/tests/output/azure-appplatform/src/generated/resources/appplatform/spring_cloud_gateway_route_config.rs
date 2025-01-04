/// Manages a Spring Cloud Gateway Route Config.
///
/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
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
///             .name("example")
///             .build_struct(),
///     );
///     let exampleSpringCloudApp = spring_cloud_app::create(
///         "exampleSpringCloudApp",
///         SpringCloudAppArgs::builder()
///             .name("example")
///             .resource_group_name("${example.name}")
///             .service_name("${exampleSpringCloudService.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudGateway = spring_cloud_gateway::create(
///         "exampleSpringCloudGateway",
///         SpringCloudGatewayArgs::builder()
///             .name("default")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudGatewayRouteConfig = spring_cloud_gateway_route_config::create(
///         "exampleSpringCloudGatewayRouteConfig",
///         SpringCloudGatewayRouteConfigArgs::builder()
///             .name("example")
///             .protocol("HTTPS")
///             .routes(
///                 vec![
///                     SpringCloudGatewayRouteConfigRoute::builder()
///                     .classificationTags(vec!["tag1", "tag2",])
///                     .description("example description").filters(vec!["StripPrefix=2",
///                     "RateLimit=1,1s",]).order(1)
///                     .predicates(vec!["Path=/api5/customer/**",])
///                     .ssoValidationEnabled(true).title("myApp route config")
///                     .tokenRelay(true).uri("https://www.example.com").build_struct(),
///                 ],
///             )
///             .spring_cloud_app_id("${exampleSpringCloudApp.id}")
///             .spring_cloud_gateway_id("${exampleSpringCloudGateway.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku_name("E0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Gateway Route Configs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudGatewayRouteConfig:SpringCloudGatewayRouteConfig example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/gateways/gateway1/routeConfigs/routeConfig1
/// ```
///
pub mod spring_cloud_gateway_route_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudGatewayRouteConfigArgs {
        /// Specifies a list of filters which are used to modify the request before sending it to the target endpoint, or the received response in app level.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Spring Cloud Gateway Route Config. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `open_api` blocks as defined below.
        #[builder(into, default)]
        pub open_api: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudGatewayRouteConfigOpenApi,
            >,
        >,
        /// Specifies a list of conditions to evaluate a route for each request in app level. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request.
        #[builder(into, default)]
        pub predicates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the protocol of routed Spring Cloud App. Allowed values are `HTTP` and `HTTPS`. Defaults to `HTTP`.
        ///
        /// > **Note:** You likely want to use `HTTPS` in a production environment, since `HTTP` offers no encryption.
        #[builder(into)]
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// One or more `route` blocks as defined below.
        #[builder(into, default)]
        pub routes: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appplatform::SpringCloudGatewayRouteConfigRoute>,
            >,
        >,
        /// The ID of the Spring Cloud App.
        #[builder(into, default)]
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Spring Cloud Gateway. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        #[builder(into)]
        pub spring_cloud_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Should the sso validation be enabled in app level?
        #[builder(into, default)]
        pub sso_validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudGatewayRouteConfigResult {
        /// Specifies a list of filters which are used to modify the request before sending it to the target endpoint, or the received response in app level.
        pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Spring Cloud Gateway Route Config. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `open_api` blocks as defined below.
        pub open_api: pulumi_wasm_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudGatewayRouteConfigOpenApi,
            >,
        >,
        /// Specifies a list of conditions to evaluate a route for each request in app level. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request.
        pub predicates: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the protocol of routed Spring Cloud App. Allowed values are `HTTP` and `HTTPS`. Defaults to `HTTP`.
        ///
        /// > **Note:** You likely want to use `HTTPS` in a production environment, since `HTTP` offers no encryption.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// One or more `route` blocks as defined below.
        pub routes: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::appplatform::SpringCloudGatewayRouteConfigRoute>,
            >,
        >,
        /// The ID of the Spring Cloud App.
        pub spring_cloud_app_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Spring Cloud Gateway. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        pub spring_cloud_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Should the sso validation be enabled in app level?
        pub sso_validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudGatewayRouteConfigArgs,
    ) -> SpringCloudGatewayRouteConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let name_binding = args.name.get_inner();
        let open_api_binding = args.open_api.get_inner();
        let predicates_binding = args.predicates.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let routes_binding = args.routes.get_inner();
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_inner();
        let spring_cloud_gateway_id_binding = args.spring_cloud_gateway_id.get_inner();
        let sso_validation_enabled_binding = args.sso_validation_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudGatewayRouteConfig:SpringCloudGatewayRouteConfig"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "openApi".into(),
                    value: &open_api_binding,
                },
                register_interface::ObjectField {
                    name: "predicates".into(),
                    value: &predicates_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "routes".into(),
                    value: &routes_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudGatewayId".into(),
                    value: &spring_cloud_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "ssoValidationEnabled".into(),
                    value: &sso_validation_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "openApi".into(),
                },
                register_interface::ResultField {
                    name: "predicates".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "routes".into(),
                },
                register_interface::ResultField {
                    name: "springCloudAppId".into(),
                },
                register_interface::ResultField {
                    name: "springCloudGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "ssoValidationEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudGatewayRouteConfigResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            open_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("openApi").unwrap(),
            ),
            predicates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("predicates").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routes").unwrap(),
            ),
            spring_cloud_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudAppId").unwrap(),
            ),
            spring_cloud_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudGatewayId").unwrap(),
            ),
            sso_validation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ssoValidationEnabled").unwrap(),
            ),
        }
    }
}
