#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_organizational_unit_child_accounts {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitChildAccountsArgs {
        /// The parent ID of the accounts.
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitChildAccountsResult {
        /// List of child accounts, which have the following attributes:
        pub accounts: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::organizations::GetOrganizationalUnitChildAccountsAccount,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parent_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOrganizationalUnitChildAccountsArgs,
    ) -> GetOrganizationalUnitChildAccountsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_id_binding = args.parent_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:organizations/getOrganizationalUnitChildAccounts:getOrganizationalUnitChildAccounts"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrganizationalUnitChildAccountsResult {
            accounts: o.get_field("accounts"),
            id: o.get_field("id"),
            parent_id: o.get_field("parentId"),
        }
    }
}
