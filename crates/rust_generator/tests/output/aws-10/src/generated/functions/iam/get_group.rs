#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// Friendly IAM group name to match.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        /// User ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Stable and unique string identifying the group.
        pub group_id: pulumi_gestalt_rust::Output<String>,
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Path to the IAM user.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// List of objects containing group member information. See below.
        pub users: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::iam::GetGroupUser>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupArgs,
    ) -> GetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_name_binding = args.group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getGroup:getGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupResult {
            arn: o.get_field("arn"),
            group_id: o.get_field("groupId"),
            group_name: o.get_field("groupName"),
            id: o.get_field("id"),
            path: o.get_field("path"),
            users: o.get_field("users"),
        }
    }
}
