/// Resource for managing AWS Audit Manager Account Registration.
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
///     let example = account_registration::create(
///         "example",
///         AccountRegistrationArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Deregister On Destroy
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account_registration::create(
///         "example",
///         AccountRegistrationArgs::builder().deregister_on_destroy(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Account Registration resources using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/accountRegistration:AccountRegistration example us-east-1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_registration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountRegistrationArgs {
        /// Identifier for the delegated administrator account.
        #[builder(into, default)]
        pub delegated_admin_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Flag to deregister AuditManager in the account upon destruction. Defaults to `false` (ie. AuditManager will remain active in the account, even if this resource is removed).
        #[builder(into, default)]
        pub deregister_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// KMS key identifier.
        #[builder(into, default)]
        pub kms_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountRegistrationResult {
        /// Identifier for the delegated administrator account.
        pub delegated_admin_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// Flag to deregister AuditManager in the account upon destruction. Defaults to `false` (ie. AuditManager will remain active in the account, even if this resource is removed).
        pub deregister_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// KMS key identifier.
        pub kms_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Status of the account registration request.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountRegistrationArgs,
    ) -> AccountRegistrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delegated_admin_account_binding = args
            .delegated_admin_account
            .get_output(context);
        let deregister_on_destroy_binding = args
            .deregister_on_destroy
            .get_output(context);
        let kms_key_binding = args.kms_key.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/accountRegistration:AccountRegistration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedAdminAccount".into(),
                    value: &delegated_admin_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deregisterOnDestroy".into(),
                    value: &deregister_on_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountRegistrationResult {
            delegated_admin_account: o.get_field("delegatedAdminAccount"),
            deregister_on_destroy: o.get_field("deregisterOnDestroy"),
            kms_key: o.get_field("kmsKey"),
            status: o.get_field("status"),
        }
    }
}
