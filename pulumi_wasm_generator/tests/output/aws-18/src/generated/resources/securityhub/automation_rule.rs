/// Resource for managing an AWS Security Hub Automation Rule.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:securityhub:AutomationRule
///     properties:
///       description: Elevate finding severity to CRITICAL when specific resources such as an S3 bucket is at risk
///       ruleName: Elevate severity of findings that relate to important resources
///       ruleOrder: 1
///       actions:
///         - findingFieldsUpdate:
///             severity:
///               label: CRITICAL
///               product: '0.0'
///             note:
///               text: This is a critical resource. Please review ASAP.
///               updatedBy: sechub-automation
///             types:
///               - Software and Configuration Checks/Industry and Regulatory Standards
///             userDefinedFields:
///               key: value
///           type: FINDING_FIELDS_UPDATE
///       criteria:
///         resourceIds:
///           - comparison: EQUALS
///             value: arn:aws:s3:::examplebucket/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub automation rule using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/automationRule:AutomationRule example arn:aws:securityhub:us-west-2:123456789012:automation-rule/473eddde-f5c4-4ae5-85c7-e922f271fffc
/// ```
pub mod automation_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomationRuleArgs {
        /// A block that specifies one or more actions to update finding fields if a finding matches the conditions specified in `Criteria`. Documented below.
        #[builder(into, default)]
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::securityhub::AutomationRuleAction>>,
        >,
        /// A block that specifies a set of ASFF finding field attributes and corresponding expected values that Security Hub uses to filter findings. Documented below.
        #[builder(into, default)]
        pub criteria: pulumi_wasm_rust::Output<
            Option<super::super::types::securityhub::AutomationRuleCriteria>,
        >,
        /// The description of the rule.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies whether a rule is the last to be applied with respect to a finding that matches the rule criteria. Defaults to `false`.
        #[builder(into, default)]
        pub is_terminal: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the rule.
        #[builder(into)]
        pub rule_name: pulumi_wasm_rust::Output<String>,
        /// An integer ranging from 1 to 1000 that represents the order in which the rule action is applied to findings. Security Hub applies rules with lower values for this parameter first.
        #[builder(into)]
        pub rule_order: pulumi_wasm_rust::Output<i32>,
        /// Whether the rule is active after it is created.
        #[builder(into, default)]
        pub rule_status: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AutomationRuleResult {
        /// A block that specifies one or more actions to update finding fields if a finding matches the conditions specified in `Criteria`. Documented below.
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::securityhub::AutomationRuleAction>>,
        >,
        /// The ARN of the Security Hub automation rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A block that specifies a set of ASFF finding field attributes and corresponding expected values that Security Hub uses to filter findings. Documented below.
        pub criteria: pulumi_wasm_rust::Output<
            Option<super::super::types::securityhub::AutomationRuleCriteria>,
        >,
        /// The description of the rule.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies whether a rule is the last to be applied with respect to a finding that matches the rule criteria. Defaults to `false`.
        pub is_terminal: pulumi_wasm_rust::Output<bool>,
        /// The name of the rule.
        pub rule_name: pulumi_wasm_rust::Output<String>,
        /// An integer ranging from 1 to 1000 that represents the order in which the rule action is applied to findings. Security Hub applies rules with lower values for this parameter first.
        pub rule_order: pulumi_wasm_rust::Output<i32>,
        /// Whether the rule is active after it is created.
        pub rule_status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AutomationRuleArgs) -> AutomationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_inner();
        let criteria_binding = args.criteria.get_inner();
        let description_binding = args.description.get_inner();
        let is_terminal_binding = args.is_terminal.get_inner();
        let rule_name_binding = args.rule_name.get_inner();
        let rule_order_binding = args.rule_order.get_inner();
        let rule_status_binding = args.rule_status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/automationRule:AutomationRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "criteria".into(),
                    value: &criteria_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "isTerminal".into(),
                    value: &is_terminal_binding,
                },
                register_interface::ObjectField {
                    name: "ruleName".into(),
                    value: &rule_name_binding,
                },
                register_interface::ObjectField {
                    name: "ruleOrder".into(),
                    value: &rule_order_binding,
                },
                register_interface::ObjectField {
                    name: "ruleStatus".into(),
                    value: &rule_status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "criteria".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "isTerminal".into(),
                },
                register_interface::ResultField {
                    name: "ruleName".into(),
                },
                register_interface::ResultField {
                    name: "ruleOrder".into(),
                },
                register_interface::ResultField {
                    name: "ruleStatus".into(),
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
        AutomationRuleResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            criteria: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("criteria").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            is_terminal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isTerminal").unwrap(),
            ),
            rule_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleName").unwrap(),
            ),
            rule_order: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleOrder").unwrap(),
            ),
            rule_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleStatus").unwrap(),
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
