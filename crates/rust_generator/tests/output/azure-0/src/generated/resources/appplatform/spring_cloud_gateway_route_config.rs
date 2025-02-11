/// Manages a Spring Cloud Gateway Route Config.
///
/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_gateway_route_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudGatewayRouteConfigArgs {
        /// Specifies a list of filters which are used to modify the request before sending it to the target endpoint, or the received response in app level.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name which should be used for this Spring Cloud Gateway Route Config. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `open_api` blocks as defined below.
        #[builder(into, default)]
        pub open_api: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::appplatform::SpringCloudGatewayRouteConfigOpenApi,
            >,
        >,
        /// Specifies a list of conditions to evaluate a route for each request in app level. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request.
        #[builder(into, default)]
        pub predicates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the protocol of routed Spring Cloud App. Allowed values are `HTTP` and `HTTPS`. Defaults to `HTTP`.
        ///
        /// > **Note:** You likely want to use `HTTPS` in a production environment, since `HTTP` offers no encryption.
        #[builder(into)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `route` blocks as defined below.
        #[builder(into, default)]
        pub routes: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::appplatform::SpringCloudGatewayRouteConfigRoute>,
            >,
        >,
        /// The ID of the Spring Cloud App.
        #[builder(into, default)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Gateway. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        #[builder(into)]
        pub spring_cloud_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the sso validation be enabled in app level?
        #[builder(into, default)]
        pub sso_validation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudGatewayRouteConfigResult {
        /// Specifies a list of filters which are used to modify the request before sending it to the target endpoint, or the received response in app level.
        pub filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Spring Cloud Gateway Route Config. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `open_api` blocks as defined below.
        pub open_api: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::appplatform::SpringCloudGatewayRouteConfigOpenApi,
            >,
        >,
        /// Specifies a list of conditions to evaluate a route for each request in app level. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request.
        pub predicates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the protocol of routed Spring Cloud App. Allowed values are `HTTP` and `HTTPS`. Defaults to `HTTP`.
        ///
        /// > **Note:** You likely want to use `HTTPS` in a production environment, since `HTTP` offers no encryption.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// One or more `route` blocks as defined below.
        pub routes: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::appplatform::SpringCloudGatewayRouteConfigRoute>,
            >,
        >,
        /// The ID of the Spring Cloud App.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Spring Cloud Gateway. Changing this forces a new Spring Cloud Gateway Route Config to be created.
        pub spring_cloud_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Should the sso validation be enabled in app level?
        pub sso_validation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudGatewayRouteConfigArgs,
    ) -> SpringCloudGatewayRouteConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let open_api_binding = args.open_api.get_output(context);
        let predicates_binding = args.predicates.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let routes_binding = args.routes.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
        let spring_cloud_gateway_id_binding = args
            .spring_cloud_gateway_id
            .get_output(context);
        let sso_validation_enabled_binding = args
            .sso_validation_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudGatewayRouteConfig:SpringCloudGatewayRouteConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "openApi".into(),
                    value: &open_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "predicates".into(),
                    value: &predicates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routes".into(),
                    value: &routes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudGatewayId".into(),
                    value: &spring_cloud_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ssoValidationEnabled".into(),
                    value: &sso_validation_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudGatewayRouteConfigResult {
            filters: o.get_field("filters"),
            name: o.get_field("name"),
            open_api: o.get_field("openApi"),
            predicates: o.get_field("predicates"),
            protocol: o.get_field("protocol"),
            routes: o.get_field("routes"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
            spring_cloud_gateway_id: o.get_field("springCloudGatewayId"),
            sso_validation_enabled: o.get_field("ssoValidationEnabled"),
        }
    }
}
