/// Associate an existing ElastiCache user and an existing user group.
///
/// > Pulumi will detect changes in the `aws.elasticache.UserGroup` since `aws.elasticache.UserGroupAssociation` changes the user IDs associated with the user group. You can ignore these changes with the `lifecycle` `ignore_changes` meta argument as shown in the example.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = user::create(
///         "default",
///         UserArgs::builder()
///             .access_string(
///                 "on ~app::* -@all +@read +@hash +@bitmap +@geo -setbit -bitfield -hset -hsetnx -hmset -hincrby -hincrbyfloat -hdel -bitop -geoadd -georadius -georadiusbymember",
///             )
///             .engine("REDIS")
///             .passwords(vec!["password123456789",])
///             .user_id("defaultUserID")
///             .user_name("default")
///             .build_struct(),
///     );
///     let example = user_group::create(
///         "example",
///         UserGroupArgs::builder()
///             .engine("REDIS")
///             .user_group_id("userGroupId")
///             .user_ids(vec!["${default.userId}",])
///             .build_struct(),
///     );
///     let exampleUser = user::create(
///         "exampleUser",
///         UserArgs::builder()
///             .access_string(
///                 "on ~app::* -@all +@read +@hash +@bitmap +@geo -setbit -bitfield -hset -hsetnx -hmset -hincrby -hincrbyfloat -hdel -bitop -geoadd -georadius -georadiusbymember",
///             )
///             .engine("REDIS")
///             .passwords(vec!["password123456789",])
///             .user_id("exampleUserID")
///             .user_name("exampleuser")
///             .build_struct(),
///     );
///     let exampleUserGroupAssociation = user_group_association::create(
///         "exampleUserGroupAssociation",
///         UserGroupAssociationArgs::builder()
///             .user_group_id("${example.userGroupId}")
///             .user_id("${exampleUser.userId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache user group associations using the `user_group_id` and `user_id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/userGroupAssociation:UserGroupAssociation example userGoupId1,userId
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupAssociationArgs {
        /// ID of the user group.
        #[builder(into)]
        pub user_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the user to associated with the user group.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserGroupAssociationResult {
        /// ID of the user group.
        pub user_group_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the user to associated with the user group.
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserGroupAssociationArgs,
    ) -> UserGroupAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let user_group_id_binding = args.user_group_id.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticache/userGroupAssociation:UserGroupAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userGroupId".into(),
                    value: user_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: user_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserGroupAssociationResult {
            user_group_id: o.get_field("userGroupId"),
            user_id: o.get_field("userId"),
        }
    }
}
