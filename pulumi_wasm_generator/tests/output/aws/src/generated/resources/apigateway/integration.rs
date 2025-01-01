/// Provides an HTTP Method Integration for an API Gateway Integration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myDemoAPI:
///     type: aws:apigateway:RestApi
///     name: MyDemoAPI
///     properties:
///       name: MyDemoAPI
///       description: This is my API for demonstration purposes
///   myDemoResource:
///     type: aws:apigateway:Resource
///     name: MyDemoResource
///     properties:
///       restApi: ${myDemoAPI.id}
///       parentId: ${myDemoAPI.rootResourceId}
///       pathPart: mydemoresource
///   myDemoMethod:
///     type: aws:apigateway:Method
///     name: MyDemoMethod
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: GET
///       authorization: NONE
///   myDemoIntegration:
///     type: aws:apigateway:Integration
///     name: MyDemoIntegration
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: ${myDemoMethod.httpMethod}
///       type: MOCK
///       cacheKeyParameters:
///         - method.request.path.param
///       cacheNamespace: foobar
///       timeoutMilliseconds: 29000
///       requestParameters:
///         integration.request.header.X-Authorization: '''static'''
///       requestTemplates:
///         application/xml: |
///           {
///              "body" : $input.json('$')
///           }
/// ```
///
/// ## Lambda integration
///
/// ```yaml
/// configuration:
///   # Variables
///   myregion:
///     type: dynamic
///   accountId:
///     type: dynamic
/// resources:
///   # API Gateway
///   api:
///     type: aws:apigateway:RestApi
///     properties:
///       name: myapi
///   resource:
///     type: aws:apigateway:Resource
///     properties:
///       pathPart: resource
///       parentId: ${api.rootResourceId}
///       restApi: ${api.id}
///   method:
///     type: aws:apigateway:Method
///     properties:
///       restApi: ${api.id}
///       resourceId: ${resource.id}
///       httpMethod: GET
///       authorization: NONE
///   integration:
///     type: aws:apigateway:Integration
///     properties:
///       restApi: ${api.id}
///       resourceId: ${resource.id}
///       httpMethod: ${method.httpMethod}
///       integrationHttpMethod: POST
///       type: AWS_PROXY
///       uri: ${lambda.invokeArn}
///   # Lambda
///   apigwLambda:
///     type: aws:lambda:Permission
///     name: apigw_lambda
///     properties:
///       statementId: AllowExecutionFromAPIGateway
///       action: lambda:InvokeFunction
///       function: ${lambda.name}
///       principal: apigateway.amazonaws.com
///       sourceArn: arn:aws:execute-api:${myregion}:${accountId}:${api.id}/*/${method.httpMethod}${resource.path}
///   lambda:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: lambda.zip
///       name: mylambda
///       role: ${role.arn}
///       handler: lambda.lambda_handler
///       runtime: python3.12
///       sourceCodeHash:
///         fn::invoke:
///           function: std:filebase64sha256
///           arguments:
///             input: lambda.zip
///           return: result
///   role:
///     type: aws:iam:Role
///     properties:
///       name: myrole
///       assumeRolePolicy: ${assumeRole.json}
/// variables:
///   # IAM
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## VPC Link
///
/// ```yaml
/// configuration:
///   name:
///     type: dynamic
///   subnetId:
///     type: dynamic
/// resources:
///   test:
///     type: aws:lb:LoadBalancer
///     properties:
///       name: ${name}
///       internal: true
///       loadBalancerType: network
///       subnets:
///         - ${subnetId}
///   testVpcLink:
///     type: aws:apigateway:VpcLink
///     name: test
///     properties:
///       name: ${name}
///       targetArn: ${test.arn}
///   testRestApi:
///     type: aws:apigateway:RestApi
///     name: test
///     properties:
///       name: ${name}
///   testResource:
///     type: aws:apigateway:Resource
///     name: test
///     properties:
///       restApi: ${testRestApi.id}
///       parentId: ${testRestApi.rootResourceId}
///       pathPart: test
///   testMethod:
///     type: aws:apigateway:Method
///     name: test
///     properties:
///       restApi: ${testRestApi.id}
///       resourceId: ${testResource.id}
///       httpMethod: GET
///       authorization: NONE
///       requestModels:
///         application/json: Error
///   testIntegration:
///     type: aws:apigateway:Integration
///     name: test
///     properties:
///       restApi: ${testRestApi.id}
///       resourceId: ${testResource.id}
///       httpMethod: ${testMethod.httpMethod}
///       requestTemplates:
///         application/json: ""
///         application/xml: |-
///           #set($inputRoot = $input.path('$'))
///           { }
///       requestParameters:
///         integration.request.header.X-Authorization: '''static'''
///         integration.request.header.X-Foo: '''Bar'''
///       type: HTTP
///       uri: https://www.google.de
///       integrationHttpMethod: GET
///       passthroughBehavior: WHEN_NO_MATCH
///       contentHandling: CONVERT_TO_TEXT
///       connectionType: VPC_LINK
///       connectionId: ${testVpcLink.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_integration` using `REST-API-ID/RESOURCE-ID/HTTP-METHOD`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/integration:Integration example 12345abcde/67890fghij/GET
/// ```
pub mod integration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationArgs {
        /// List of cache key parameters for the integration.
        #[builder(into, default)]
        pub cache_key_parameters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Integration's cache namespace.
        #[builder(into, default)]
        pub cache_namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the VpcLink used for the integration. **Required** if `connection_type` is `VPC_LINK`
        #[builder(into, default)]
        pub connection_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Integration input's [connectionType](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/#connectionType). Valid values are `INTERNET` (default for connections through the public routable internet), and `VPC_LINK` (for private connections between API Gateway and a network load balancer in a VPC).
        #[builder(into, default)]
        pub connection_type: pulumi_wasm_rust::Output<Option<String>>,
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the passthroughBehaviors is configured to support payload pass-through.
        #[builder(into, default)]
        pub content_handling: pulumi_wasm_rust::Output<Option<String>>,
        /// Credentials required for the integration. For `AWS` integrations, 2 options are available. To specify an IAM Role for Amazon API Gateway to assume, use the role's ARN. To require that the caller's identity be passed through from the request, specify the string `arn:aws:iam::\*:user/\*`.
        #[builder(into, default)]
        pub credentials: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTION`, `ANY`)
        /// when calling the associated resource.
        #[builder(into)]
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// Integration HTTP method
        /// (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONs`, `ANY`, `PATCH`) specifying how API Gateway will interact with the back end.
        /// **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// Not all methods are compatible with all `AWS` integrations.
        /// e.g., Lambda function [can only be invoked](https://github.com/awslabs/aws-apigateway-importer/issues/9#issuecomment-129651005) via `POST`.
        #[builder(into, default)]
        pub integration_http_method: pulumi_wasm_rust::Output<Option<String>>,
        /// Integration passthrough behavior (`WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`).  **Required** if `request_templates` is used.
        #[builder(into, default)]
        pub passthrough_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of request query string parameters and headers that should be passed to the backend responder.
        /// For example: `request_parameters = { "integration.request.header.X-Some-Other-Header" = "method.request.header.X-Some-Header" }`
        #[builder(into, default)]
        pub request_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of the integration's request templates.
        #[builder(into, default)]
        pub request_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// API resource ID.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Custom timeout between 50 and 300,000 milliseconds. The default value is 29,000 milliseconds. You need to raise a [Service Quota Ticket](https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html) to increase time beyond 29,000 milliseconds.
        #[builder(into, default)]
        pub timeout_milliseconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// TLS configuration. See below.
        #[builder(into, default)]
        pub tls_config: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::IntegrationTlsConfig>,
        >,
        /// Integration input's [type](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/). Valid values are `HTTP` (for HTTP backends), `MOCK` (not calling any real backend), `AWS` (for AWS services), `AWS_PROXY` (for Lambda proxy integration) and `HTTP_PROXY` (for HTTP proxy integration). An `HTTP` or `HTTP_PROXY` integration with a `connection_type` of `VPC_LINK` is referred to as a private integration and uses a VpcLink to connect API Gateway to a network load balancer of a VPC.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Input's URI. **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// For HTTP integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the RFC-3986 specification . For AWS integrations, the URI should be of the form `arn:aws:apigateway:{region}:{subdomain.service|service}:{path|action}/{service_api}`. `region`, `subdomain` and `service` are used to determine the right endpoint.
        /// e.g., `arn:aws:apigateway:eu-west-1:lambda:path/2015-03-31/functions/arn:aws:lambda:eu-west-1:123456789012:function:my-func/invocations`. For private integrations, the URI parameter is not used for routing requests to your endpoint, but is used for setting the Host header and for certificate validation.
        #[builder(into, default)]
        pub uri: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IntegrationResult {
        /// List of cache key parameters for the integration.
        pub cache_key_parameters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Integration's cache namespace.
        pub cache_namespace: pulumi_wasm_rust::Output<String>,
        /// ID of the VpcLink used for the integration. **Required** if `connection_type` is `VPC_LINK`
        pub connection_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Integration input's [connectionType](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/#connectionType). Valid values are `INTERNET` (default for connections through the public routable internet), and `VPC_LINK` (for private connections between API Gateway and a network load balancer in a VPC).
        pub connection_type: pulumi_wasm_rust::Output<Option<String>>,
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the passthroughBehaviors is configured to support payload pass-through.
        pub content_handling: pulumi_wasm_rust::Output<Option<String>>,
        /// Credentials required for the integration. For `AWS` integrations, 2 options are available. To specify an IAM Role for Amazon API Gateway to assume, use the role's ARN. To require that the caller's identity be passed through from the request, specify the string `arn:aws:iam::\*:user/\*`.
        pub credentials: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTION`, `ANY`)
        /// when calling the associated resource.
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// Integration HTTP method
        /// (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONs`, `ANY`, `PATCH`) specifying how API Gateway will interact with the back end.
        /// **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// Not all methods are compatible with all `AWS` integrations.
        /// e.g., Lambda function [can only be invoked](https://github.com/awslabs/aws-apigateway-importer/issues/9#issuecomment-129651005) via `POST`.
        pub integration_http_method: pulumi_wasm_rust::Output<Option<String>>,
        /// Integration passthrough behavior (`WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`).  **Required** if `request_templates` is used.
        pub passthrough_behavior: pulumi_wasm_rust::Output<String>,
        /// Map of request query string parameters and headers that should be passed to the backend responder.
        /// For example: `request_parameters = { "integration.request.header.X-Some-Other-Header" = "method.request.header.X-Some-Header" }`
        pub request_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of the integration's request templates.
        pub request_templates: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// API resource ID.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API.
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Custom timeout between 50 and 300,000 milliseconds. The default value is 29,000 milliseconds. You need to raise a [Service Quota Ticket](https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html) to increase time beyond 29,000 milliseconds.
        pub timeout_milliseconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// TLS configuration. See below.
        pub tls_config: pulumi_wasm_rust::Output<
            Option<super::super::types::apigateway::IntegrationTlsConfig>,
        >,
        /// Integration input's [type](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/). Valid values are `HTTP` (for HTTP backends), `MOCK` (not calling any real backend), `AWS` (for AWS services), `AWS_PROXY` (for Lambda proxy integration) and `HTTP_PROXY` (for HTTP proxy integration). An `HTTP` or `HTTP_PROXY` integration with a `connection_type` of `VPC_LINK` is referred to as a private integration and uses a VpcLink to connect API Gateway to a network load balancer of a VPC.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Input's URI. **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// For HTTP integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the RFC-3986 specification . For AWS integrations, the URI should be of the form `arn:aws:apigateway:{region}:{subdomain.service|service}:{path|action}/{service_api}`. `region`, `subdomain` and `service` are used to determine the right endpoint.
        /// e.g., `arn:aws:apigateway:eu-west-1:lambda:path/2015-03-31/functions/arn:aws:lambda:eu-west-1:123456789012:function:my-func/invocations`. For private integrations, the URI parameter is not used for routing requests to your endpoint, but is used for setting the Host header and for certificate validation.
        pub uri: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IntegrationArgs) -> IntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cache_key_parameters_binding = args.cache_key_parameters.get_inner();
        let cache_namespace_binding = args.cache_namespace.get_inner();
        let connection_id_binding = args.connection_id.get_inner();
        let connection_type_binding = args.connection_type.get_inner();
        let content_handling_binding = args.content_handling.get_inner();
        let credentials_binding = args.credentials.get_inner();
        let http_method_binding = args.http_method.get_inner();
        let integration_http_method_binding = args.integration_http_method.get_inner();
        let passthrough_behavior_binding = args.passthrough_behavior.get_inner();
        let request_parameters_binding = args.request_parameters.get_inner();
        let request_templates_binding = args.request_templates.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let timeout_milliseconds_binding = args.timeout_milliseconds.get_inner();
        let tls_config_binding = args.tls_config.get_inner();
        let type__binding = args.type_.get_inner();
        let uri_binding = args.uri.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/integration:Integration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cacheKeyParameters".into(),
                    value: &cache_key_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "cacheNamespace".into(),
                    value: &cache_namespace_binding,
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
                    name: "contentHandling".into(),
                    value: &content_handling_binding,
                },
                register_interface::ObjectField {
                    name: "credentials".into(),
                    value: &credentials_binding,
                },
                register_interface::ObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding,
                },
                register_interface::ObjectField {
                    name: "integrationHttpMethod".into(),
                    value: &integration_http_method_binding,
                },
                register_interface::ObjectField {
                    name: "passthroughBehavior".into(),
                    value: &passthrough_behavior_binding,
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
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "timeoutMilliseconds".into(),
                    value: &timeout_milliseconds_binding,
                },
                register_interface::ObjectField {
                    name: "tlsConfig".into(),
                    value: &tls_config_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "uri".into(),
                    value: &uri_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cacheKeyParameters".into(),
                },
                register_interface::ResultField {
                    name: "cacheNamespace".into(),
                },
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "connectionType".into(),
                },
                register_interface::ResultField {
                    name: "contentHandling".into(),
                },
                register_interface::ResultField {
                    name: "credentials".into(),
                },
                register_interface::ResultField {
                    name: "httpMethod".into(),
                },
                register_interface::ResultField {
                    name: "integrationHttpMethod".into(),
                },
                register_interface::ResultField {
                    name: "passthroughBehavior".into(),
                },
                register_interface::ResultField {
                    name: "requestParameters".into(),
                },
                register_interface::ResultField {
                    name: "requestTemplates".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
                register_interface::ResultField {
                    name: "timeoutMilliseconds".into(),
                },
                register_interface::ResultField {
                    name: "tlsConfig".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uri".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationResult {
            cache_key_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheKeyParameters").unwrap(),
            ),
            cache_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheNamespace").unwrap(),
            ),
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            connection_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionType").unwrap(),
            ),
            content_handling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentHandling").unwrap(),
            ),
            credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentials").unwrap(),
            ),
            http_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpMethod").unwrap(),
            ),
            integration_http_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationHttpMethod").unwrap(),
            ),
            passthrough_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passthroughBehavior").unwrap(),
            ),
            request_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestParameters").unwrap(),
            ),
            request_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestTemplates").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            timeout_milliseconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeoutMilliseconds").unwrap(),
            ),
            tls_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsConfig").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uri").unwrap()),
        }
    }
}
