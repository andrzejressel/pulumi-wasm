/// Manages an Amazon API Gateway Version 2 integration.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api.html).
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the VPC link for a private integration. Supported only for HTTP APIs. Must be between 1 and 1024 characters in length.
        #[builder(into, default)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of the network connection to the integration endpoint. Valid values: `INTERNET`, `VPC_LINK`. Default is `INTERNET`.
        #[builder(into, default)]
        pub connection_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub content_handling_strategy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Credentials required for the integration, if any.
        #[builder(into, default)]
        pub credentials_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the integration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Integration's HTTP method. Must be specified if `integration_type` is not `MOCK`.
        #[builder(into, default)]
        pub integration_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS service action to invoke. Supported only for HTTP APIs when `integration_type` is `AWS_PROXY`. See the [AWS service integration reference](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services-reference.html) documentation for supported values. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub integration_subtype: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Integration type of an integration.
        /// Valid values: `AWS` (supported only for WebSocket APIs), `AWS_PROXY`, `HTTP` (supported only for WebSocket APIs), `HTTP_PROXY`, `MOCK` (supported only for WebSocket APIs). For an HTTP API private integration, use `HTTP_PROXY`.
        #[builder(into)]
        pub integration_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// URI of the Lambda function for a Lambda proxy integration, when `integration_type` is `AWS_PROXY`.
        /// For an `HTTP` integration, specify a fully-qualified URL. For an HTTP API private integration, specify the ARN of an Application Load Balancer listener, Network Load Balancer listener, or AWS Cloud Map service.
        #[builder(into, default)]
        pub integration_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the `request_templates` attribute.
        /// Valid values: `WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`. Default is `WHEN_NO_MATCH`. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub passthrough_behavior: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The [format of the payload](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format) sent to an integration. Valid values: `1.0`, `2.0`. Default is `1.0`.
        #[builder(into, default)]
        pub payload_format_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// For WebSocket APIs, a key-value map specifying request parameters that are passed from the method request to the backend.
        /// For HTTP APIs with a specified `integration_subtype`, a key-value map specifying parameters that are passed to `AWS_PROXY` integrations.
        /// For HTTP APIs without a specified `integration_subtype`, a key-value map specifying how to transform HTTP requests before sending them to the backend.
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html) for details.
        #[builder(into, default)]
        pub request_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of [Velocity](https://velocity.apache.org/) templates that are applied on the request payload based on the value of the Content-Type header sent by the client. Supported only for WebSocket APIs.
        #[builder(into, default)]
        pub request_templates: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Mappings to transform the HTTP response from a backend integration before returning the response to clients. Supported only for HTTP APIs.
        #[builder(into, default)]
        pub response_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigatewayv2::IntegrationResponseParameter>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration.
        #[builder(into, default)]
        pub template_selection_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Custom timeout between 50 and 29,000 milliseconds for WebSocket APIs and between 50 and 30,000 milliseconds for HTTP APIs.
        /// The default timeout is 29 seconds for WebSocket APIs and 30 seconds for HTTP APIs.
        /// this provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub timeout_milliseconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// TLS configuration for a private integration. Supported only for HTTP APIs.
        #[builder(into, default)]
        pub tls_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigatewayv2::IntegrationTlsConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationResult {
        /// API identifier.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the VPC link for a private integration. Supported only for HTTP APIs. Must be between 1 and 1024 characters in length.
        pub connection_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of the network connection to the integration endpoint. Valid values: `INTERNET`, `VPC_LINK`. Default is `INTERNET`.
        pub connection_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// How to handle response payload content type conversions. Valid values: `CONVERT_TO_BINARY`, `CONVERT_TO_TEXT`. Supported only for WebSocket APIs.
        pub content_handling_strategy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Credentials required for the integration, if any.
        pub credentials_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the integration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Integration's HTTP method. Must be specified if `integration_type` is not `MOCK`.
        pub integration_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [integration response selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions) for the integration.
        pub integration_response_selection_expression: pulumi_gestalt_rust::Output<
            String,
        >,
        /// AWS service action to invoke. Supported only for HTTP APIs when `integration_type` is `AWS_PROXY`. See the [AWS service integration reference](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services-reference.html) documentation for supported values. Must be between 1 and 128 characters in length.
        pub integration_subtype: pulumi_gestalt_rust::Output<Option<String>>,
        /// Integration type of an integration.
        /// Valid values: `AWS` (supported only for WebSocket APIs), `AWS_PROXY`, `HTTP` (supported only for WebSocket APIs), `HTTP_PROXY`, `MOCK` (supported only for WebSocket APIs). For an HTTP API private integration, use `HTTP_PROXY`.
        pub integration_type: pulumi_gestalt_rust::Output<String>,
        /// URI of the Lambda function for a Lambda proxy integration, when `integration_type` is `AWS_PROXY`.
        /// For an `HTTP` integration, specify a fully-qualified URL. For an HTTP API private integration, specify the ARN of an Application Load Balancer listener, Network Load Balancer listener, or AWS Cloud Map service.
        pub integration_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the `request_templates` attribute.
        /// Valid values: `WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`. Default is `WHEN_NO_MATCH`. Supported only for WebSocket APIs.
        pub passthrough_behavior: pulumi_gestalt_rust::Output<Option<String>>,
        /// The [format of the payload](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.proxy-format) sent to an integration. Valid values: `1.0`, `2.0`. Default is `1.0`.
        pub payload_format_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// For WebSocket APIs, a key-value map specifying request parameters that are passed from the method request to the backend.
        /// For HTTP APIs with a specified `integration_subtype`, a key-value map specifying parameters that are passed to `AWS_PROXY` integrations.
        /// For HTTP APIs without a specified `integration_subtype`, a key-value map specifying how to transform HTTP requests before sending them to the backend.
        /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html) for details.
        pub request_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of [Velocity](https://velocity.apache.org/) templates that are applied on the request payload based on the value of the Content-Type header sent by the client. Supported only for WebSocket APIs.
        pub request_templates: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Mappings to transform the HTTP response from a backend integration before returning the response to clients. Supported only for HTTP APIs.
        pub response_parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigatewayv2::IntegrationResponseParameter>>,
        >,
        /// The [template selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-template-selection-expressions) for the integration.
        pub template_selection_expression: pulumi_gestalt_rust::Output<Option<String>>,
        /// Custom timeout between 50 and 29,000 milliseconds for WebSocket APIs and between 50 and 30,000 milliseconds for HTTP APIs.
        /// The default timeout is 29 seconds for WebSocket APIs and 30 seconds for HTTP APIs.
        /// this provider will only perform drift detection of its value when present in a configuration.
        pub timeout_milliseconds: pulumi_gestalt_rust::Output<i32>,
        /// TLS configuration for a private integration. Supported only for HTTP APIs.
        pub tls_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigatewayv2::IntegrationTlsConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationArgs,
    ) -> IntegrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let connection_id_binding = args.connection_id.get_output(context);
        let connection_type_binding = args.connection_type.get_output(context);
        let content_handling_strategy_binding = args
            .content_handling_strategy
            .get_output(context);
        let credentials_arn_binding = args.credentials_arn.get_output(context);
        let description_binding = args.description.get_output(context);
        let integration_method_binding = args.integration_method.get_output(context);
        let integration_subtype_binding = args.integration_subtype.get_output(context);
        let integration_type_binding = args.integration_type.get_output(context);
        let integration_uri_binding = args.integration_uri.get_output(context);
        let passthrough_behavior_binding = args.passthrough_behavior.get_output(context);
        let payload_format_version_binding = args
            .payload_format_version
            .get_output(context);
        let request_parameters_binding = args.request_parameters.get_output(context);
        let request_templates_binding = args.request_templates.get_output(context);
        let response_parameters_binding = args.response_parameters.get_output(context);
        let template_selection_expression_binding = args
            .template_selection_expression
            .get_output(context);
        let timeout_milliseconds_binding = args.timeout_milliseconds.get_output(context);
        let tls_config_binding = args.tls_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/integration:Integration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: connection_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionType".into(),
                    value: connection_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentHandlingStrategy".into(),
                    value: content_handling_strategy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credentialsArn".into(),
                    value: credentials_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationMethod".into(),
                    value: integration_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationSubtype".into(),
                    value: integration_subtype_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationType".into(),
                    value: integration_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationUri".into(),
                    value: integration_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passthroughBehavior".into(),
                    value: passthrough_behavior_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "payloadFormatVersion".into(),
                    value: payload_format_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestParameters".into(),
                    value: request_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestTemplates".into(),
                    value: request_templates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responseParameters".into(),
                    value: response_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateSelectionExpression".into(),
                    value: template_selection_expression_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeoutMilliseconds".into(),
                    value: timeout_milliseconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsConfig".into(),
                    value: tls_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationResult {
            api_id: o.get_field("apiId"),
            connection_id: o.get_field("connectionId"),
            connection_type: o.get_field("connectionType"),
            content_handling_strategy: o.get_field("contentHandlingStrategy"),
            credentials_arn: o.get_field("credentialsArn"),
            description: o.get_field("description"),
            integration_method: o.get_field("integrationMethod"),
            integration_response_selection_expression: o
                .get_field("integrationResponseSelectionExpression"),
            integration_subtype: o.get_field("integrationSubtype"),
            integration_type: o.get_field("integrationType"),
            integration_uri: o.get_field("integrationUri"),
            passthrough_behavior: o.get_field("passthroughBehavior"),
            payload_format_version: o.get_field("payloadFormatVersion"),
            request_parameters: o.get_field("requestParameters"),
            request_templates: o.get_field("requestTemplates"),
            response_parameters: o.get_field("responseParameters"),
            template_selection_expression: o.get_field("templateSelectionExpression"),
            timeout_milliseconds: o.get_field("timeoutMilliseconds"),
            tls_config: o.get_field("tlsConfig"),
        }
    }
}
