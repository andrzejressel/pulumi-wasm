/// Manages an Amazon API Gateway Version 2 integration.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = integration::create(
///         "example",
///         IntegrationArgs::builder()
///             .api_id("${exampleAwsApigatewayv2Api.id}")
///             .integration_type("MOCK")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Lambda Integration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: example.zip
///       name: Example
///       role: ${exampleAwsIamRole.arn}
///       handler: index.handler
///       runtime: nodejs20.x
///   exampleIntegration:
///     type: aws:apigatewayv2:Integration
///     name: example
///     properties:
///       apiId: ${exampleAwsApigatewayv2Api.id}
///       integrationType: AWS_PROXY
///       connectionType: INTERNET
///       contentHandlingStrategy: CONVERT_TO_TEXT
///       description: Lambda example
///       integrationMethod: POST
///       integrationUri: ${example.invokeArn}
///       passthroughBehavior: WHEN_NO_MATCH
/// ```
///
/// ### AWS Service Integration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apigatewayv2:Integration
///     properties:
///       apiId: ${exampleAwsApigatewayv2Api.id}
///       credentialsArn: ${exampleAwsIamRole.arn}
///       description: SQS example
///       integrationType: AWS_PROXY
///       integrationSubtype: SQS-SendMessage
///       requestParameters:
///         QueueUrl: $request.header.queueUrl
///         MessageBody: $request.body.message
/// ```
///
/// ### Private Integration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apigatewayv2:Integration
///     properties:
///       apiId: ${exampleAwsApigatewayv2Api.id}
///       credentialsArn: ${exampleAwsIamRole.arn}
///       description: Example with a load balancer
///       integrationType: HTTP_PROXY
///       integrationUri: ${exampleAwsLbListener.arn}
///       integrationMethod: ANY
///       connectionType: VPC_LINK
///       connectionId: ${exampleAwsApigatewayv2VpcLink.id}
///       tlsConfig:
///         serverNameToVerify: example.com
///       requestParameters:
///         append:header.authforintegration: $context.authorizer.authorizerResponse
///         overwrite:path: staticValueForIntegration
///       responseParameters:
///         - statusCode: 403
///           mappings:
///             append:header.auth: $context.authorizer.authorizerResponse
///         - statusCode: 200
///           mappings:
///             overwrite:statuscode: '204'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_integration` using the API identifier and integration identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/integration:Integration example aabbccddee/1122334
/// ```
/// -> __Note:__ The API Gateway managed integration created as part of [_quick_create_](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-basic-concept.html#apigateway-definition-quick-create) cannot be imported.
///
pub mod integration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the VPC link for a private integration. Supported only for HTTP APIs. Must be between 1 and 1024 characters in length.
        #[builder(into, default)]
        pub connection_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Type of the network connection to the integration endpoint. Valid values: `INTERNET`, `VPC_LINK`. Default is `INTERNET`.
        #[builder(into, default)]
        pub connection_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub content_handling_strategy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Credentials required for the integration, if any.
        #[builder(into, default)]
        pub credentials_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Description of the integration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Integration's HTTP method. Must be specified if `integration_type` is not `MOCK`.
        #[builder(into, default)]
        pub integration_method: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// AWS service action to invoke. Supported only for HTTP APIs when `integration_type` is `AWS_PROXY`. See the [AWS service integration reference](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services-reference.html) documentation for supported values. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub integration_subtype: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Integration type of an integration.
        /// Valid values: `AWS` (supported only for WebSocket APIs), `AWS_PROXY`, `HTTP` (supported only for WebSocket APIs), `HTTP_PROXY`, `MOCK` (supported only for WebSocket APIs). For an HTTP API private integration, use `HTTP_PROXY`.
        #[builder(into)]
        pub integration_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// URI of the Lambda function for a Lambda proxy integration, when `integration_type` is `AWS_PROXY`.
        /// For an `HTTP` integration, specify a fully-qualified URL. For an HTTP API private integration, specify the ARN of an Application Load Balancer listener, Network Load Balancer listener, or AWS Cloud Map service.
        #[builder(into, default)]
        pub integration_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the `request_templates` attribute.
        /// Valid values: `WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`. Default is `WHEN_NO_MATCH`. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub passthrough_behavior: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The [format of the payload](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format) sent to an integration. Valid values: `1.0`, `2.0`. Default is `1.0`.
        #[builder(into, default)]
        pub payload_format_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// For WebSocket APIs, a key-value map specifying request parameters that are passed from the method request to the backend.
        /// For HTTP APIs with a specified `integration_subtype`, a key-value map specifying parameters that are passed to `AWS_PROXY` integrations.
        /// For HTTP APIs without a specified `integration_subtype`, a key-value map specifying how to transform HTTP requests before sending them to the backend.
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html) for details.
        #[builder(into, default)]
        pub request_parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of [Velocity](https://velocity.apache.org/) templates that are applied on the request payload based on the value of the Content-Type header sent by the client. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub request_templates: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Mappings to transform the HTTP response from a backend integration before returning the response to clients. Supported only for HTTP APIs.
        #[builder(into, default)]
        pub response_parameters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::apigatewayv2::IntegrationResponseParameter>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration.
        #[builder(into, default)]
        pub template_selection_expression: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Custom timeout between 50 and 29,000 milliseconds for WebSocket APIs and between 50 and 30,000 milliseconds for HTTP APIs.
        /// The default timeout is 29 seconds for WebSocket APIs and 30 seconds for HTTP APIs.
        /// this provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub timeout_milliseconds: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// TLS configuration for a private integration. Supported only for HTTP APIs.
        #[builder(into, default)]
        pub tls_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apigatewayv2::IntegrationTlsConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationResult {
        /// API identifier.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// ID of the VPC link for a private integration. Supported only for HTTP APIs. Must be between 1 and 1024 characters in length.
        pub connection_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of the network connection to the integration endpoint. Valid values: `INTERNET`, `VPC_LINK`. Default is `INTERNET`.
        pub connection_type: pulumi_wasm_rust::Output<Option<String>>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`. Supported only for WebSocket APIs.
        pub content_handling_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// Credentials required for the integration, if any.
        pub credentials_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the integration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Integration's HTTP method. Must be specified if `integration_type` is not `MOCK`.
        pub integration_method: pulumi_wasm_rust::Output<Option<String>>,
        /// The [integration response selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions) for the integration.
        pub integration_response_selection_expression: pulumi_wasm_rust::Output<String>,
        /// AWS service action to invoke. Supported only for HTTP APIs when `integration_type` is `AWS_PROXY`. See the [AWS service integration reference](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services-reference.html) documentation for supported values. Must be between 1 and 128 characters in length.
        pub integration_subtype: pulumi_wasm_rust::Output<Option<String>>,
        /// Integration type of an integration.
        /// Valid values: `AWS` (supported only for WebSocket APIs), `AWS_PROXY`, `HTTP` (supported only for WebSocket APIs), `HTTP_PROXY`, `MOCK` (supported only for WebSocket APIs). For an HTTP API private integration, use `HTTP_PROXY`.
        pub integration_type: pulumi_wasm_rust::Output<String>,
        /// URI of the Lambda function for a Lambda proxy integration, when `integration_type` is `AWS_PROXY`.
        /// For an `HTTP` integration, specify a fully-qualified URL. For an HTTP API private integration, specify the ARN of an Application Load Balancer listener, Network Load Balancer listener, or AWS Cloud Map service.
        pub integration_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the `request_templates` attribute.
        /// Valid values: `WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`. Default is `WHEN_NO_MATCH`. Supported only for WebSocket APIs.
        pub passthrough_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// The [format of the payload](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format) sent to an integration. Valid values: `1.0`, `2.0`. Default is `1.0`.
        pub payload_format_version: pulumi_wasm_rust::Output<Option<String>>,
        /// For WebSocket APIs, a key-value map specifying request parameters that are passed from the method request to the backend.
        /// For HTTP APIs with a specified `integration_subtype`, a key-value map specifying parameters that are passed to `AWS_PROXY` integrations.
        /// For HTTP APIs without a specified `integration_subtype`, a key-value map specifying how to transform HTTP requests before sending them to the backend.
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html) for details.
        pub request_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of [Velocity](https://velocity.apache.org/) templates that are applied on the request payload based on the value of the Content-Type header sent by the client. Supported only for WebSocket APIs.
        pub request_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Mappings to transform the HTTP response from a backend integration before returning the response to clients. Supported only for HTTP APIs.
        pub response_parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigatewayv2::IntegrationResponseParameter>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration.
        pub template_selection_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom timeout between 50 and 29,000 milliseconds for WebSocket APIs and between 50 and 30,000 milliseconds for HTTP APIs.
        /// The default timeout is 29 seconds for WebSocket APIs and 30 seconds for HTTP APIs.
        /// this provider will only perform drift detection of its value when present in a configuration.
        pub timeout_milliseconds: pulumi_wasm_rust::Output<i32>,
        /// TLS configuration for a private integration. Supported only for HTTP APIs.
        pub tls_config: pulumi_wasm_rust::Output<
            Option<super::super::types::apigatewayv2::IntegrationTlsConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IntegrationArgs,
    ) -> IntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_output(context).get_inner();
        let connection_id_binding = args.connection_id.get_output(context).get_inner();
        let connection_type_binding = args
            .connection_type
            .get_output(context)
            .get_inner();
        let content_handling_strategy_binding = args
            .content_handling_strategy
            .get_output(context)
            .get_inner();
        let credentials_arn_binding = args
            .credentials_arn
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let integration_method_binding = args
            .integration_method
            .get_output(context)
            .get_inner();
        let integration_subtype_binding = args
            .integration_subtype
            .get_output(context)
            .get_inner();
        let integration_type_binding = args
            .integration_type
            .get_output(context)
            .get_inner();
        let integration_uri_binding = args
            .integration_uri
            .get_output(context)
            .get_inner();
        let passthrough_behavior_binding = args
            .passthrough_behavior
            .get_output(context)
            .get_inner();
        let payload_format_version_binding = args
            .payload_format_version
            .get_output(context)
            .get_inner();
        let request_parameters_binding = args
            .request_parameters
            .get_output(context)
            .get_inner();
        let request_templates_binding = args
            .request_templates
            .get_output(context)
            .get_inner();
        let response_parameters_binding = args
            .response_parameters
            .get_output(context)
            .get_inner();
        let template_selection_expression_binding = args
            .template_selection_expression
            .get_output(context)
            .get_inner();
        let timeout_milliseconds_binding = args
            .timeout_milliseconds
            .get_output(context)
            .get_inner();
        let tls_config_binding = args.tls_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/integration:Integration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "connectionType".into(),
                    value: &connection_type_binding,
                },
                register_interface::ObjectField {
                    name: "contentHandlingStrategy".into(),
                    value: &content_handling_strategy_binding,
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
                    name: "integrationMethod".into(),
                    value: &integration_method_binding,
                },
                register_interface::ObjectField {
                    name: "integrationSubtype".into(),
                    value: &integration_subtype_binding,
                },
                register_interface::ObjectField {
                    name: "integrationType".into(),
                    value: &integration_type_binding,
                },
                register_interface::ObjectField {
                    name: "integrationUri".into(),
                    value: &integration_uri_binding,
                },
                register_interface::ObjectField {
                    name: "passthroughBehavior".into(),
                    value: &passthrough_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "payloadFormatVersion".into(),
                    value: &payload_format_version_binding,
                },
                register_interface::ObjectField {
                    name: "requestParameters".into(),
                    value: &request_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "requestTemplates".into(),
                    value: &request_templates_binding,
                },
                register_interface::ObjectField {
                    name: "responseParameters".into(),
                    value: &response_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "templateSelectionExpression".into(),
                    value: &template_selection_expression_binding,
                },
                register_interface::ObjectField {
                    name: "timeoutMilliseconds".into(),
                    value: &timeout_milliseconds_binding,
                },
                register_interface::ObjectField {
                    name: "tlsConfig".into(),
                    value: &tls_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "connectionType".into(),
                },
                register_interface::ResultField {
                    name: "contentHandlingStrategy".into(),
                },
                register_interface::ResultField {
                    name: "credentialsArn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "integrationMethod".into(),
                },
                register_interface::ResultField {
                    name: "integrationResponseSelectionExpression".into(),
                },
                register_interface::ResultField {
                    name: "integrationSubtype".into(),
                },
                register_interface::ResultField {
                    name: "integrationType".into(),
                },
                register_interface::ResultField {
                    name: "integrationUri".into(),
                },
                register_interface::ResultField {
                    name: "passthroughBehavior".into(),
                },
                register_interface::ResultField {
                    name: "payloadFormatVersion".into(),
                },
                register_interface::ResultField {
                    name: "requestParameters".into(),
                },
                register_interface::ResultField {
                    name: "requestTemplates".into(),
                },
                register_interface::ResultField {
                    name: "responseParameters".into(),
                },
                register_interface::ResultField {
                    name: "templateSelectionExpression".into(),
                },
                register_interface::ResultField {
                    name: "timeoutMilliseconds".into(),
                },
                register_interface::ResultField {
                    name: "tlsConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            connection_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionType").unwrap(),
            ),
            content_handling_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentHandlingStrategy").unwrap(),
            ),
            credentials_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialsArn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            integration_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationMethod").unwrap(),
            ),
            integration_response_selection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationResponseSelectionExpression").unwrap(),
            ),
            integration_subtype: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationSubtype").unwrap(),
            ),
            integration_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationType").unwrap(),
            ),
            integration_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationUri").unwrap(),
            ),
            passthrough_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passthroughBehavior").unwrap(),
            ),
            payload_format_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("payloadFormatVersion").unwrap(),
            ),
            request_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestParameters").unwrap(),
            ),
            request_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestTemplates").unwrap(),
            ),
            response_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseParameters").unwrap(),
            ),
            template_selection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateSelectionExpression").unwrap(),
            ),
            timeout_milliseconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeoutMilliseconds").unwrap(),
            ),
            tls_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsConfig").unwrap(),
            ),
        }
    }
}
