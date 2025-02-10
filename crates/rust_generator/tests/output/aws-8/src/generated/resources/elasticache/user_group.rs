/// Provides an ElastiCache user group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = user::create(
///         "test",
///         UserArgs::builder()
///             .access_string(
///                 "on ~app::* -@all +@read +@hash +@bitmap +@geo -setbit -bitfield -hset -hsetnx -hmset -hincrby -hincrbyfloat -hdel -bitop -geoadd -georadius -georadiusbymember",
///             )
///             .engine("REDIS")
///             .passwords(vec!["password123456789",])
///             .user_id("testUserId")
///             .user_name("default")
///             .build_struct(),
///     );
///     let testUserGroup = user_group::create(
///         "testUserGroup",
///         UserGroupArgs::builder()
///             .engine("REDIS")
///             .user_group_id("userGroupId")
///             .user_ids(vec!["${test.userId}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache user groups using the `user_group_id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/userGroup:UserGroup my_user_group userGoupId1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupArgs {
        /// The current supported value is `REDIS`.
        #[builder(into)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the user group.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of user IDs that belong to the user group.
        #[builder(into, default)]
        pub user_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct UserGroupResult {
        /// The ARN that identifies the user group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The current supported value is `REDIS`.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the user group.
        ///
        /// The following arguments are optional:
        pub user_group_id: pulumi_gestalt_rust::Output<String>,
        /// The list of user IDs that belong to the user group.
        pub user_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserGroupArgs,
    ) -> UserGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let engine_binding = args.engine.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_group_id_binding = args.user_group_id.get_output(context);
        let user_ids_binding = args.user_ids.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticache/userGroup:UserGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userGroupId".into(),
                    value: user_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userIds".into(),
                    value: user_ids_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserGroupResult {
            arn: o.get_field("arn"),
            engine: o.get_field("engine"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_group_id: o.get_field("userGroupId"),
            user_ids: o.get_field("userIds"),
        }
    }
}
