/// Manages a Config Organization Managed Rule. More information about these rules can be found in the [Enabling AWS Config Rules Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/config-rule-multi-account-deployment.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. For working with Organization Custom Rules (those invoking a custom Lambda Function), see the `aws.cfg.OrganizationCustomRule` resource.
///
/// > **NOTE:** This resource must be created in the Organization master account and rules will include the master account unless its ID is added to the `excluded_accounts` argument.
///
/// > **NOTE:** Every Organization account except those configured in the `excluded_accounts` argument must have a Configuration Recorder with proper IAM permissions before the rule will successfully create or update. See also the `aws.cfg.Recorder` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization::create(
///         "example",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(
///                 vec!["config-multiaccountsetup.amazonaws.com",],
///             )
///             .feature_set("ALL")
///             .build_struct(),
///     );
///     let exampleOrganizationManagedRule = organization_managed_rule::create(
///         "exampleOrganizationManagedRule",
///         OrganizationManagedRuleArgs::builder()
///             .name("example")
///             .rule_identifier("IAM_PASSWORD_POLICY")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Config Organization Managed Rules using the name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/organizationManagedRule:OrganizationManagedRule example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_managed_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationManagedRuleArgs {
        /// Description of the rule
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of AWS account identifiers to exclude from the rule
        #[builder(into, default)]
        pub excluded_accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A string in JSON format that is passed to the AWS Config Rule Lambda Function
        #[builder(into, default)]
        pub input_parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
        #[builder(into, default)]
        pub maximum_execution_frequency: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the rule
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the AWS resource to evaluate
        #[builder(into, default)]
        pub resource_id_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of types of AWS resources to evaluate
        #[builder(into, default)]
        pub resource_types_scopes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Identifier of an available AWS Config Managed Rule to call. For available values, see the [List of AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/managed-rules-by-aws-config.html) documentation
        #[builder(into)]
        pub rule_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tag key of AWS resources to evaluate
        #[builder(into, default)]
        pub tag_key_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tag value of AWS resources to evaluate
        #[builder(into, default)]
        pub tag_value_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationManagedRuleResult {
        /// Amazon Resource Name (ARN) of the rule
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the rule
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of AWS account identifiers to exclude from the rule
        pub excluded_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A string in JSON format that is passed to the AWS Config Rule Lambda Function
        pub input_parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
        pub maximum_execution_frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the rule
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the AWS resource to evaluate
        pub resource_id_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of types of AWS resources to evaluate
        pub resource_types_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Identifier of an available AWS Config Managed Rule to call. For available values, see the [List of AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/managed-rules-by-aws-config.html) documentation
        pub rule_identifier: pulumi_gestalt_rust::Output<String>,
        /// Tag key of AWS resources to evaluate
        pub tag_key_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Tag value of AWS resources to evaluate
        pub tag_value_scope: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationManagedRuleArgs,
    ) -> OrganizationManagedRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let excluded_accounts_binding = args.excluded_accounts.get_output(context);
        let input_parameters_binding = args.input_parameters.get_output(context);
        let maximum_execution_frequency_binding = args
            .maximum_execution_frequency
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_id_scope_binding = args.resource_id_scope.get_output(context);
        let resource_types_scopes_binding = args
            .resource_types_scopes
            .get_output(context);
        let rule_identifier_binding = args.rule_identifier.get_output(context);
        let tag_key_scope_binding = args.tag_key_scope.get_output(context);
        let tag_value_scope_binding = args.tag_value_scope.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/organizationManagedRule:OrganizationManagedRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludedAccounts".into(),
                    value: excluded_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputParameters".into(),
                    value: input_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumExecutionFrequency".into(),
                    value: maximum_execution_frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceIdScope".into(),
                    value: resource_id_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTypesScopes".into(),
                    value: resource_types_scopes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleIdentifier".into(),
                    value: rule_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagKeyScope".into(),
                    value: tag_key_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagValueScope".into(),
                    value: tag_value_scope_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationManagedRuleResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            excluded_accounts: o.get_field("excludedAccounts"),
            input_parameters: o.get_field("inputParameters"),
            maximum_execution_frequency: o.get_field("maximumExecutionFrequency"),
            name: o.get_field("name"),
            resource_id_scope: o.get_field("resourceIdScope"),
            resource_types_scopes: o.get_field("resourceTypesScopes"),
            rule_identifier: o.get_field("ruleIdentifier"),
            tag_key_scope: o.get_field("tagKeyScope"),
            tag_value_scope: o.get_field("tagValueScope"),
        }
    }
}
