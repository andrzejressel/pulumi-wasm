/// Manages the Security Hub Organization Configuration.
///
/// > **NOTE:** This resource requires an `aws.securityhub.OrganizationAdminAccount` to be configured (not necessarily with Pulumi). More information about managing Security Hub in an organization can be found in the [Managing administrator and member accounts](https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-accounts.html) documentation.
///
/// > **NOTE:** In order to set the `configuration_type` to `CENTRAL`, the delegated admin must be a member account of the organization and not the management account. Central configuration also requires an `aws.securityhub.FindingAggregator` to be configured.
///
/// > **NOTE:** This is an advanced AWS resource. Pulumi will automatically assume management of the Security Hub Organization Configuration without import and perform no actions on removal from the Pulumi program.
///
/// > **NOTE:** Deleting this resource resets security hub to a local organization configuration with auto enable false.
///
/// ## Example Usage
///
/// ### Local Configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization::create(
///         "example",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(vec!["securityhub.amazonaws.com",])
///             .feature_set("ALL")
///             .build_struct(),
///     );
///     let exampleOrganizationAdminAccount = organization_admin_account::create(
///         "exampleOrganizationAdminAccount",
///         OrganizationAdminAccountArgs::builder()
///             .admin_account_id("123456789012")
///             .build_struct(),
///     );
///     let exampleOrganizationConfiguration = organization_configuration::create(
///         "exampleOrganizationConfiguration",
///         OrganizationConfigurationArgs::builder().auto_enable(true).build_struct(),
///     );
/// }
/// ```
///
/// ### Central Configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization_admin_account::create(
///         "example",
///         OrganizationAdminAccountArgs::builder()
///             .admin_account_id("123456789012")
///             .build_struct(),
///     );
///     let exampleFindingAggregator = finding_aggregator::create(
///         "exampleFindingAggregator",
///         FindingAggregatorArgs::builder().linking_mode("ALL_REGIONS").build_struct(),
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
/// ## Import
///
/// Using `pulumi import`, import an existing Security Hub enabled account using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/organizationConfiguration:OrganizationConfiguration example 123456789012
/// ```
pub mod organization_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationArgs {
        /// Whether to automatically enable Security Hub for new accounts in the organization.
        #[builder(into)]
        pub auto_enable: pulumi_wasm_rust::Output<bool>,
        /// Whether to automatically enable Security Hub default standards for new member accounts in the organization. By default, this parameter is equal to `DEFAULT`, and new member accounts are automatically enabled with default Security Hub standards. To opt out of enabling default standards for new member accounts, set this parameter equal to `NONE`.
        #[builder(into, default)]
        pub auto_enable_standards: pulumi_wasm_rust::Output<Option<String>>,
        /// Provides information about the way an organization is configured in Security Hub.
        #[builder(into, default)]
        pub organization_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::securityhub::OrganizationConfigurationOrganizationConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationResult {
        /// Whether to automatically enable Security Hub for new accounts in the organization.
        pub auto_enable: pulumi_wasm_rust::Output<bool>,
        /// Whether to automatically enable Security Hub default standards for new member accounts in the organization. By default, this parameter is equal to `DEFAULT`, and new member accounts are automatically enabled with default Security Hub standards. To opt out of enabling default standards for new member accounts, set this parameter equal to `NONE`.
        pub auto_enable_standards: pulumi_wasm_rust::Output<String>,
        /// Provides information about the way an organization is configured in Security Hub.
        pub organization_configuration: pulumi_wasm_rust::Output<
            super::super::types::securityhub::OrganizationConfigurationOrganizationConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationConfigurationArgs,
    ) -> OrganizationConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_enable_binding = args.auto_enable.get_inner();
        let auto_enable_standards_binding = args.auto_enable_standards.get_inner();
        let organization_configuration_binding = args
            .organization_configuration
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/organizationConfiguration:OrganizationConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoEnable".into(),
                    value: &auto_enable_binding,
                },
                register_interface::ObjectField {
                    name: "autoEnableStandards".into(),
                    value: &auto_enable_standards_binding,
                },
                register_interface::ObjectField {
                    name: "organizationConfiguration".into(),
                    value: &organization_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoEnable".into(),
                },
                register_interface::ResultField {
                    name: "autoEnableStandards".into(),
                },
                register_interface::ResultField {
                    name: "organizationConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationConfigurationResult {
            auto_enable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoEnable").unwrap(),
            ),
            auto_enable_standards: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoEnableStandards").unwrap(),
            ),
            organization_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationConfiguration").unwrap(),
            ),
        }
    }
}
