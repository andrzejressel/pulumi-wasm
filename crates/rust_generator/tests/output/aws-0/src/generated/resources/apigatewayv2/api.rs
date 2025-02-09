/// Manages an Amazon API Gateway Version 2 API.
///
/// > **Note:** Amazon API Gateway Version 2 resources are used for creating and deploying WebSocket and HTTP APIs. To create and deploy REST APIs, use Amazon API Gateway Version 1 resources.
///
/// ## Example Usage
///
/// ### Basic WebSocket API
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
/// }
/// ```
///
/// ### Basic HTTP API
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiArgs {
        /// An [API key selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions).
        /// Valid values: `$context.authorizer.usageIdentifierKey`, `$request.header.x-api-key`. Defaults to `$request.header.x-api-key`.
        /// Applicable for WebSocket APIs.
        #[builder(into, default)]
        pub api_key_selection_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// An OpenAPI specification that defines the set of routes and integrations to create as part of the HTTP APIs. Supported only for HTTP APIs.
        #[builder(into, default)]
        pub body: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Cross-origin resource sharing (CORS) [configuration](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html). Applicable for HTTP APIs.
        #[builder(into, default)]
        pub cors_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigatewayv2::ApiCorsConfiguration>,
        >,
        /// Part of _quick create_. Specifies any credentials required for the integration. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub credentials_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the API. Must be less than or equal to 1024 characters in length.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether clients can invoke the API by using the default `execute-api` endpoint.
        /// By default, clients can invoke the API with the default `{api_id}.execute-api.{region}.amazonaws.com endpoint`.
        /// To require that clients use a custom domain name to invoke the API, disable the default endpoint.
        #[builder(into, default)]
        pub disable_execute_api_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether warnings should return an error while API Gateway is creating or updating the resource using an OpenAPI specification. Defaults to `false`. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub fail_on_warnings: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the API. Must be less than or equal to 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// API protocol. Valid values: `HTTP`, `WEBSOCKET`.
        #[builder(into)]
        pub protocol_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Part of _quick create_. Specifies any [route key](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-routes.html). Applicable for HTTP APIs.
        #[builder(into, default)]
        pub route_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [route selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-selection-expressions) for the API.
        /// Defaults to `$request.method $request.path`.
        #[builder(into, default)]
        pub route_selection_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Map of tags to assign to the API. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Part of _quick create_. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes.
        /// For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN.
        /// The type of the integration will be `HTTP_PROXY` or `AWS_PROXY`, respectively. Applicable for HTTP APIs.
        #[builder(into, default)]
        pub target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version identifier for the API. Must be between 1 and 64 characters in length.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiResult {
        /// URI of the API, of the form `https://{api-id}.execute-api.{region}.amazonaws.com` for HTTP APIs and `wss://{api-id}.execute-api.{region}.amazonaws.com` for WebSocket APIs.
        pub api_endpoint: pulumi_gestalt_rust::Output<String>,
        /// An [API key selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions).
        /// Valid values: `$context.authorizer.usageIdentifierKey`, `$request.header.x-api-key`. Defaults to `$request.header.x-api-key`.
        /// Applicable for WebSocket APIs.
        pub api_key_selection_expression: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the API.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// An OpenAPI specification that defines the set of routes and integrations to create as part of the HTTP APIs. Supported only for HTTP APIs.
        pub body: pulumi_gestalt_rust::Output<Option<String>>,
        /// Cross-origin resource sharing (CORS) [configuration](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html). Applicable for HTTP APIs.
        pub cors_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigatewayv2::ApiCorsConfiguration>,
        >,
        /// Part of _quick create_. Specifies any credentials required for the integration. Applicable for HTTP APIs.
        pub credentials_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the API. Must be less than or equal to 1024 characters in length.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether clients can invoke the API by using the default `execute-api` endpoint.
        /// By default, clients can invoke the API with the default `{api_id}.execute-api.{region}.amazonaws.com endpoint`.
        /// To require that clients use a custom domain name to invoke the API, disable the default endpoint.
        pub disable_execute_api_endpoint: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN prefix to be used in an `aws.lambda.Permission`'s `source_arn` attribute
        /// or in an `aws.iam.Policy` to authorize access to the [`@connections` API](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-how-to-call-websocket-api-connections.html).
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-control-access-iam.html) for details.
        pub execution_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether warnings should return an error while API Gateway is creating or updating the resource using an OpenAPI specification. Defaults to `false`. Applicable for HTTP APIs.
        pub fail_on_warnings: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the API. Must be less than or equal to 128 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// API protocol. Valid values: `HTTP`, `WEBSOCKET`.
        pub protocol_type: pulumi_gestalt_rust::Output<String>,
        /// Part of _quick create_. Specifies any [route key](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-routes.html). Applicable for HTTP APIs.
        pub route_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [route selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-route-selection-expressions) for the API.
        /// Defaults to `$request.method $request.path`.
        pub route_selection_expression: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the API. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Part of _quick create_. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes.
        /// For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN.
        /// The type of the integration will be `HTTP_PROXY` or `AWS_PROXY`, respectively. Applicable for HTTP APIs.
        pub target: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version identifier for the API. Must be between 1 and 64 characters in length.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApiArgs,
    ) -> ApiResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_key_selection_expression_binding_1 = args
            .api_key_selection_expression
            .get_output(context);
        let api_key_selection_expression_binding = api_key_selection_expression_binding_1
            .get_inner();
        let body_binding_1 = args.body.get_output(context);
        let body_binding = body_binding_1.get_inner();
        let cors_configuration_binding_1 = args.cors_configuration.get_output(context);
        let cors_configuration_binding = cors_configuration_binding_1.get_inner();
        let credentials_arn_binding_1 = args.credentials_arn.get_output(context);
        let credentials_arn_binding = credentials_arn_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let disable_execute_api_endpoint_binding_1 = args
            .disable_execute_api_endpoint
            .get_output(context);
        let disable_execute_api_endpoint_binding = disable_execute_api_endpoint_binding_1
            .get_inner();
        let fail_on_warnings_binding_1 = args.fail_on_warnings.get_output(context);
        let fail_on_warnings_binding = fail_on_warnings_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let protocol_type_binding_1 = args.protocol_type.get_output(context);
        let protocol_type_binding = protocol_type_binding_1.get_inner();
        let route_key_binding_1 = args.route_key.get_output(context);
        let route_key_binding = route_key_binding_1.get_inner();
        let route_selection_expression_binding_1 = args
            .route_selection_expression
            .get_output(context);
        let route_selection_expression_binding = route_selection_expression_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let target_binding_1 = args.target.get_output(context);
        let target_binding = target_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiResult {
            api_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiEndpoint"),
            ),
            api_key_selection_expression: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKeySelectionExpression"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            body: pulumi_gestalt_rust::__private::into_domain(o.extract_field("body")),
            cors_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("corsConfiguration"),
            ),
            credentials_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("credentialsArn"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disable_execute_api_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableExecuteApiEndpoint"),
            ),
            execution_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executionArn"),
            ),
            fail_on_warnings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failOnWarnings"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            protocol_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocolType"),
            ),
            route_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routeKey"),
            ),
            route_selection_expression: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routeSelectionExpression"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("target"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
