#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user_groups {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserGroupsArgs {
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserGroupsResult {
        /// List of groups. See `groups` below.
        pub groups: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cognito::GetUserGroupsGroup>,
        >,
        /// User pool identifier.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserGroupsArgs,
    ) -> GetUserGroupsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let user_pool_id_binding = args.user_pool_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cognito/getUserGroups:getUserGroups".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserGroupsResult {
            groups: o.get_field("groups"),
            id: o.get_field("id"),
            user_pool_id: o.get_field("userPoolId"),
        }
    }
}
