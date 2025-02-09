/// Resource for managing AWS Audit Manager Organization Admin Account Registration.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization_admin_account_registration::create(
///         "example",
///         OrganizationAdminAccountRegistrationArgs::builder()
///             .admin_account_id("123456789012")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Organization Admin Account Registration using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/organizationAdminAccountRegistration:OrganizationAdminAccountRegistration example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_admin_account_registration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountRegistrationArgs {
        /// Identifier for the organization administrator account.
        #[builder(into)]
        pub admin_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountRegistrationResult {
        /// Identifier for the organization administrator account.
        pub admin_account_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the organization.
        pub organization_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationAdminAccountRegistrationArgs,
    ) -> OrganizationAdminAccountRegistrationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_account_id_binding = args.admin_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/organizationAdminAccountRegistration:OrganizationAdminAccountRegistration"
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
        OrganizationAdminAccountRegistrationResult {
            admin_account_id: o.get_field("adminAccountId"),
            organization_id: o.get_field("organizationId"),
        }
    }
}
