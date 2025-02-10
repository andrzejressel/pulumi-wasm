/// Manages an Amazon API Gateway Version 2 route.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/welcome.html) for [WebSocket](https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-develop-routes.html) and [HTTP](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-routes.html) APIs.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api::create(
///         "example",
///         ApiArgs::builder()
///             .name("example-websocket-api")
///             .protocol_type("WEBSOCKET")
///             .route_selection_expression("$request.body.action")
///             .build_struct(),
///     );
///     let exampleRoute = route::create(
///         "exampleRoute",
///         RouteArgs::builder().api_id("${example.id}").route_key("$default").build_struct(),
///     );
/// }
/// ```
///
/// ### HTTP Proxy Integration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api::create(
///         "example",
///         ApiArgs::builder().name("example-http-api").protocol_type("HTTP").build_struct(),
///     );
///     let exampleIntegration = integration::create(
///         "exampleIntegration",
///         IntegrationArgs::builder()
///             .api_id("${example.id}")
///             .integration_method("ANY")
///             .integration_type("HTTP_PROXY")
///             .integration_uri("https://example.com/{proxy}")
///             .build_struct(),
///     );
///     let exampleRoute = route::create(
///         "exampleRoute",
///         RouteArgs::builder()
///             .api_id("${example.id}")
///             .route_key("ANY /example/{proxy+}")
///             .target("integrations/${exampleIntegration.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_route` using the API identifier and route identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/route:Route example aabbccddee/1122334
/// ```
/// -> __Note:__ The API Gateway managed route created as part of [_quick_create_](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-basic-concept.html#apigateway-definition-quick-create) cannot be imported.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Boolean whether an API key is required for the route. Defaults to `false`. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub api_key_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Authorization scopes supported by this route. The scopes are used with a JWT authorizer to authorize the method invocation.
        #[builder(into, default)]
        pub authorization_scopes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Authorization type for the route.
        /// For WebSocket APIs, valid values are `NONE` for open access, `AWS_IAM` for using AWS IAM permissions, and `CUSTOM` for using a Lambda authorizer.
        /// For HTTP APIs, valid values are `NONE` for open access, `JWT` for using JSON Web Tokens, `AWS_IAM` for using AWS IAM permissions, and `CUSTOM` for using a Lambda authorizer.
        /// Defaults to `NONE`.
        #[builder(into, default)]
        pub authorization_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the `aws.apigatewayv2.Authorizer` resource to be associated with this route.
        #[builder(into, default)]
        pub authorizer_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [model selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-model-selection-expressions) for the route. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub model_selection_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Operation name for the route. Must be between 1 and 64 characters in length.
        #[builder(into, default)]
        pub operation_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Request models for the route. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub request_models: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Request parameters for the route. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub request_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigatewayv2::RouteRequestParameter>>,
        >,
        /// Route key for the route. For HTTP APIs, the route key can be either `$default`, or a combination of an HTTP method and resource path, for example, `GET /pets`.
        #[builder(into)]
        pub route_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The [route response selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-response-selection-expressions) for the route. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub route_response_selection_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Target for the route, of the form `integrations/`*`IntegrationID`*, where *`IntegrationID`* is the identifier of an `aws.apigatewayv2.Integration` resource.
        #[builder(into, default)]
        pub target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// API identifier.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// Boolean whether an API key is required for the route. Defaults to `false`. Supported only for WebSocket APIs.
        pub api_key_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Authorization scopes supported by this route. The scopes are used with a JWT authorizer to authorize the method invocation.
        pub authorization_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Authorization type for the route.
        /// For WebSocket APIs, valid values are `NONE` for open access, `AWS_IAM` for using AWS IAM permissions, and `CUSTOM` for using a Lambda authorizer.
        /// For HTTP APIs, valid values are `NONE` for open access, `JWT` for using JSON Web Tokens, `AWS_IAM` for using AWS IAM permissions, and `CUSTOM` for using a Lambda authorizer.
        /// Defaults to `NONE`.
        pub authorization_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the `aws.apigatewayv2.Authorizer` resource to be associated with this route.
        pub authorizer_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [model selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-model-selection-expressions) for the route. Supported only for WebSocket APIs.
        pub model_selection_expression: pulumi_gestalt_rust::Output<Option<String>>,
        /// Operation name for the route. Must be between 1 and 64 characters in length.
        pub operation_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Request models for the route. Supported only for WebSocket APIs.
        pub request_models: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Request parameters for the route. Supported only for WebSocket APIs.
        pub request_parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigatewayv2::RouteRequestParameter>>,
        >,
        /// Route key for the route. For HTTP APIs, the route key can be either `$default`, or a combination of an HTTP method and resource path, for example, `GET /pets`.
        pub route_key: pulumi_gestalt_rust::Output<String>,
        /// The [route response selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-response-selection-expressions) for the route. Supported only for WebSocket APIs.
        pub route_response_selection_expression: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Target for the route, of the form `integrations/`*`IntegrationID`*, where *`IntegrationID`* is the identifier of an `aws.apigatewayv2.Integration` resource.
        pub target: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteArgs,
    ) -> RouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let api_key_required_binding = args.api_key_required.get_output(context);
        let authorization_scopes_binding = args.authorization_scopes.get_output(context);
        let authorization_type_binding = args.authorization_type.get_output(context);
        let authorizer_id_binding = args.authorizer_id.get_output(context);
        let model_selection_expression_binding = args
            .model_selection_expression
            .get_output(context);
        let operation_name_binding = args.operation_name.get_output(context);
        let request_models_binding = args.request_models.get_output(context);
        let request_parameters_binding = args.request_parameters.get_output(context);
        let route_key_binding = args.route_key.get_output(context);
        let route_response_selection_expression_binding = args
            .route_response_selection_expression
            .get_output(context);
        let target_binding = args.target.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/route:Route".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiKeyRequired".into(),
                    value: api_key_required_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationScopes".into(),
                    value: authorization_scopes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationType".into(),
                    value: authorization_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerId".into(),
                    value: authorizer_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modelSelectionExpression".into(),
                    value: model_selection_expression_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operationName".into(),
                    value: operation_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestModels".into(),
                    value: request_models_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestParameters".into(),
                    value: request_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeKey".into(),
                    value: route_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeResponseSelectionExpression".into(),
                    value: route_response_selection_expression_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: target_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteResult {
            api_id: o.get_field("apiId"),
            api_key_required: o.get_field("apiKeyRequired"),
            authorization_scopes: o.get_field("authorizationScopes"),
            authorization_type: o.get_field("authorizationType"),
            authorizer_id: o.get_field("authorizerId"),
            model_selection_expression: o.get_field("modelSelectionExpression"),
            operation_name: o.get_field("operationName"),
            request_models: o.get_field("requestModels"),
            request_parameters: o.get_field("requestParameters"),
            route_key: o.get_field("routeKey"),
            route_response_selection_expression: o
                .get_field("routeResponseSelectionExpression"),
            target: o.get_field("target"),
        }
    }
}
