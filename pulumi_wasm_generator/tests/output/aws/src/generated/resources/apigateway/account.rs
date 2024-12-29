/// Provides a settings of an API Gateway Account. Settings is applied region-wide per `provider` block.
///
/// > **Note:** By default, destroying this resource will keep your account settings intact. Set `reset_on_delete` to `true` to reset the account setttings to default. In a future major version of the provider, destroying the resource will reset account settings.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["apigateway.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let cloudwatch = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["logs:CreateLogGroup", "logs:CreateLogStream",
///                     "logs:DescribeLogGroups", "logs:DescribeLogStreams",
///                     "logs:PutLogEvents", "logs:GetLogEvents", "logs:FilterLogEvents",])
///                     .effect("Allow").resources(vec!["*",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let cloudwatchRole = role::create(
///         "cloudwatchRole",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("api_gateway_cloudwatch_global")
///             .build_struct(),
///     );
///     let cloudwatchRolePolicy = role_policy::create(
///         "cloudwatchRolePolicy",
///         RolePolicyArgs::builder()
///             .name("default")
///             .policy("${cloudwatch.json}")
///             .role("${cloudwatchRole.id}")
///             .build_struct(),
///     );
///     let demo = account::create(
///         "demo",
///         AccountArgs::builder()
///             .cloudwatch_role_arn("${cloudwatchRole.arn}")
///             .build_struct(),
///     );
/// }
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// ARN of an IAM role for CloudWatch (to allow logging & monitoring). See more [in AWS Docs](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-stage-settings.html#how-to-stage-settings-console). Logging & monitoring can be enabled/disabled and otherwise tuned on the API Gateway Stage level.
        #[builder(into, default)]
        pub cloudwatch_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, destroying the resource will reset account settings to default, otherwise account settings are not modified.
        /// Defaults to `false`.
        /// Will be removed in a future major version of the provider.
        #[builder(into, default)]
        pub reset_on_delete: pulumi_wasm_rust::Output<Option<bool>>,
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
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloudwatch_role_arn_binding = args.cloudwatch_role_arn.get_inner();
        let reset_on_delete_binding = args.reset_on_delete.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/account:Account".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKeyVersion".into(),
                },
                register_interface::ResultField {
                    name: "cloudwatchRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "features".into(),
                },
                register_interface::ResultField {
                    name: "resetOnDelete".into(),
                },
                register_interface::ResultField {
                    name: "throttleSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            api_key_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeyVersion").unwrap(),
            ),
            cloudwatch_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudwatchRoleArn").unwrap(),
            ),
            features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("features").unwrap(),
            ),
            reset_on_delete: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resetOnDelete").unwrap(),
            ),
            throttle_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throttleSettings").unwrap(),
            ),
        }
    }
}
