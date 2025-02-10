#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_groups {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupsArgs {
        /// Identity Store ID associated with the Single Sign-On (SSO) Instance.
        #[builder(into)]
        pub identity_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupsResult {
        /// List of Identity Store Groups
        pub groups: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::identitystore::GetGroupsGroup>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identity_store_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupsArgs,
    ) -> GetGroupsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_store_id_binding = args.identity_store_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:identitystore/getGroups:getGroups".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityStoreId".into(),
                    value: identity_store_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupsResult {
            groups: o.get_field("groups"),
            id: o.get_field("id"),
            identity_store_id: o.get_field("identityStoreId"),
        }
    }
}
