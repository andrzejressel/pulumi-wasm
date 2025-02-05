/// Manages Security Hub configuration policy
///
/// > **NOTE:** This resource requires `aws.securityhub.OrganizationConfiguration` to be configured of type `CENTRAL`. More information about Security Hub central configuration and configuration policies can be found in the [How Security Hub configuration policies work](https://docs.aws.amazon.com/securityhub/latest/userguide/configuration-policies-overview.html) documentation.
///
/// ## Example Usage
///
/// ### Default standards enabled
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod configuration_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationPolicyArgs {
        /// Defines how Security Hub is configured. See below.
        #[builder(into)]
        pub configuration_policy: pulumi_wasm_rust::InputOrOutput<
            super::super::types::securityhub::ConfigurationPolicyConfigurationPolicy,
        >,
        /// The description of the configuration policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the configuration policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationPolicyResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Defines how Security Hub is configured. See below.
        pub configuration_policy: pulumi_wasm_rust::Output<
            super::super::types::securityhub::ConfigurationPolicyConfigurationPolicy,
        >,
        /// The description of the configuration policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the configuration policy.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConfigurationPolicyArgs,
    ) -> ConfigurationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_policy_binding = args
            .configuration_policy
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/configurationPolicy:ConfigurationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationPolicy".into(),
                    value: &configuration_policy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConfigurationPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            configuration_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationPolicy"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
