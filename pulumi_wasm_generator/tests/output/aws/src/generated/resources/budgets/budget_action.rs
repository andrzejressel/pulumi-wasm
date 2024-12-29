/// Provides a budget action resource. Budget actions are cost savings controls that run either automatically on your behalf or by using a workflow approval process.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleBudgetAction:
///     type: aws:budgets:BudgetAction
///     name: example
///     properties:
///       budgetName: ${exampleBudget.name}
///       actionType: APPLY_IAM_POLICY
///       approvalModel: AUTOMATIC
///       notificationType: ACTUAL
///       executionRoleArn: ${exampleRole.arn}
///       actionThreshold:
///         actionThresholdType: ABSOLUTE_VALUE
///         actionThresholdValue: 100
///       definition:
///         iamActionDefinition:
///           policyArn: ${examplePolicy.arn}
///           roles:
///             - ${exampleRole.name}
///       subscribers:
///         - address: example@example.example
///           subscriptionType: EMAIL
///       tags:
///         Tag1: Value1
///         Tag2: Value2
///   examplePolicy:
///     type: aws:iam:Policy
///     name: example
///     properties:
///       name: example
///       description: My example policy
///       policy: ${example.json}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleBudget:
///     type: aws:budgets:Budget
///     name: example
///     properties:
///       name: example
///       budgetType: USAGE
///       limitAmount: '10.0'
///       limitUnit: dollars
///       timePeriodStart: 2006-01-02_15:04
///       timeUnit: MONTHLY
/// variables:
///   example:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ec2:Describe*
///             resources:
///               - '*'
///   current:
///     fn::invoke:
///       Function: aws:getPartition
///       Arguments: {}
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - budgets.${current.dnsSuffix}
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import budget actions using `AccountID:ActionID:BudgetName`. For example:
///
/// ```sh
/// $ pulumi import aws:budgets/budgetAction:BudgetAction myBudget 123456789012:some-id:myBudget
/// ```
pub mod budget_action {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetActionArgs {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The trigger threshold of the action. See Action Threshold.
        #[builder(into)]
        pub action_threshold: pulumi_wasm_rust::Output<
            super::super::types::budgets::BudgetActionActionThreshold,
        >,
        /// The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition. Valid values are `APPLY_IAM_POLICY`, `APPLY_SCP_POLICY`, and `RUN_SSM_DOCUMENTS`.
        #[builder(into)]
        pub action_type: pulumi_wasm_rust::Output<String>,
        /// This specifies if the action needs manual or automatic approval. Valid values are `AUTOMATIC` and `MANUAL`.
        #[builder(into)]
        pub approval_model: pulumi_wasm_rust::Output<String>,
        /// The name of a budget.
        #[builder(into)]
        pub budget_name: pulumi_wasm_rust::Output<String>,
        /// Specifies all of the type-specific parameters. See Definition.
        #[builder(into)]
        pub definition: pulumi_wasm_rust::Output<
            super::super::types::budgets::BudgetActionDefinition,
        >,
        /// The role passed for action execution and reversion. Roles and actions must be in the same account.
        #[builder(into)]
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// The type of a notification. Valid values are `ACTUAL` or `FORECASTED`.
        #[builder(into)]
        pub notification_type: pulumi_wasm_rust::Output<String>,
        /// A list of subscribers. See Subscriber.
        #[builder(into)]
        pub subscribers: pulumi_wasm_rust::Output<
            Vec<super::super::types::budgets::BudgetActionSubscriber>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BudgetActionResult {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The id of the budget action.
        pub action_id: pulumi_wasm_rust::Output<String>,
        /// The trigger threshold of the action. See Action Threshold.
        pub action_threshold: pulumi_wasm_rust::Output<
            super::super::types::budgets::BudgetActionActionThreshold,
        >,
        /// The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition. Valid values are `APPLY_IAM_POLICY`, `APPLY_SCP_POLICY`, and `RUN_SSM_DOCUMENTS`.
        pub action_type: pulumi_wasm_rust::Output<String>,
        /// This specifies if the action needs manual or automatic approval. Valid values are `AUTOMATIC` and `MANUAL`.
        pub approval_model: pulumi_wasm_rust::Output<String>,
        /// The ARN of the budget action.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of a budget.
        pub budget_name: pulumi_wasm_rust::Output<String>,
        /// Specifies all of the type-specific parameters. See Definition.
        pub definition: pulumi_wasm_rust::Output<
            super::super::types::budgets::BudgetActionDefinition,
        >,
        /// The role passed for action execution and reversion. Roles and actions must be in the same account.
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// The type of a notification. Valid values are `ACTUAL` or `FORECASTED`.
        pub notification_type: pulumi_wasm_rust::Output<String>,
        /// The status of the budget action.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A list of subscribers. See Subscriber.
        pub subscribers: pulumi_wasm_rust::Output<
            Vec<super::super::types::budgets::BudgetActionSubscriber>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BudgetActionArgs) -> BudgetActionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let action_threshold_binding = args.action_threshold.get_inner();
        let action_type_binding = args.action_type.get_inner();
        let approval_model_binding = args.approval_model.get_inner();
        let budget_name_binding = args.budget_name.get_inner();
        let definition_binding = args.definition.get_inner();
        let execution_role_arn_binding = args.execution_role_arn.get_inner();
        let notification_type_binding = args.notification_type.get_inner();
        let subscribers_binding = args.subscribers.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:budgets/budgetAction:BudgetAction".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "actionThreshold".into(),
                    value: &action_threshold_binding,
                },
                register_interface::ObjectField {
                    name: "actionType".into(),
                    value: &action_type_binding,
                },
                register_interface::ObjectField {
                    name: "approvalModel".into(),
                    value: &approval_model_binding,
                },
                register_interface::ObjectField {
                    name: "budgetName".into(),
                    value: &budget_name_binding,
                },
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "notificationType".into(),
                    value: &notification_type_binding,
                },
                register_interface::ObjectField {
                    name: "subscribers".into(),
                    value: &subscribers_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "actionId".into(),
                },
                register_interface::ResultField {
                    name: "actionThreshold".into(),
                },
                register_interface::ResultField {
                    name: "actionType".into(),
                },
                register_interface::ResultField {
                    name: "approvalModel".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "budgetName".into(),
                },
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "executionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "notificationType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subscribers".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BudgetActionResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            action_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionId").unwrap(),
            ),
            action_threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionThreshold").unwrap(),
            ),
            action_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionType").unwrap(),
            ),
            approval_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalModel").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            budget_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("budgetName").unwrap(),
            ),
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionRoleArn").unwrap(),
            ),
            notification_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subscribers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscribers").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
