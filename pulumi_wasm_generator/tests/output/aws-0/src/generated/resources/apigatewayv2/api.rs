/// Manages an Amazon API Gateway Version 2 API.
///
/// > **Note:** Amazon API Gateway Version 2 resources are used for creating and deploying WebSocket and HTTP APIs. To create and deploy REST APIs, use Amazon API Gateway Version 1 resources.
///
/// ## Example Usage
///
/// ### Basic WebSocket API
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// }
/// ```
///
/// ### Basic HTTP API
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api::create(
///         "example",
///         ApiArgs::builder().name("example-http-api").protocol_type("HTTP").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_api` using the API identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/api:Api example aabbccddee
/// ```
pub mod api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiArgs {
        /// An [API key selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions).
        /// Valid values: `$context.authorizer.usageIdentifierKey`, `$request.header.x-api-key`. Defaults to `$request.header.x-api-key`.
        /// Applicable for WebSocket APIs.
        #[builder(into, default)]
        pub api_key_selection_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// An OpenAPI specification that defines the set of routes and integrations to create as part of the HTTP APIs. Supported only for HTTP APIs.
        #[builder(into, default)]
        pub body: pulumi_wasm_rust::Output<Option<String>>,
        /// Cross-origin resource sharing (CORS) [configuration](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html). Applicable for HTTP APIs.
        #[builder(into, default)]
        pub cors_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::ApiCorsConfiguration>,
        >,
        /// Part of _quick create_. Specifies any credentials required for the integration. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub credentials_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the API. Must be less than or equal to 1024 characters in length.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether clients can invoke the API by using the default `execute-api` endpoint.
        /// By default, clients can invoke the API with the default `{api_id}.execute-api.{region}.amazonaws.com endpoint`.
        /// To require that clients use a custom domain name to invoke the API, disable the default endpoint.
        #[builder(into, default)]
        pub disable_execute_api_endpoint: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether warnings should return an error while API Gateway is creating or updating the resource using an OpenAPI specification. Defaults to `false`. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub fail_on_warnings: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the API. Must be less than or equal to 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// API protocol. Valid values: `HTTP`, `WEBSOCKET`.
        #[builder(into)]
        pub protocol_type: pulumi_wasm_rust::Output<String>,
        /// Part of _quick create_. Specifies any [route key](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-routes.html). Applicable for HTTP APIs.
        #[builder(into, default)]
        pub route_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The [route selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-selection-expressions) for the API.
        /// Defaults to `$request.method $request.path`.
        #[builder(into, default)]
        pub route_selection_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the API. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Part of _quick create_. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes.
        /// For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN.
        /// The type of the integration will be `HTTP_PROXY` or `AWS_PROXY`, respectively. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub target: pulumi_wasm_rust::Output<Option<String>>,
        /// Version identifier for the API. Must be between 1 and 64 characters in length.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiResult {
        /// URI of the API, of the form `https://{api-id}.execute-api.{region}.amazonaws.com` for HTTP APIs and `wss://{api-id}.execute-api.{region}.amazonaws.com` for WebSocket APIs.
        pub api_endpoint: pulumi_wasm_rust::Output<String>,
        /// An [API key selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions).
        /// Valid values: `$context.authorizer.usageIdentifierKey`, `$request.header.x-api-key`. Defaults to `$request.header.x-api-key`.
        /// Applicable for WebSocket APIs.
        pub api_key_selection_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the API.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// An OpenAPI specification that defines the set of routes and integrations to create as part of the HTTP APIs. Supported only for HTTP APIs.
        pub body: pulumi_wasm_rust::Output<Option<String>>,
        /// Cross-origin resource sharing (CORS) [configuration](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html). Applicable for HTTP APIs.
        pub cors_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::ApiCorsConfiguration>,
        >,
        /// Part of _quick create_. Specifies any credentials required for the integration. Applicable for HTTP APIs.
        pub credentials_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the API. Must be less than or equal to 1024 characters in length.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether clients can invoke the API by using the default `execute-api` endpoint.
        /// By default, clients can invoke the API with the default `{api_id}.execute-api.{region}.amazonaws.com endpoint`.
        /// To require that clients use a custom domain name to invoke the API, disable the default endpoint.
        pub disable_execute_api_endpoint: pulumi_wasm_rust::Output<Option<bool>>,
        /// ARN prefix to be used in an `aws.lambda.Permission`'s `source_arn` attribute
        /// or in an `aws.iam.Policy` to authorize access to the [`@connections` API](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api-connections.html).
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-control-access-iam.html) for details.
        pub execution_arn: pulumi_wasm_rust::Output<String>,
        /// Whether warnings should return an error while API Gateway is creating or updating the resource using an OpenAPI specification. Defaults to `false`. Applicable for HTTP APIs.
        pub fail_on_warnings: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the API. Must be less than or equal to 128 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
        /// API protocol. Valid values: `HTTP`, `WEBSOCKET`.
        pub protocol_type: pulumi_wasm_rust::Output<String>,
        /// Part of _quick create_. Specifies any [route key](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-routes.html). Applicable for HTTP APIs.
        pub route_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The [route selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-selection-expressions) for the API.
        /// Defaults to `$request.method $request.path`.
        pub route_selection_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the API. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Part of _quick create_. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes.
        /// For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN.
        /// The type of the integration will be `HTTP_PROXY` or `AWS_PROXY`, respectively. Applicable for HTTP APIs.
        pub target: pulumi_wasm_rust::Output<Option<String>>,
        /// Version identifier for the API. Must be between 1 and 64 characters in length.
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiArgs) -> ApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_key_selection_expression_binding = args
            .api_key_selection_expression
            .get_inner();
        let body_binding = args.body.get_inner();
        let cors_configuration_binding = args.cors_configuration.get_inner();
        let credentials_arn_binding = args.credentials_arn.get_inner();
        let description_binding = args.description.get_inner();
        let disable_execute_api_endpoint_binding = args
            .disable_execute_api_endpoint
            .get_inner();
        let fail_on_warnings_binding = args.fail_on_warnings.get_inner();
        let name_binding = args.name.get_inner();
        let protocol_type_binding = args.protocol_type.get_inner();
        let route_key_binding = args.route_key.get_inner();
        let route_selection_expression_binding = args
            .route_selection_expression
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let target_binding = args.target.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/api:Api".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiKeySelectionExpression".into(),
                    value: &api_key_selection_expression_binding,
                },
                register_interface::ObjectField {
                    name: "body".into(),
                    value: &body_binding,
                },
                register_interface::ObjectField {
                    name: "corsConfiguration".into(),
                    value: &cors_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "credentialsArn".into(),
                    value: &credentials_arn_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disableExecuteApiEndpoint".into(),
                    value: &disable_execute_api_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "failOnWarnings".into(),
                    value: &fail_on_warnings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "protocolType".into(),
                    value: &protocol_type_binding,
                },
                register_interface::ObjectField {
                    name: "routeKey".into(),
                    value: &route_key_binding,
                },
                register_interface::ObjectField {
                    name: "routeSelectionExpression".into(),
                    value: &route_selection_expression_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "apiKeySelectionExpression".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "body".into(),
                },
                register_interface::ResultField {
                    name: "corsConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "credentialsArn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableExecuteApiEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "executionArn".into(),
                },
                register_interface::ResultField {
                    name: "failOnWarnings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protocolType".into(),
                },
                register_interface::ResultField {
                    name: "routeKey".into(),
                },
                register_interface::ResultField {
                    name: "routeSelectionExpression".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiResult {
            api_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiEndpoint").unwrap(),
            ),
            api_key_selection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeySelectionExpression").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("body").unwrap(),
            ),
            cors_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("corsConfiguration").unwrap(),
            ),
            credentials_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialsArn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_execute_api_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableExecuteApiEndpoint").unwrap(),
            ),
            execution_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionArn").unwrap(),
            ),
            fail_on_warnings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failOnWarnings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protocol_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocolType").unwrap(),
            ),
            route_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeKey").unwrap(),
            ),
            route_selection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeSelectionExpression").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
