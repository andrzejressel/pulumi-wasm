/// Manages Security Hub configuration policy associations.
///
/// > **NOTE:** This resource requires `aws.securityhub.OrganizationConfiguration` to be configured with type `CENTRAL`. More information about Security Hub central configuration and configuration policies can be found in the [How Security Hub configuration policies work](https://docs.aws.amazon.com/securityhub/latest/userguide/configuration-policies-overview.html) documentation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let accountExample = configuration_policy_association::create(
///         "accountExample",
///         ConfigurationPolicyAssociationArgs::builder()
///             .policy_id("${exampleConfigurationPolicy.id}")
///             .target_id("123456789012")
///             .build_struct(),
///     );
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
///     let ouExample = configuration_policy_association::create(
///         "ouExample",
///         ConfigurationPolicyAssociationArgs::builder()
///             .policy_id("${exampleConfigurationPolicy.id}")
///             .target_id("ou-abcd-12345678")
///             .build_struct(),
///     );
///     let rootExample = configuration_policy_association::create(
///         "rootExample",
///         ConfigurationPolicyAssociationArgs::builder()
///             .policy_id("${exampleConfigurationPolicy.id}")
///             .target_id("r-abcd")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an existing Security Hub enabled account using the target id. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/configurationPolicyAssociation:ConfigurationPolicyAssociation example_account_association 123456789012
/// ```
pub mod configuration_policy_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationPolicyAssociationArgs {
        /// The universally unique identifier (UUID) of the configuration policy.
        #[builder(into)]
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// The identifier of the target account, organizational unit, or the root to associate with the specified configuration.
        #[builder(into)]
        pub target_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationPolicyAssociationResult {
        /// The universally unique identifier (UUID) of the configuration policy.
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// The identifier of the target account, organizational unit, or the root to associate with the specified configuration.
        pub target_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConfigurationPolicyAssociationArgs,
    ) -> ConfigurationPolicyAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_id_binding = args.policy_id.get_inner();
        let target_id_binding = args.target_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/configurationPolicyAssociation:ConfigurationPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policyId".into(),
                },
                register_interface::ResultField {
                    name: "targetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationPolicyAssociationResult {
            policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyId").unwrap(),
            ),
            target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetId").unwrap(),
            ),
        }
    }
}
