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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRoleArgs,
    ) -> GetRoleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getRole:getRole".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRoleResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            assume_role_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assumeRolePolicy"),
            ),
            create_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createDate"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            max_session_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxSessionDuration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            permissions_boundary: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissionsBoundary"),
            ),
            role_last_useds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleLastUseds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            unique_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uniqueId"),
            ),
        }
    }
}
