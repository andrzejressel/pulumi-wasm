/// Provides a settings of an API Gateway Account. Settings is applied region-wide per `provider` block.
///
/// > **Note:** By default, destroying this resource will keep your account settings intact. Set `reset_on_delete` to `true` to reset the account setttings to default. In a future major version of the provider, destroying the resource will reset account settings.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   demo:
///     type: aws:apigateway:Account
///     properties:
///       cloudwatchRoleArn: ${cloudwatchRole.arn}
///   cloudwatchRole:
///     type: aws:iam:Role
///     name: cloudwatch
///     properties:
///       name: api_gateway_cloudwatch_global
///       assumeRolePolicy: ${assumeRole.json}
///   cloudwatchRolePolicy:
///     type: aws:iam:RolePolicy
///     name: cloudwatch
///     properties:
///       name: default
///       role: ${cloudwatchRole.id}
///       policy: ${cloudwatch.json}
/// variables:
///   assumeRole:
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
///   cloudwatch:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - logs:CreateLogGroup
///               - logs:CreateLogStream
///               - logs:DescribeLogGroups
///               - logs:DescribeLogStreams
///               - logs:PutLogEvents
///               - logs:GetLogEvents
///               - logs:FilterLogEvents
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import API Gateway Accounts using the word `api-gateway-account`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/account:Account demo api-gateway-account
/// ```
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// ARN of an IAM role for CloudWatch (to allow logging & monitoring). See more [in AWS Docs](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-stage-settings.html#how-to-stage-settings-console). Logging & monitoring can be enabled/disabled and otherwise tuned on the API Gateway Stage level.
        #[builder(into, default)]
        pub cloudwatch_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If `true`, destroying the resource will reset account settings to default, otherwise account settings are not modified.
        /// Defaults to `false`.
        /// Will be removed in a future major version of the provider.
        #[builder(into, default)]
        pub reset_on_delete: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// The version of the API keys used for the account.
        pub api_key_version: pulumi_wasm_rust::Output<String>,
        /// ARN of an IAM role for CloudWatch (to allow logging & monitoring). See more [in AWS Docs](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-stage-settings.html#how-to-stage-settings-console). Logging & monitoring can be enabled/disabled and otherwise tuned on the API Gateway Stage level.
        pub cloudwatch_role_arn: pulumi_wasm_rust::Output<String>,
        /// A list of features supported for the account.
        pub features: pulumi_wasm_rust::Output<Vec<String>>,
        /// If `true`, destroying the resource will reset account settings to default, otherwise account settings are not modified.
        /// Defaults to `false`.
        /// Will be removed in a future major version of the provider.
        pub reset_on_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// Account-Level throttle settings. See exported fields below.
        pub throttle_settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::apigateway::AccountThrottleSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloudwatch_role_arn_binding = args
            .cloudwatch_role_arn
            .get_output(context)
            .get_inner();
        let reset_on_delete_binding = args
            .reset_on_delete
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudwatchRoleArn".into(),
                    value: &cloudwatch_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resetOnDelete".into(),
                    value: &reset_on_delete_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountResult {
            api_key_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiKeyVersion"),
            ),
            cloudwatch_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudwatchRoleArn"),
            ),
            features: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("features"),
            ),
            reset_on_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resetOnDelete"),
            ),
            throttle_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("throttleSettings"),
            ),
        }
    }
}
