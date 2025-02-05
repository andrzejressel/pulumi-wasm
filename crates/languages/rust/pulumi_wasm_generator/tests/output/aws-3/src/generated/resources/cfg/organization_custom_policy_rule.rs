/// Manages a Config Organization Custom Policy Rule. More information about these rules can be found in the [Enabling AWS Config Rules Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/config-rule-multi-account-deployment.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. For working with Organization Managed Rules (those invoking an AWS managed rule), see the `aws_config_organization_managed__rule` resource.
///
/// > **NOTE:** This resource must be created in the Organization master account and rules will include the master account unless its ID is added to the `excluded_accounts` argument.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization_custom_policy_rule::create(
///         "example",
///         OrganizationCustomPolicyRuleArgs::builder()
///             .name("example_rule_name")
///             .policy_runtime("guard-2.x.x")
///             .policy_text(
///                 "let status = ['ACTIVE']\n\nrule tableisactive when\n    resourceType == \"AWS::DynamoDB::Table\" {\n    configuration.tableStatus == %status\n}\n\nrule checkcompliance when\n    resourceType == \"AWS::DynamoDB::Table\"\n    tableisactive {\n        let pitr = supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus\n        %pitr == \"ENABLED\"\n    }\n",
///             )
///             .resource_types_scopes(vec!["AWS::DynamoDB::Table",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a Config Organization Custom Policy Rule using the `name` argument. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/organizationCustomPolicyRule:OrganizationCustomPolicyRule example example_rule_name
/// ```
pub mod organization_custom_policy_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationCustomPolicyRuleArgs {
        /// List of AWS account identifiers to exclude from the rule
        #[builder(into, default)]
        pub debug_log_delivery_accounts: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Description of the rule
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of AWS account identifiers to exclude from the rule
        #[builder(into, default)]
        pub excluded_accounts: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A string in JSON format that is passed to the AWS Config Rule Lambda Function
        #[builder(into, default)]
        pub input_parameters: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
        #[builder(into, default)]
        pub maximum_execution_frequency: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// name of the rule
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// runtime system for your organization AWS Config Custom Policy rules
        #[builder(into)]
        pub policy_runtime: pulumi_wasm_rust::InputOrOutput<String>,
        /// policy definition containing the logic for your organization AWS Config Custom Policy rule
        #[builder(into)]
        pub policy_text: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifier of the AWS resource to evaluate
        #[builder(into, default)]
        pub resource_id_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of types of AWS resources to evaluate
        #[builder(into, default)]
        pub resource_types_scopes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Tag key of AWS resources to evaluate
        #[builder(into, default)]
        pub tag_key_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tag value of AWS resources to evaluate
        #[builder(into, default)]
        pub tag_value_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub trigger_types: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationCustomPolicyRuleResult {
        /// Amazon Resource Name (ARN) of the rule
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of AWS account identifiers to exclude from the rule
        pub debug_log_delivery_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Description of the rule
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AWS account identifiers to exclude from the rule
        pub excluded_accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A string in JSON format that is passed to the AWS Config Rule Lambda Function
        pub input_parameters: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
        pub maximum_execution_frequency: pulumi_wasm_rust::Output<Option<String>>,
        /// name of the rule
        pub name: pulumi_wasm_rust::Output<String>,
        /// runtime system for your organization AWS Config Custom Policy rules
        pub policy_runtime: pulumi_wasm_rust::Output<String>,
        /// policy definition containing the logic for your organization AWS Config Custom Policy rule
        pub policy_text: pulumi_wasm_rust::Output<String>,
        /// Identifier of the AWS resource to evaluate
        pub resource_id_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// List of types of AWS resources to evaluate
        pub resource_types_scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Tag key of AWS resources to evaluate
        pub tag_key_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Tag value of AWS resources to evaluate
        pub tag_value_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`
        ///
        /// The following arguments are optional:
        pub trigger_types: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OrganizationCustomPolicyRuleArgs,
    ) -> OrganizationCustomPolicyRuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let debug_log_delivery_accounts_binding = args
            .debug_log_delivery_accounts
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let excluded_accounts_binding = args
            .excluded_accounts
            .get_output(context)
            .get_inner();
        let input_parameters_binding = args
            .input_parameters
            .get_output(context)
            .get_inner();
        let maximum_execution_frequency_binding = args
            .maximum_execution_frequency
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_runtime_binding = args.policy_runtime.get_output(context).get_inner();
        let policy_text_binding = args.policy_text.get_output(context).get_inner();
        let resource_id_scope_binding = args
            .resource_id_scope
            .get_output(context)
            .get_inner();
        let resource_types_scopes_binding = args
            .resource_types_scopes
            .get_output(context)
            .get_inner();
        let tag_key_scope_binding = args.tag_key_scope.get_output(context).get_inner();
        let tag_value_scope_binding = args
            .tag_value_scope
            .get_output(context)
            .get_inner();
        let trigger_types_binding = args.trigger_types.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/organizationCustomPolicyRule:OrganizationCustomPolicyRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "debugLogDeliveryAccounts".into(),
                    value: &debug_log_delivery_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "excludedAccounts".into(),
                    value: &excluded_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "inputParameters".into(),
                    value: &input_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "maximumExecutionFrequency".into(),
                    value: &maximum_execution_frequency_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyRuntime".into(),
                    value: &policy_runtime_binding,
                },
                register_interface::ObjectField {
                    name: "policyText".into(),
                    value: &policy_text_binding,
                },
                register_interface::ObjectField {
                    name: "resourceIdScope".into(),
                    value: &resource_id_scope_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTypesScopes".into(),
                    value: &resource_types_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "tagKeyScope".into(),
                    value: &tag_key_scope_binding,
                },
                register_interface::ObjectField {
                    name: "tagValueScope".into(),
                    value: &tag_value_scope_binding,
                },
                register_interface::ObjectField {
                    name: "triggerTypes".into(),
                    value: &trigger_types_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OrganizationCustomPolicyRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            debug_log_delivery_accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("debugLogDeliveryAccounts"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            excluded_accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludedAccounts"),
            ),
            input_parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputParameters"),
            ),
            maximum_execution_frequency: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maximumExecutionFrequency"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy_runtime: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyRuntime"),
            ),
            policy_text: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyText"),
            ),
            resource_id_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceIdScope"),
            ),
            resource_types_scopes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceTypesScopes"),
            ),
            tag_key_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagKeyScope"),
            ),
            tag_value_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagValueScope"),
            ),
            trigger_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("triggerTypes"),
            ),
        }
    }
}
