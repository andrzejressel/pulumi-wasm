/// Provides an API Gateway Authorizer.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   demo:
///     type: aws:apigateway:Authorizer
///     properties:
///       name: demo
///       restApi: ${demoRestApi.id}
///       authorizerUri: ${authorizer.invokeArn}
///       authorizerCredentials: ${invocationRole.arn}
///   demoRestApi:
///     type: aws:apigateway:RestApi
///     name: demo
///     properties:
///       name: auth-demo
///   invocationRole:
///     type: aws:iam:Role
///     name: invocation_role
///     properties:
///       name: api_gateway_auth_invocation
///       path: /
///       assumeRolePolicy: ${invocationAssumeRole.json}
///   invocationPolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: invocation_policy
///     properties:
///       name: default
///       role: ${invocationRole.id}
///       policy: ${invocationPolicy.json}
///   lambda:
///     type: aws:iam:Role
///     properties:
///       name: demo-lambda
///       assumeRolePolicy: ${lambdaAssumeRole.json}
///   authorizer:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: lambda-function.zip
///       name: api_gateway_authorizer
///       role: ${lambda.arn}
///       handler: exports.example
///       sourceCodeHash:
///         fn::invoke:
///           Function: std:filebase64sha256
///           Arguments:
///             input: lambda-function.zip
///           Return: result
/// variables:
///   invocationAssumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - apigateway.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   invocationPolicy:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - lambda:InvokeFunction
///             resources:
///               - ${authorizer.arn}
///   lambdaAssumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS API Gateway Authorizer using the `REST-API-ID/AUTHORIZER-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/authorizer:Authorizer authorizer 12345abcde/example
/// ```
pub mod authorizer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizerArgs {
        /// Credentials required for the authorizer. To specify an IAM Role for API Gateway to assume, use the IAM Role ARN.
        #[builder(into, default)]
        pub authorizer_credentials: pulumi_wasm_rust::Output<Option<String>>,
        /// TTL of cached authorizer results in seconds. Defaults to `300`.
        #[builder(into, default)]
        pub authorizer_result_ttl_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Authorizer's Uniform Resource Identifier (URI). This must be a well-formed Lambda function URI in the form of `arn:aws:apigateway:{region}:lambda:path/{service_api}`,
        /// e.g., `arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:012345678912:function:my-function/invocations`
        #[builder(into, default)]
        pub authorizer_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Source of the identity in an incoming request. Defaults to `method.request.header.Authorization`. For `REQUEST` type, this may be a comma-separated list of values, including headers, query string parameters and stage variables - e.g., `"method.request.header.SomeHeaderName,method.request.querystring.SomeQueryStringName,stageVariables.SomeStageVariableName"`
        #[builder(into, default)]
        pub identity_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Validation expression for the incoming identity. For `TOKEN` type, this value should be a regular expression. The incoming token from the client is matched against this expression, and will proceed if the token matches. If the token doesn't match, the client receives a 401 Unauthorized response.
        #[builder(into, default)]
        pub identity_validation_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the authorizer
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// List of the Amazon Cognito user pool ARNs. Each element is of this format: `arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}`.
        #[builder(into, default)]
        pub provider_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Type of the authorizer. Possible values are `TOKEN` for a Lambda function using a single authorization token submitted in a custom header, `REQUEST` for a Lambda function using incoming request parameters, or `COGNITO_USER_POOLS` for using an Amazon Cognito user pool. Defaults to `TOKEN`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthorizerResult {
        /// ARN of the API Gateway Authorizer
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Credentials required for the authorizer. To specify an IAM Role for API Gateway to assume, use the IAM Role ARN.
        pub authorizer_credentials: pulumi_wasm_rust::Output<Option<String>>,
        /// TTL of cached authorizer results in seconds. Defaults to `300`.
        pub authorizer_result_ttl_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Authorizer's Uniform Resource Identifier (URI). This must be a well-formed Lambda function URI in the form of `arn:aws:apigateway:{region}:lambda:path/{service_api}`,
        /// e.g., `arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:012345678912:function:my-function/invocations`
        pub authorizer_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Source of the identity in an incoming request. Defaults to `method.request.header.Authorization`. For `REQUEST` type, this may be a comma-separated list of values, including headers, query string parameters and stage variables - e.g., `"method.request.header.SomeHeaderName,method.request.querystring.SomeQueryStringName,stageVariables.SomeStageVariableName"`
        pub identity_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Validation expression for the incoming identity. For `TOKEN` type, this value should be a regular expression. The incoming token from the client is matched against this expression, and will proceed if the token matches. If the token doesn't match, the client receives a 401 Unauthorized response.
        pub identity_validation_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the authorizer
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of the Amazon Cognito user pool ARNs. Each element is of this format: `arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}`.
        pub provider_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ID of the associated REST API
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Type of the authorizer. Possible values are `TOKEN` for a Lambda function using a single authorization token submitted in a custom header, `REQUEST` for a Lambda function using incoming request parameters, or `COGNITO_USER_POOLS` for using an Amazon Cognito user pool. Defaults to `TOKEN`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AuthorizerArgs) -> AuthorizerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorizer_credentials_binding = args.authorizer_credentials.get_inner();
        let authorizer_result_ttl_in_seconds_binding = args
            .authorizer_result_ttl_in_seconds
            .get_inner();
        let authorizer_uri_binding = args.authorizer_uri.get_inner();
        let identity_source_binding = args.identity_source.get_inner();
        let identity_validation_expression_binding = args
            .identity_validation_expression
            .get_inner();
        let name_binding = args.name.get_inner();
        let provider_arns_binding = args.provider_arns.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/authorizer:Authorizer".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizerCredentials".into(),
                    value: &authorizer_credentials_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerResultTtlInSeconds".into(),
                    value: &authorizer_result_ttl_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerUri".into(),
                    value: &authorizer_uri_binding,
                },
                register_interface::ObjectField {
                    name: "identitySource".into(),
                    value: &identity_source_binding,
                },
                register_interface::ObjectField {
                    name: "identityValidationExpression".into(),
                    value: &identity_validation_expression_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "providerArns".into(),
                    value: &provider_arns_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authorizerCredentials".into(),
                },
                register_interface::ResultField {
                    name: "authorizerResultTtlInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "authorizerUri".into(),
                },
                register_interface::ResultField {
                    name: "identitySource".into(),
                },
                register_interface::ResultField {
                    name: "identityValidationExpression".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "providerArns".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
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
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authorizer_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerCredentials").unwrap(),
            ),
            authorizer_result_ttl_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerResultTtlInSeconds").unwrap(),
            ),
            authorizer_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerUri").unwrap(),
            ),
            identity_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identitySource").unwrap(),
            ),
            identity_validation_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityValidationExpression").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            provider_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerArns").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
