#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_role {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoleArgs {
        /// Friendly IAM role name to match.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags attached to the role.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRoleResult {
        /// ARN of the role.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Policy document associated with the role.
        pub assume_role_policy: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the role in RFC 3339 format.
        pub create_date: pulumi_gestalt_rust::Output<String>,
        /// Description for the role.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Maximum session duration.
        pub max_session_duration: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Path to the role.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the policy that is used to set the permissions boundary for the role.
        pub permissions_boundary: pulumi_gestalt_rust::Output<String>,
        /// Contains information about the last time that an IAM role was used. See `role_last_used` for details.
        pub role_last_useds: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::iam::GetRoleRoleLastUsed>,
        >,
        /// Tags attached to the role.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Stable and unique string identifying the role.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRoleArgs,
    ) -> GetRoleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getRole:getRole".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRoleResult {
            arn: o.get_field("arn"),
            assume_role_policy: o.get_field("assumeRolePolicy"),
            create_date: o.get_field("createDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            max_session_duration: o.get_field("maxSessionDuration"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            permissions_boundary: o.get_field("permissionsBoundary"),
            role_last_useds: o.get_field("roleLastUseds"),
            tags: o.get_field("tags"),
            unique_id: o.get_field("uniqueId"),
        }
    }
}
