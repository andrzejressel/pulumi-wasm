#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific User by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the User.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Returns information on a specific User by User id
        #[builder(into, default)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// The Amazon Resource Name (ARN) of the User.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the user account in the directory used for identity management.
        pub directory_user_id: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the hierarchy group for the user.
        pub hierarchy_group_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A block that contains information about the identity of the user. Documented below.
        pub identity_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetUserIdentityInfo>,
        >,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A block that contains information about the phone settings for the user. Documented below.
        pub phone_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetUserPhoneConfig>,
        >,
        /// The identifier of the routing profile for the user.
        pub routing_profile_id: pulumi_gestalt_rust::Output<String>,
        /// A list of identifiers for the security profiles for the user.
        pub security_profile_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the User.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserArgs,
    ) -> GetUserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: &user_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserResult {
            arn: o.get_field("arn"),
            directory_user_id: o.get_field("directoryUserId"),
            hierarchy_group_id: o.get_field("hierarchyGroupId"),
            id: o.get_field("id"),
            identity_infos: o.get_field("identityInfos"),
            instance_id: o.get_field("instanceId"),
            name: o.get_field("name"),
            phone_configs: o.get_field("phoneConfigs"),
            routing_profile_id: o.get_field("routingProfileId"),
            security_profile_ids: o.get_field("securityProfileIds"),
            tags: o.get_field("tags"),
            user_id: o.get_field("userId"),
        }
    }
}
