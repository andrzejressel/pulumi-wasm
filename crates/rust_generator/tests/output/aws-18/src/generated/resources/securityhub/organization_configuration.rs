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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationArgs {
        /// Whether to automatically enable Security Hub for new accounts in the organization.
        #[builder(into)]
        pub auto_enable: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Whether to automatically enable Security Hub default standards for new member accounts in the organization. By default, this parameter is equal to `DEFAULT`, and new member accounts are automatically enabled with default Security Hub standards. To opt out of enabling default standards for new member accounts, set this parameter equal to `NONE`.
        #[builder(into, default)]
        pub auto_enable_standards: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provides information about the way an organization is configured in Security Hub.
        #[builder(into, default)]
        pub organization_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::securityhub::OrganizationConfigurationOrganizationConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationResult {
        /// Whether to automatically enable Security Hub for new accounts in the organization.
        pub auto_enable: pulumi_gestalt_rust::Output<bool>,
        /// Whether to automatically enable Security Hub default standards for new member accounts in the organization. By default, this parameter is equal to `DEFAULT`, and new member accounts are automatically enabled with default Security Hub standards. To opt out of enabling default standards for new member accounts, set this parameter equal to `NONE`.
        pub auto_enable_standards: pulumi_gestalt_rust::Output<String>,
        /// Provides information about the way an organization is configured in Security Hub.
        pub organization_configuration: pulumi_gestalt_rust::Output<
            super::super::types::securityhub::OrganizationConfigurationOrganizationConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConfigurationArgs,
    ) -> OrganizationConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_enable_binding = args.auto_enable.get_output(context);
        let auto_enable_standards_binding = args
            .auto_enable_standards
            .get_output(context);
        let organization_configuration_binding = args
            .organization_configuration
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securityhub/organizationConfiguration:OrganizationConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoEnable".into(),
                    value: auto_enable_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoEnableStandards".into(),
                    value: auto_enable_standards_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationConfiguration".into(),
                    value: organization_configuration_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationConfigurationResult {
            auto_enable: o.get_field("autoEnable"),
            auto_enable_standards: o.get_field("autoEnableStandards"),
            organization_configuration: o.get_field("organizationConfiguration"),
        }
    }
}
