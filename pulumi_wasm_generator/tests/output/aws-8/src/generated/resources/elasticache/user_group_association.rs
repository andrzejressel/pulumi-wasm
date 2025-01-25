/// Associate an existing ElastiCache user and an existing user group.
///
/// > Pulumi will detect changes in the `aws.elasticache.UserGroup` since `aws.elasticache.UserGroupAssociation` changes the user IDs associated with the user group. You can ignore these changes with the `lifecycle` `ignore_changes` meta argument as shown in the example.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod user_group_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupAssociationArgs {
        /// ID of the user group.
        #[builder(into)]
        pub user_group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the user to associated with the user group.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserGroupAssociationResult {
        /// ID of the user group.
        pub user_group_id: pulumi_wasm_rust::Output<String>,
        /// ID of the user to associated with the user group.
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UserGroupAssociationArgs,
    ) -> UserGroupAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let user_group_id_binding = args.user_group_id.get_output(context).get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/userGroupAssociation:UserGroupAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "userGroupId".into(),
                    value: &user_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "userGroupId".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserGroupAssociationResult {
            user_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userGroupId").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
        }
    }
}
