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
///           function: std:filebase64sha256
///           arguments:
///             input: lambda-function.zip
///           return: result
/// variables:
///   invocationAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - lambda:InvokeFunction
///             resources:
///               - ${authorizer.arn}
///   lambdaAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorizer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizerArgs {
        /// Credentials required for the authorizer. To specify an IAM Role for API Gateway to assume, use the IAM Role ARN.
        #[builder(into, default)]
        pub authorizer_credentials: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// TTL of cached authorizer results in seconds. Defaults to `300`.
        #[builder(into, default)]
        pub authorizer_result_ttl_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Authorizer's Uniform Resource Identifier (URI). This must be a well-formed Lambda function URI in the form of `arn:aws:apigateway:{region}:lambda:path/{service_api}`,
        /// e.g., `arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:012345678912:function:my-function/invocations`
        #[builder(into, default)]
        pub authorizer_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Source of the identity in an incoming request. Defaults to `method.request.header.Authorization`. For `REQUEST` type, this may be a comma-separated list of values, including headers, query string parameters and stage variables - e.g., `"method.request.header.SomeHeaderName,method.request.querystring.SomeQueryStringName,stageVariables.SomeStageVariableName"`
        #[builder(into, default)]
        pub identity_source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Validation expression for the incoming identity. For `TOKEN` type, this value should be a regular expression. The incoming token from the client is matched against this expression, and will proceed if the token matches. If the token doesn't match, the client receives a 401 Unauthorized response.
        #[builder(into, default)]
        pub identity_validation_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Name of the authorizer
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of the Amazon Cognito user pool ARNs. Each element is of this format: `arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}`.
        #[builder(into, default)]
        pub provider_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of the authorizer. Possible values are `TOKEN` for a Lambda function using a single authorization token submitted in a custom header, `REQUEST` for a Lambda function using incoming request parameters, or `COGNITO_USER_POOLS` for using an Amazon Cognito user pool. Defaults to `TOKEN`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthorizerResult {
        /// ARN of the API Gateway Authorizer
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Credentials required for the authorizer. To specify an IAM Role for API Gateway to assume, use the IAM Role ARN.
        pub authorizer_credentials: pulumi_gestalt_rust::Output<Option<String>>,
        /// TTL of cached authorizer results in seconds. Defaults to `300`.
        pub authorizer_result_ttl_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Authorizer's Uniform Resource Identifier (URI). This must be a well-formed Lambda function URI in the form of `arn:aws:apigateway:{region}:lambda:path/{service_api}`,
        /// e.g., `arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:012345678912:function:my-function/invocations`
        pub authorizer_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Source of the identity in an incoming request. Defaults to `method.request.header.Authorization`. For `REQUEST` type, this may be a comma-separated list of values, including headers, query string parameters and stage variables - e.g., `"method.request.header.SomeHeaderName,method.request.querystring.SomeQueryStringName,stageVariables.SomeStageVariableName"`
        pub identity_source: pulumi_gestalt_rust::Output<Option<String>>,
        /// Validation expression for the incoming identity. For `TOKEN` type, this value should be a regular expression. The incoming token from the client is matched against this expression, and will proceed if the token matches. If the token doesn't match, the client receives a 401 Unauthorized response.
        pub identity_validation_expression: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the authorizer
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of the Amazon Cognito user pool ARNs. Each element is of this format: `arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}`.
        pub provider_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// ID of the associated REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Type of the authorizer. Possible values are `TOKEN` for a Lambda function using a single authorization token submitted in a custom header, `REQUEST` for a Lambda function using incoming request parameters, or `COGNITO_USER_POOLS` for using an Amazon Cognito user pool. Defaults to `TOKEN`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthorizerArgs,
    ) -> AuthorizerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorizer_credentials_binding = args
            .authorizer_credentials
            .get_output(context);
        let authorizer_result_ttl_in_seconds_binding = args
            .authorizer_result_ttl_in_seconds
            .get_output(context);
        let authorizer_uri_binding = args.authorizer_uri.get_output(context);
        let identity_source_binding = args.identity_source.get_output(context);
        let identity_validation_expression_binding = args
            .identity_validation_expression
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let provider_arns_binding = args.provider_arns.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/authorizer:Authorizer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerCredentials".into(),
                    value: &authorizer_credentials_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerResultTtlInSeconds".into(),
                    value: &authorizer_result_ttl_in_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerUri".into(),
                    value: &authorizer_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identitySource".into(),
                    value: &identity_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityValidationExpression".into(),
                    value: &identity_validation_expression_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerArns".into(),
                    value: &provider_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthorizerResult {
            arn: o.get_field("arn"),
            authorizer_credentials: o.get_field("authorizerCredentials"),
            authorizer_result_ttl_in_seconds: o
                .get_field("authorizerResultTtlInSeconds"),
            authorizer_uri: o.get_field("authorizerUri"),
            identity_source: o.get_field("identitySource"),
            identity_validation_expression: o.get_field("identityValidationExpression"),
            name: o.get_field("name"),
            provider_arns: o.get_field("providerArns"),
            rest_api: o.get_field("restApi"),
            type_: o.get_field("type"),
        }
    }
}
