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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_account_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogAccountPolicyArgs {
        /// Text of the account policy. Refer to the [AWS docs](https://docs.aws.amazon.com/cli/latest/reference/logs/put-account-policy.html) for more information.
        #[builder(into)]
        pub policy_document: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the account policy.
        #[builder(into)]
        pub policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of account policy. One of `DATA_PROTECTION_POLICY`, `SUBSCRIPTION_FILTER_POLICY`, `FIELD_INDEX_POLICY` or `TRANSFORMER_POLICY`. You can have one account policy per type in an account.
        #[builder(into)]
        pub policy_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Currently defaults to and only accepts the value: `ALL`.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Criteria for applying a subscription filter policy to a selection of log groups. The only allowable criteria selector is `LogGroupName NOT IN []`.
        #[builder(into, default)]
        pub selection_criteria: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogAccountPolicyResult {
        /// Text of the account policy. Refer to the [AWS docs](https://docs.aws.amazon.com/cli/latest/reference/logs/put-account-policy.html) for more information.
        pub policy_document: pulumi_gestalt_rust::Output<String>,
        /// Name of the account policy.
        pub policy_name: pulumi_gestalt_rust::Output<String>,
        /// Type of account policy. One of `DATA_PROTECTION_POLICY`, `SUBSCRIPTION_FILTER_POLICY`, `FIELD_INDEX_POLICY` or `TRANSFORMER_POLICY`. You can have one account policy per type in an account.
        pub policy_type: pulumi_gestalt_rust::Output<String>,
        /// Currently defaults to and only accepts the value: `ALL`.
        pub scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Criteria for applying a subscription filter policy to a selection of log groups. The only allowable criteria selector is `LogGroupName NOT IN []`.
        pub selection_criteria: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogAccountPolicyArgs,
    ) -> LogAccountPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_document_binding = args.policy_document.get_output(context);
        let policy_name_binding = args.policy_name.get_output(context);
        let policy_type_binding = args.policy_type.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let selection_criteria_binding = args.selection_criteria.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logAccountPolicy:LogAccountPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDocument".into(),
                    value: policy_document_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyName".into(),
                    value: policy_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyType".into(),
                    value: policy_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selectionCriteria".into(),
                    value: selection_criteria_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogAccountPolicyResult {
            policy_document: o.get_field("policyDocument"),
            policy_name: o.get_field("policyName"),
            policy_type: o.get_field("policyType"),
            scope: o.get_field("scope"),
            selection_criteria: o.get_field("selectionCriteria"),
        }
    }
}
