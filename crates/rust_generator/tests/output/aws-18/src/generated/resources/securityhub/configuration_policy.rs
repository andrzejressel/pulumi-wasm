/// Manages Security Hub configuration policy
///
/// > **NOTE:** This resource requires `aws.securityhub.OrganizationConfiguration` to be configured of type `CENTRAL`. More information about Security Hub central configuration and configuration policies can be found in the [How Security Hub configuration policies work](https://docs.aws.amazon.com/securityhub/latest/userguide/configuration-policies-overview.html) documentation.
///
/// ## Example Usage
///
/// ### Default standards enabled
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = finding_aggregator::create(
///         "example",
///         FindingAggregatorArgs::builder().linking_mode("ALL_REGIONS").build_struct(),
///     );
///     let exampleConfigurationPolicy = configuration_policy::create(
///         "exampleConfigurationPolicy",
///         ConfigurationPolicyArgs::builder()
///             .configuration_policy(
///                 ConfigurationPolicyConfigurationPolicy::builder()
///                     .enabledStandardArns(
///                         vec![
///                             "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
///                             "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
///                         ],
///                     )
///                     .securityControlsConfiguration(
///                         ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration::builder()
///                             .disabledControlIdentifiers(vec![])
///                             .build_struct(),
///                     )
///                     .serviceEnabled(true)
///                     .build_struct(),
///             )
///             .description("This is an example configuration policy")
///             .name("Example")
///             .build_struct(),
///     );
///     let exampleOrganizationConfiguration = organization_configuration::create(
///         "exampleOrganizationConfiguration",
///         OrganizationConfigurationArgs::builder()
///             .auto_enable(false)
///             .auto_enable_standards("NONE")
///             .organization_configuration(
///                 OrganizationConfigurationOrganizationConfiguration::builder()
///                     .configurationType("CENTRAL")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Disabled Policy
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let disabled = configuration_policy::create(
///         "disabled",
///         ConfigurationPolicyArgs::builder()
///             .configuration_policy(
///                 ConfigurationPolicyConfigurationPolicy::builder()
///                     .serviceEnabled(false)
///                     .build_struct(),
///             )
///             .description("This is an example of disabled configuration policy")
///             .name("Disabled")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Custom Control Configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let disabled = configuration_policy::create(
///         "disabled",
///         ConfigurationPolicyArgs::builder()
///             .configuration_policy(
///                 ConfigurationPolicyConfigurationPolicy::builder()
///                     .enabledStandardArns(
///                         vec![
///                             "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
///                             "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
///                         ],
///                     )
///                     .securityControlsConfiguration(
///                         ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration::builder()
///                             .enabledControlIdentifiers(vec!["APIGateway.1", "IAM.7",])
///                             .securityControlCustomParameters(
///                                 vec![
///                                     ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter::builder()
///                                     .parameters(vec![ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter::builder()
///                                     . enum
///                                     (ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterEnum::builder()
///                                     .value("INFO").build_struct()).name("loggingLevel")
///                                     .valueType("CUSTOM").build_struct(),])
///                                     .securityControlId("APIGateway.1").build_struct(),
///                                     ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter::builder()
///                                     .parameters(vec![ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter::builder()
///                                     .bool(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterBool::builder()
///                                     .value(false).build_struct())
///                                     .name("RequireLowercaseCharacters").valueType("CUSTOM")
///                                     .build_struct(),
///                                     ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter::builder()
///                                     .int(ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameterInt::builder()
///                                     .value(60).build_struct()).name("MaxPasswordAge")
///                                     .valueType("CUSTOM").build_struct(),])
///                                     .securityControlId("IAM.7").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .serviceEnabled(true)
///                     .build_struct(),
///             )
///             .description(
///                 "This is an example of configuration policy with custom control settings",
///             )
///             .name("Custom Controls")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an existing Security Hub enabled account using the universally unique identifier (UUID) of the policy. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/configurationPolicy:ConfigurationPolicy example "00000000-1111-2222-3333-444444444444"
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationPolicyArgs {
        /// Defines how Security Hub is configured. See below.
        #[builder(into)]
        pub configuration_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::securityhub::ConfigurationPolicyConfigurationPolicy,
        >,
        /// The description of the configuration policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the configuration policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationPolicyResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Defines how Security Hub is configured. See below.
        pub configuration_policy: pulumi_gestalt_rust::Output<
            super::super::types::securityhub::ConfigurationPolicyConfigurationPolicy,
        >,
        /// The description of the configuration policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the configuration policy.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationPolicyArgs,
    ) -> ConfigurationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_policy_binding = args.configuration_policy.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securityhub/configurationPolicy:ConfigurationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationPolicy".into(),
                    value: configuration_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationPolicyResult {
            arn: o.get_field("arn"),
            configuration_policy: o.get_field("configurationPolicy"),
            description: o.get_field("description"),
            name: o.get_field("name"),
        }
    }
}
