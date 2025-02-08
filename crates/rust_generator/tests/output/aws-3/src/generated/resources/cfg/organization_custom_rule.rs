/// Manages a Config Organization Custom Rule. More information about these rules can be found in the [Enabling AWS Config Rules Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/config-rule-multi-account-deployment.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. For working with Organization Managed Rules (those invoking an AWS managed rule), see the `aws_config_organization_managed__rule` resource.
///
/// > **NOTE:** This resource must be created in the Organization master account and rules will include the master account unless its ID is added to the `excluded_accounts` argument.
///
/// > **NOTE:** The proper Lambda permission to allow the AWS Config service invoke the Lambda Function must be in place before the rule will successfully create or update. See also the `aws.lambda.Permission` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = permission::create(
///         "example",
///         PermissionArgs::builder()
///             .action("lambda:InvokeFunction")
///             .function("${exampleAwsLambdaFunction.arn}")
///             .principal("config.amazonaws.com")
///             .statement_id("AllowExecutionFromConfig")
///             .build_struct(),
///     );
///     let exampleOrganization = organization::create(
///         "exampleOrganization",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(
///                 vec!["config-multiaccountsetup.amazonaws.com",],
///             )
///             .feature_set("ALL")
///             .build_struct(),
///     );
///     let exampleOrganizationCustomRule = organization_custom_rule::create(
///         "exampleOrganizationCustomRule",
///         OrganizationCustomRuleArgs::builder()
///             .lambda_function_arn("${exampleAwsLambdaFunction.arn}")
///             .name("example")
///             .trigger_types(vec!["ConfigurationItemChangeNotification",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Config Organization Custom Rules using the name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/organizationCustomRule:OrganizationCustomRule example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_custom_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationCustomRuleArgs {
        /// Description of the rule
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of AWS account identifiers to exclude from the rule
        #[builder(into, default)]
        pub excluded_accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A string in JSON format that is passed to the AWS Config Rule Lambda Function
        #[builder(into, default)]
        pub input_parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the rule Lambda Function
        #[builder(into)]
        pub lambda_function_arn: pulumi_gestalt_rust::InputOrOutput<String>,
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
        /// Tag key of AWS resources to evaluate
        #[builder(into, default)]
        pub tag_key_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tag value of AWS resources to evaluate
        #[builder(into, default)]
        pub tag_value_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`, and `ScheduledNotification`
        #[builder(into)]
        pub trigger_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationCustomRuleResult {
        /// Amazon Resource Name (ARN) of the rule
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the rule
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of AWS account identifiers to exclude from the rule
        pub excluded_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A string in JSON format that is passed to the AWS Config Rule Lambda Function
        pub input_parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the rule Lambda Function
        pub lambda_function_arn: pulumi_gestalt_rust::Output<String>,
        /// The maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
        pub maximum_execution_frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the rule
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the AWS resource to evaluate
        pub resource_id_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of types of AWS resources to evaluate
        pub resource_types_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Tag key of AWS resources to evaluate
        pub tag_key_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Tag value of AWS resources to evaluate
        pub tag_value_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`, and `ScheduledNotification`
        pub trigger_types: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OrganizationCustomRuleArgs,
    ) -> OrganizationCustomRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let excluded_accounts_binding = args
            .excluded_accounts
            .get_output(context)
            .get_inner();
        let input_parameters_binding = args
            .input_parameters
            .get_output(context)
            .get_inner();
        let lambda_function_arn_binding = args
            .lambda_function_arn
            .get_output(context)
            .get_inner();
        let maximum_execution_frequency_binding = args
            .maximum_execution_frequency
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
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
            type_: "aws:cfg/organizationCustomRule:OrganizationCustomRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "lambdaFunctionArn".into(),
                    value: &lambda_function_arn_binding,
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
        OrganizationCustomRuleResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            excluded_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludedAccounts"),
            ),
            input_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputParameters"),
            ),
            lambda_function_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lambdaFunctionArn"),
            ),
            maximum_execution_frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maximumExecutionFrequency"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_id_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceIdScope"),
            ),
            resource_types_scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTypesScopes"),
            ),
            tag_key_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagKeyScope"),
            ),
            tag_value_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagValueScope"),
            ),
            trigger_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerTypes"),
            ),
        }
    }
}
