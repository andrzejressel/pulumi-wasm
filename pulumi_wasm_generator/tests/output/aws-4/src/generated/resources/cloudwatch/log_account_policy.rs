/// Provides a CloudWatch Log Account Policy resource.
///
/// ## Example Usage
///
/// ### Account Data Protection Policy
///
/// ```yaml
/// resources:
///   dataProtection:
///     type: aws:cloudwatch:LogAccountPolicy
///     name: data_protection
///     properties:
///       policyName: data-protection
///       policyType: DATA_PROTECTION_POLICY
///       policyDocument:
///         fn::toJSON:
///           Name: DataProtection
///           Version: 2021-06-01
///           Statement:
///             - Sid: Audit
///               DataIdentifier:
///                 - arn:aws:dataprotection::aws:data-identifier/EmailAddress
///               Operation:
///                 Audit:
///                   FindingsDestination: {}
///             - Sid: Redact
///               DataIdentifier:
///                 - arn:aws:dataprotection::aws:data-identifier/EmailAddress
///               Operation:
///                 Deidentify:
///                   MaskConfig: {}
/// ```
///
/// ### Subscription Filter Policy
///
/// ```yaml
/// resources:
///   subscriptionFilter:
///     type: aws:cloudwatch:LogAccountPolicy
///     name: subscription_filter
///     properties:
///       policyName: subscription-filter
///       policyType: SUBSCRIPTION_FILTER_POLICY
///       policyDocument:
///         fn::toJSON:
///           DestinationArn: ${test.arn}
///           FilterPattern: test
///       selectionCriteria: LogGroupName NOT IN ["excluded_log_group_name"]
/// ```
///
/// ### Field Index Policy
///
/// ```yaml
/// resources:
///   fieldIndex:
///     type: aws:cloudwatch:LogAccountPolicy
///     name: field_index
///     properties:
///       policyName: field-index
///       policyType: FIELD_INDEX_POLICY
///       policyDocument:
///         fn::toJSON:
///           Fields:
///             - field1
///             - field2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import this resource using the `policy_name` and `policy_type` separated by `:`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logAccountPolicy:LogAccountPolicy example "my-account-policy:SUBSCRIPTION_FILTER_POLICY"
/// ```
pub mod log_account_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogAccountPolicyArgs {
        /// Text of the account policy. Refer to the [AWS docs](https://docs.aws.amazon.com/cli/latest/reference/logs/put-account-policy.html) for more information.
        #[builder(into)]
        pub policy_document: pulumi_wasm_rust::Output<String>,
        /// Name of the account policy.
        #[builder(into)]
        pub policy_name: pulumi_wasm_rust::Output<String>,
        /// Type of account policy. One of `DATA_PROTECTION_POLICY`, `SUBSCRIPTION_FILTER_POLICY`, `FIELD_INDEX_POLICY` or `TRANSFORMER_POLICY`. You can have one account policy per type in an account.
        #[builder(into)]
        pub policy_type: pulumi_wasm_rust::Output<String>,
        /// Currently defaults to and only accepts the value: `ALL`.
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Criteria for applying a subscription filter policy to a selection of log groups. The only allowable criteria selector is `LogGroupName NOT IN []`.
        #[builder(into, default)]
        pub selection_criteria: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogAccountPolicyResult {
        /// Text of the account policy. Refer to the [AWS docs](https://docs.aws.amazon.com/cli/latest/reference/logs/put-account-policy.html) for more information.
        pub policy_document: pulumi_wasm_rust::Output<String>,
        /// Name of the account policy.
        pub policy_name: pulumi_wasm_rust::Output<String>,
        /// Type of account policy. One of `DATA_PROTECTION_POLICY`, `SUBSCRIPTION_FILTER_POLICY`, `FIELD_INDEX_POLICY` or `TRANSFORMER_POLICY`. You can have one account policy per type in an account.
        pub policy_type: pulumi_wasm_rust::Output<String>,
        /// Currently defaults to and only accepts the value: `ALL`.
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Criteria for applying a subscription filter policy to a selection of log groups. The only allowable criteria selector is `LogGroupName NOT IN []`.
        pub selection_criteria: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LogAccountPolicyArgs) -> LogAccountPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_document_binding = args.policy_document.get_inner();
        let policy_name_binding = args.policy_name.get_inner();
        let policy_type_binding = args.policy_type.get_inner();
        let scope_binding = args.scope.get_inner();
        let selection_criteria_binding = args.selection_criteria.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logAccountPolicy:LogAccountPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding,
                },
                register_interface::ObjectField {
                    name: "policyName".into(),
                    value: &policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "selectionCriteria".into(),
                    value: &selection_criteria_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policyDocument".into(),
                },
                register_interface::ResultField {
                    name: "policyName".into(),
                },
                register_interface::ResultField {
                    name: "policyType".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "selectionCriteria".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogAccountPolicyResult {
            policy_document: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDocument").unwrap(),
            ),
            policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyName").unwrap(),
            ),
            policy_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyType").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            selection_criteria: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selectionCriteria").unwrap(),
            ),
        }
    }
}
