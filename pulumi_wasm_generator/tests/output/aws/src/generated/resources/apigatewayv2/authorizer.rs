/// Manages an Amazon API Gateway Version 2 authorizer.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
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
///     let example = authorizer::create(
///         "example",
///         AuthorizerArgs::builder()
///             .api_id("${exampleAwsApigatewayv2Api.id}")
///             .authorizer_type("REQUEST")
///             .authorizer_uri("${exampleAwsLambdaFunction.invokeArn}")
///             .identity_sources(vec!["route.request.header.Auth",])
///             .name("example-authorizer")
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
///     let example = authorizer::create(
///         "example",
///         AuthorizerArgs::builder()
///             .api_id("${exampleAwsApigatewayv2Api.id}")
///             .authorizer_payload_format_version("2.0")
///             .authorizer_type("REQUEST")
///             .authorizer_uri("${exampleAwsLambdaFunction.invokeArn}")
///             .identity_sources(vec!["$request.header.Authorization",])
///             .name("example-authorizer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_authorizer` using the API identifier and authorizer identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/authorizer:Authorizer example aabbccddee/1122334
/// ```
pub mod authorizer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizerArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Required credentials as an IAM role for API Gateway to invoke the authorizer.
        /// Supported only for `REQUEST` authorizers.
        #[builder(into, default)]
        pub authorizer_credentials_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Format of the payload sent to an HTTP API Lambda authorizer. Required for HTTP API Lambda authorizers.
        /// Valid values: `1.0`, `2.0`.
        #[builder(into, default)]
        pub authorizer_payload_format_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Time to live (TTL) for cached authorizer results, in seconds. If it equals 0, authorization caching is disabled.
        /// If it is greater than 0, API Gateway caches authorizer responses. The maximum value is 3600, or 1 hour. Defaults to `300`.
        /// Supported only for HTTP API Lambda authorizers.
        #[builder(into, default)]
        pub authorizer_result_ttl_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Authorizer type. Valid values: `JWT`, `REQUEST`.
        /// Specify `REQUEST` for a Lambda function using incoming request parameters.
        /// For HTTP APIs, specify `JWT` to use JSON Web Tokens.
        #[builder(into)]
        pub authorizer_type: pulumi_wasm_rust::Output<String>,
        /// Authorizer's Uniform Resource Identifier (URI).
        /// For `REQUEST` authorizers this must be a well-formed Lambda function URI, such as the `invoke_arn` attribute of the `aws.lambda.Function` resource.
        /// Supported only for `REQUEST` authorizers. Must be between 1 and 2048 characters in length.
        #[builder(into, default)]
        pub authorizer_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether a Lambda authorizer returns a response in a simple format. If enabled, the Lambda authorizer can return a boolean value instead of an IAM policy.
        /// Supported only for HTTP APIs.
        #[builder(into, default)]
        pub enable_simple_responses: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identity sources for which authorization is requested.
        /// For `REQUEST` authorizers the value is a list of one or more mapping expressions of the specified request parameters.
        /// For `JWT` authorizers the single entry specifies where to extract the JSON Web Token (JWT) from inbound requests.
        #[builder(into, default)]
        pub identity_sources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration of a JWT authorizer. Required for the `JWT` authorizer type.
        /// Supported only for HTTP APIs.
        #[builder(into, default)]
        pub jwt_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::AuthorizerJwtConfiguration>,
        >,
        /// Name of the authorizer. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthorizerResult {
        /// API identifier.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Required credentials as an IAM role for API Gateway to invoke the authorizer.
        /// Supported only for `REQUEST` authorizers.
        pub authorizer_credentials_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Format of the payload sent to an HTTP API Lambda authorizer. Required for HTTP API Lambda authorizers.
        /// Valid values: `1.0`, `2.0`.
        pub authorizer_payload_format_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Time to live (TTL) for cached authorizer results, in seconds. If it equals 0, authorization caching is disabled.
        /// If it is greater than 0, API Gateway caches authorizer responses. The maximum value is 3600, or 1 hour. Defaults to `300`.
        /// Supported only for HTTP API Lambda authorizers.
        pub authorizer_result_ttl_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// Authorizer type. Valid values: `JWT`, `REQUEST`.
        /// Specify `REQUEST` for a Lambda function using incoming request parameters.
        /// For HTTP APIs, specify `JWT` to use JSON Web Tokens.
        pub authorizer_type: pulumi_wasm_rust::Output<String>,
        /// Authorizer's Uniform Resource Identifier (URI).
        /// For `REQUEST` authorizers this must be a well-formed Lambda function URI, such as the `invoke_arn` attribute of the `aws.lambda.Function` resource.
        /// Supported only for `REQUEST` authorizers. Must be between 1 and 2048 characters in length.
        pub authorizer_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether a Lambda authorizer returns a response in a simple format. If enabled, the Lambda authorizer can return a boolean value instead of an IAM policy.
        /// Supported only for HTTP APIs.
        pub enable_simple_responses: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identity sources for which authorization is requested.
        /// For `REQUEST` authorizers the value is a list of one or more mapping expressions of the specified request parameters.
        /// For `JWT` authorizers the single entry specifies where to extract the JSON Web Token (JWT) from inbound requests.
        pub identity_sources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration of a JWT authorizer. Required for the `JWT` authorizer type.
        /// Supported only for HTTP APIs.
        pub jwt_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::AuthorizerJwtConfiguration>,
        >,
        /// Name of the authorizer. Must be between 1 and 128 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AuthorizerArgs) -> AuthorizerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let authorizer_credentials_arn_binding = args
            .authorizer_credentials_arn
            .get_inner();
        let authorizer_payload_format_version_binding = args
            .authorizer_payload_format_version
            .get_inner();
        let authorizer_result_ttl_in_seconds_binding = args
            .authorizer_result_ttl_in_seconds
            .get_inner();
        let authorizer_type_binding = args.authorizer_type.get_inner();
        let authorizer_uri_binding = args.authorizer_uri.get_inner();
        let enable_simple_responses_binding = args.enable_simple_responses.get_inner();
        let identity_sources_binding = args.identity_sources.get_inner();
        let jwt_configuration_binding = args.jwt_configuration.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/authorizer:Authorizer".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerCredentialsArn".into(),
                    value: &authorizer_credentials_arn_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerPayloadFormatVersion".into(),
                    value: &authorizer_payload_format_version_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerResultTtlInSeconds".into(),
                    value: &authorizer_result_ttl_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerType".into(),
                    value: &authorizer_type_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerUri".into(),
                    value: &authorizer_uri_binding,
                },
                register_interface::ObjectField {
                    name: "enableSimpleResponses".into(),
                    value: &enable_simple_responses_binding,
                },
                register_interface::ObjectField {
                    name: "identitySources".into(),
                    value: &identity_sources_binding,
                },
                register_interface::ObjectField {
                    name: "jwtConfiguration".into(),
                    value: &jwt_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "authorizerCredentialsArn".into(),
                },
                register_interface::ResultField {
                    name: "authorizerPayloadFormatVersion".into(),
                },
                register_interface::ResultField {
                    name: "authorizerResultTtlInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "authorizerType".into(),
                },
                register_interface::ResultField {
                    name: "authorizerUri".into(),
                },
                register_interface::ResultField {
                    name: "enableSimpleResponses".into(),
                },
                register_interface::ResultField {
                    name: "identitySources".into(),
                },
                register_interface::ResultField {
                    name: "jwtConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthorizerResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            authorizer_credentials_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerCredentialsArn").unwrap(),
            ),
            authorizer_payload_format_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerPayloadFormatVersion").unwrap(),
            ),
            authorizer_result_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerResultTtlInSeconds").unwrap(),
            ),
            authorizer_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerType").unwrap(),
            ),
            authorizer_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerUri").unwrap(),
            ),
            enable_simple_responses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableSimpleResponses").unwrap(),
            ),
            identity_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identitySources").unwrap(),
            ),
            jwt_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jwtConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}