/// Manages a GuardDuty Organization Admin Account. The AWS account utilizing this resource must be an Organizations primary account. More information about Organizations support in GuardDuty can be found in the [GuardDuty User Guide](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_organizations.html).
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
///             .aws_service_access_principals(vec!["guardduty.amazonaws.com",])
///             .feature_set("ALL")
///             .build_struct(),
///     );
///     let exampleDetector = detector::create(
///         "exampleDetector",
///         DetectorArgs::builder().build_struct(),
///     );
///     let exampleOrganizationAdminAccount = organization_admin_account::create(
///         "exampleOrganizationAdminAccount",
///         OrganizationAdminAccountArgs::builder()
///             .admin_account_id("123456789012")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty Organization Admin Account using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/organizationAdminAccount:OrganizationAdminAccount example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_admin_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountArgs {
        /// AWS account identifier to designate as a delegated administrator for GuardDuty.
        #[builder(into)]
        pub admin_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountResult {
        /// AWS account identifier to designate as a delegated administrator for GuardDuty.
        pub admin_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationAdminAccountArgs,
    ) -> OrganizationAdminAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_account_id_binding = args.admin_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:guardduty/organizationAdminAccount:OrganizationAdminAccount"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminAccountId".into(),
                    value: admin_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationAdminAccountResult {
            admin_account_id: o.get_field("adminAccountId"),
        }
    }
}
