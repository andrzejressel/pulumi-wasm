/// Gives an external source (like an EventBridge Rule, SNS, or S3) permission to access the Lambda function.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   allowCloudwatch:
///     type: aws:lambda:Permission
///     name: allow_cloudwatch
///     properties:
///       statementId: AllowExecutionFromCloudWatch
///       action: lambda:InvokeFunction
///       function: ${testLambda.name}
///       principal: events.amazonaws.com
///       sourceArn: arn:aws:events:eu-west-1:111122223333:rule/RunDaily
///       qualifier: ${testAlias.name}
///   testAlias:
///     type: aws:lambda:Alias
///     name: test_alias
///     properties:
///       name: testalias
///       description: a sample description
///       functionName: ${testLambda.name}
///       functionVersion: $LATEST
///   testLambda:
///     type: aws:lambda:Function
///     name: test_lambda
///     properties:
///       code:
///         fn::FileArchive: lambdatest.zip
///       name: lambda_function_name
///       role: ${iamForLambda.arn}
///       handler: exports.handler
///       runtime: nodejs20.x
///   iamForLambda:
///     type: aws:iam:Role
///     name: iam_for_lambda
///     properties:
///       name: iam_for_lambda
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid:
///               Principal:
///                 Service: lambda.amazonaws.com
/// ```
///
/// ### With SNS
///
/// ```yaml
/// resources:
///   withSns:
///     type: aws:lambda:Permission
///     name: with_sns
///     properties:
///       statementId: AllowExecutionFromSNS
///       action: lambda:InvokeFunction
///       function: ${func.name}
///       principal: sns.amazonaws.com
///       sourceArn: ${default.arn}
///   default:
///     type: aws:sns:Topic
///     properties:
///       name: call-lambda-maybe
///   lambda:
///     type: aws:sns:TopicSubscription
///     properties:
///       topic: ${default.arn}
///       protocol: lambda
///       endpoint: ${func.arn}
///   func:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: lambdatest.zip
///       name: lambda_called_from_sns
///       role: ${defaultRole.arn}
///       handler: exports.handler
///       runtime: python3.12
///   defaultRole:
///     type: aws:iam:Role
///     name: default
///     properties:
///       name: iam_for_lambda_with_sns
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid:
///               Principal:
///                 Service: lambda.amazonaws.com
/// ```
///
/// ### With API Gateway REST API
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lambdaPermission = permission::create(
///         "lambdaPermission",
///         PermissionArgs::builder()
///             .action("lambda:InvokeFunction")
///             .function("MyDemoFunction")
///             .principal("apigateway.amazonaws.com")
///             .source_arn("${myDemoAPI.executionArn}/*")
///             .statement_id("AllowMyDemoAPIInvoke")
///             .build_struct(),
///     );
///     let myDemoAPI = rest_api::create(
///         "myDemoAPI",
///         RestApiArgs::builder()
///             .description("This is my API for demonstration purposes")
///             .name("MyDemoAPI")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With CloudWatch Log Group
///
/// ```yaml
/// resources:
///   logging:
///     type: aws:lambda:Permission
///     properties:
///       action: lambda:InvokeFunction
///       function: ${loggingFunction.name}
///       principal: logs.eu-west-1.amazonaws.com
///       sourceArn: ${default.arn}:*
///   default:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: /default
///   loggingLogSubscriptionFilter:
///     type: aws:cloudwatch:LogSubscriptionFilter
///     name: logging
///     properties:
///       destinationArn: ${loggingFunction.arn}
///       filterPattern:
///       logGroup: ${default.name}
///       name: logging_default
///     options:
///       dependson:
///         - ${logging}
///   loggingFunction:
///     type: aws:lambda:Function
///     name: logging
///     properties:
///       code:
///         fn::FileArchive: lamba_logging.zip
///       name: lambda_called_from_cloudwatch_logs
///       handler: exports.handler
///       role: ${defaultRole.arn}
///       runtime: python3.12
///   defaultRole:
///     type: aws:iam:Role
///     name: default
///     properties:
///       name: iam_for_lambda_called_from_cloudwatch_logs
///       assumeRolePolicy: ${assumeRole.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
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
/// ### With Cross-Account Invocation Policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let url = function_url::create(
///         "url",
///         FunctionUrlArgs::builder()
///             .authorization_type("AWS_IAM")
///             .function_name("${example.functionName}")
///             .build_struct(),
///     );
///     let urlPermission = permission::create(
///         "urlPermission",
///         PermissionArgs::builder()
///             .action("lambda:InvokeFunctionUrl")
///             .function("${example.functionName}")
///             .function_url_auth_type("AWS_IAM")
///             .principal("arn:aws:iam::444455556666:role/example")
///             .source_account("444455556666")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lambda permission statements using function_name/statement_id with an optional qualifier. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/permission:Permission test_lambda_permission my_test_lambda_function/AllowExecutionFromCloudWatch
/// ```
/// ```sh
/// $ pulumi import aws:lambda/permission:Permission test_lambda_permission my_test_lambda_function:qualifier_name/AllowExecutionFromCloudWatch
/// ```
pub mod permission {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionArgs {
        /// The AWS Lambda action you want to allow in this statement. (e.g., `lambda:InvokeFunction`)
        #[builder(into)]
        pub action: pulumi_wasm_rust::Output<String>,
        /// The Event Source Token to validate.  Used with [Alexa Skills](https://developer.amazon.com/docs/custom-skills/host-a-custom-skill-as-an-aws-lambda-function.html#use-aws-cli).
        #[builder(into, default)]
        pub event_source_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Lambda function whose resource policy you are updating
        #[builder(into)]
        pub function: pulumi_wasm_rust::Output<String>,
        /// Lambda Function URLs [authentication type](https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html). Valid values are: `AWS_IAM` or `NONE`. Only supported for `lambda:InvokeFunctionUrl` action.
        #[builder(into, default)]
        pub function_url_auth_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The principal who is getting this permission e.g., `s3.amazonaws.com`, an AWS account ID, or AWS IAM principal, or AWS service principal such as `events.amazonaws.com` or `sns.amazonaws.com`.
        #[builder(into)]
        pub principal: pulumi_wasm_rust::Output<String>,
        /// The identifier for your organization in AWS Organizations. Use this to grant permissions to all the AWS accounts under this organization.
        ///
        /// [1]: https://developer.amazon.com/docs/custom-skills/host-a-custom-skill-as-an-aws-lambda-function.html#use-aws-cli
        /// [2]: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html
        /// [3]: https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html
        #[builder(into, default)]
        pub principal_org_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Query parameter to specify function version or alias name. The permission will then apply to the specific qualified ARN e.g., `arn:aws:lambda:aws-region:acct-id:function:function-name:2`
        #[builder(into, default)]
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
        /// This parameter is used when allowing cross-account access, or for S3 and SES. The AWS account ID (without a hyphen) of the source owner.
        #[builder(into, default)]
        pub source_account: pulumi_wasm_rust::Output<Option<String>>,
        /// When the principal is an AWS service, the ARN of the specific resource within that service to grant permission to.
        /// Without this, any resource from `principal` will be granted permission – even if that resource is from another account.
        /// For S3, this should be the ARN of the S3 Bucket.
        /// For EventBridge events, this should be the ARN of the EventBridge Rule.
        /// For API Gateway, this should be the ARN of the API, as described [here](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html).
        #[builder(into, default)]
        pub source_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique statement identifier. By default generated by the provider.
        #[builder(into, default)]
        pub statement_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A statement identifier prefix. The provider will generate a unique suffix. Conflicts with `statement_id`.
        #[builder(into, default)]
        pub statement_id_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PermissionResult {
        /// The AWS Lambda action you want to allow in this statement. (e.g., `lambda:InvokeFunction`)
        pub action: pulumi_wasm_rust::Output<String>,
        /// The Event Source Token to validate.  Used with [Alexa Skills](https://developer.amazon.com/docs/custom-skills/host-a-custom-skill-as-an-aws-lambda-function.html#use-aws-cli).
        pub event_source_token: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Lambda function whose resource policy you are updating
        pub function: pulumi_wasm_rust::Output<String>,
        /// Lambda Function URLs [authentication type](https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html). Valid values are: `AWS_IAM` or `NONE`. Only supported for `lambda:InvokeFunctionUrl` action.
        pub function_url_auth_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The principal who is getting this permission e.g., `s3.amazonaws.com`, an AWS account ID, or AWS IAM principal, or AWS service principal such as `events.amazonaws.com` or `sns.amazonaws.com`.
        pub principal: pulumi_wasm_rust::Output<String>,
        /// The identifier for your organization in AWS Organizations. Use this to grant permissions to all the AWS accounts under this organization.
        ///
        /// [1]: https://developer.amazon.com/docs/custom-skills/host-a-custom-skill-as-an-aws-lambda-function.html#use-aws-cli
        /// [2]: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html
        /// [3]: https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html
        pub principal_org_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Query parameter to specify function version or alias name. The permission will then apply to the specific qualified ARN e.g., `arn:aws:lambda:aws-region:acct-id:function:function-name:2`
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
        /// This parameter is used when allowing cross-account access, or for S3 and SES. The AWS account ID (without a hyphen) of the source owner.
        pub source_account: pulumi_wasm_rust::Output<Option<String>>,
        /// When the principal is an AWS service, the ARN of the specific resource within that service to grant permission to.
        /// Without this, any resource from `principal` will be granted permission – even if that resource is from another account.
        /// For S3, this should be the ARN of the S3 Bucket.
        /// For EventBridge events, this should be the ARN of the EventBridge Rule.
        /// For API Gateway, this should be the ARN of the API, as described [here](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html).
        pub source_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique statement identifier. By default generated by the provider.
        pub statement_id: pulumi_wasm_rust::Output<String>,
        /// A statement identifier prefix. The provider will generate a unique suffix. Conflicts with `statement_id`.
        pub statement_id_prefix: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PermissionArgs) -> PermissionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let event_source_token_binding = args.event_source_token.get_inner();
        let function_binding = args.function.get_inner();
        let function_url_auth_type_binding = args.function_url_auth_type.get_inner();
        let principal_binding = args.principal.get_inner();
        let principal_org_id_binding = args.principal_org_id.get_inner();
        let qualifier_binding = args.qualifier.get_inner();
        let source_account_binding = args.source_account.get_inner();
        let source_arn_binding = args.source_arn.get_inner();
        let statement_id_binding = args.statement_id.get_inner();
        let statement_id_prefix_binding = args.statement_id_prefix.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/permission:Permission".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "eventSourceToken".into(),
                    value: &event_source_token_binding,
                },
                register_interface::ObjectField {
                    name: "function".into(),
                    value: &function_binding,
                },
                register_interface::ObjectField {
                    name: "functionUrlAuthType".into(),
                    value: &function_url_auth_type_binding,
                },
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "principalOrgId".into(),
                    value: &principal_org_id_binding,
                },
                register_interface::ObjectField {
                    name: "qualifier".into(),
                    value: &qualifier_binding,
                },
                register_interface::ObjectField {
                    name: "sourceAccount".into(),
                    value: &source_account_binding,
                },
                register_interface::ObjectField {
                    name: "sourceArn".into(),
                    value: &source_arn_binding,
                },
                register_interface::ObjectField {
                    name: "statementId".into(),
                    value: &statement_id_binding,
                },
                register_interface::ObjectField {
                    name: "statementIdPrefix".into(),
                    value: &statement_id_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "eventSourceToken".into(),
                },
                register_interface::ResultField {
                    name: "function".into(),
                },
                register_interface::ResultField {
                    name: "functionUrlAuthType".into(),
                },
                register_interface::ResultField {
                    name: "principal".into(),
                },
                register_interface::ResultField {
                    name: "principalOrgId".into(),
                },
                register_interface::ResultField {
                    name: "qualifier".into(),
                },
                register_interface::ResultField {
                    name: "sourceAccount".into(),
                },
                register_interface::ResultField {
                    name: "sourceArn".into(),
                },
                register_interface::ResultField {
                    name: "statementId".into(),
                },
                register_interface::ResultField {
                    name: "statementIdPrefix".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PermissionResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            event_source_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventSourceToken").unwrap(),
            ),
            function: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("function").unwrap(),
            ),
            function_url_auth_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionUrlAuthType").unwrap(),
            ),
            principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principal").unwrap(),
            ),
            principal_org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalOrgId").unwrap(),
            ),
            qualifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifier").unwrap(),
            ),
            source_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceAccount").unwrap(),
            ),
            source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArn").unwrap(),
            ),
            statement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statementId").unwrap(),
            ),
            statement_id_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statementIdPrefix").unwrap(),
            ),
        }
    }
}
