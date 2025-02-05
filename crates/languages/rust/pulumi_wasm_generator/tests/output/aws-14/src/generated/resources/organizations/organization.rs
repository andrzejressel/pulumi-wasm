/// Provides a resource to create an organization.
///
/// !> **WARNING:** When migrating from a `feature_set` of `CONSOLIDATED_BILLING` to `ALL`, the Organization account owner will received an email stating the following: "You started the process to enable all features for your AWS organization. As part of that process, all member accounts that joined your organization by invitation must approve the change. You don’t need approval from member accounts that you directly created from within your AWS organization." After all member accounts have accepted the invitation, the Organization account owner must then finalize the changes via the [AWS Console](https://console.aws.amazon.com/organizations/home#/organization/settings/migration-progress). Until these steps are performed, the provider will perpetually show a difference, and the `DescribeOrganization` API will continue to show the `FeatureSet` as `CONSOLIDATED_BILLING`. See the [AWS Organizations documentation](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html) for more information.
///
/// !> **WARNING:** [Warning from the AWS Docs](https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html): "We recommend that you enable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the service is aware that it can create the resources that are required for the integration. How the service creates those resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service."
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let org = organization::create(
///         "org",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(
///                 vec!["cloudtrail.amazonaws.com", "config.amazonaws.com",],
///             )
///             .feature_set("ALL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the AWS organization using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:organizations/organization:Organization my_org o-1234567
/// ```
pub mod organization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationArgs {
        /// List of AWS service principal names for which you want to enable integration with your organization. This is typically in the form of a URL, such as service-abbreviation.amazonaws.com. Organization must have `feature_set` set to `ALL`. Some services do not support enablement via this endpoint, see [warning in aws docs](https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html).
        #[builder(into, default)]
        pub aws_service_access_principals: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// List of Organizations policy types to enable in the Organization Root. Organization must have `feature_set` set to `ALL`. For additional information about valid policy types (e.g., `AISERVICES_OPT_OUT_POLICY`, `BACKUP_POLICY`, `RESOURCE_CONTROL_POLICY`, `SERVICE_CONTROL_POLICY`, and `TAG_POLICY`), see the [AWS Organizations API Reference](https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnablePolicyType.html).
        #[builder(into, default)]
        pub enabled_policy_types: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specify "ALL" (default) or "CONSOLIDATED_BILLING".
        #[builder(into, default)]
        pub feature_set: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationResult {
        /// List of organization accounts including the master account. For a list excluding the master account, see the `non_master_accounts` attribute. All elements have these attributes:
        pub accounts: pulumi_wasm_rust::Output<
            Vec<super::super::types::organizations::OrganizationAccount>,
        >,
        /// ARN of the root
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of AWS service principal names for which you want to enable integration with your organization. This is typically in the form of a URL, such as service-abbreviation.amazonaws.com. Organization must have `feature_set` set to `ALL`. Some services do not support enablement via this endpoint, see [warning in aws docs](https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html).
        pub aws_service_access_principals: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of Organizations policy types to enable in the Organization Root. Organization must have `feature_set` set to `ALL`. For additional information about valid policy types (e.g., `AISERVICES_OPT_OUT_POLICY`, `BACKUP_POLICY`, `RESOURCE_CONTROL_POLICY`, `SERVICE_CONTROL_POLICY`, and `TAG_POLICY`), see the [AWS Organizations API Reference](https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnablePolicyType.html).
        pub enabled_policy_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specify "ALL" (default) or "CONSOLIDATED_BILLING".
        pub feature_set: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the master account
        pub master_account_arn: pulumi_wasm_rust::Output<String>,
        /// Email address of the master account
        pub master_account_email: pulumi_wasm_rust::Output<String>,
        /// Identifier of the master account
        pub master_account_id: pulumi_wasm_rust::Output<String>,
        /// Name of the master account
        pub master_account_name: pulumi_wasm_rust::Output<String>,
        /// List of organization accounts excluding the master account. For a list including the master account, see the `accounts` attribute. All elements have these attributes:
        pub non_master_accounts: pulumi_wasm_rust::Output<
            Vec<super::super::types::organizations::OrganizationNonMasterAccount>,
        >,
        /// List of organization roots. All elements have these attributes:
        pub roots: pulumi_wasm_rust::Output<
            Vec<super::super::types::organizations::OrganizationRoot>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OrganizationArgs,
    ) -> OrganizationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_service_access_principals_binding = args
            .aws_service_access_principals
            .get_output(context)
            .get_inner();
        let enabled_policy_types_binding = args
            .enabled_policy_types
            .get_output(context)
            .get_inner();
        let feature_set_binding = args.feature_set.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:organizations/organization:Organization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsServiceAccessPrincipals".into(),
                    value: &aws_service_access_principals_binding,
                },
                register_interface::ObjectField {
                    name: "enabledPolicyTypes".into(),
                    value: &enabled_policy_types_binding,
                },
                register_interface::ObjectField {
                    name: "featureSet".into(),
                    value: &feature_set_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OrganizationResult {
            accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accounts"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            aws_service_access_principals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsServiceAccessPrincipals"),
            ),
            enabled_policy_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabledPolicyTypes"),
            ),
            feature_set: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("featureSet"),
            ),
            master_account_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterAccountArn"),
            ),
            master_account_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterAccountEmail"),
            ),
            master_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterAccountId"),
            ),
            master_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterAccountName"),
            ),
            non_master_accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nonMasterAccounts"),
            ),
            roots: pulumi_wasm_rust::__private::into_domain(o.extract_field("roots")),
        }
    }
}
