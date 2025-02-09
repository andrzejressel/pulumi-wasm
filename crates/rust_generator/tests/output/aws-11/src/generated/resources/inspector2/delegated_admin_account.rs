/// Resource for managing an Amazon Inspector Delegated Admin Account.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:inspector2:DelegatedAdminAccount
///     properties:
///       accountId: ${current.accountId}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Inspector Delegated Admin Account using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:inspector2/delegatedAdminAccount:DelegatedAdminAccount example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod delegated_admin_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DelegatedAdminAccountArgs {
        /// Account to enable as delegated admin account.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DelegatedAdminAccountResult {
        /// Account to enable as delegated admin account.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Status of this delegated admin account.
        pub relationship_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DelegatedAdminAccountArgs,
    ) -> DelegatedAdminAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector2/delegatedAdminAccount:DelegatedAdminAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DelegatedAdminAccountResult {
            account_id: o.get_field("accountId"),
            relationship_status: o.get_field("relationshipStatus"),
        }
    }
}
