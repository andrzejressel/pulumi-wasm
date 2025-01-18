/// Provides an ElastiCache user group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod user_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupArgs {
        /// The current supported value is `REDIS`.
        #[builder(into)]
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the user group.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_group_id: pulumi_wasm_rust::Output<String>,
        /// The list of user IDs that belong to the user group.
        #[builder(into, default)]
        pub user_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct UserGroupResult {
        /// The ARN that identifies the user group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The current supported value is `REDIS`.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the user group.
        ///
        /// The following arguments are optional:
        pub user_group_id: pulumi_wasm_rust::Output<String>,
        /// The list of user IDs that belong to the user group.
        pub user_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserGroupArgs) -> UserGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_binding = args.engine.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_group_id_binding = args.user_group_id.get_inner();
        let user_ids_binding = args.user_ids.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/userGroup:UserGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userGroupId".into(),
                    value: &user_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "userIds".into(),
                    value: &user_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userGroupId".into(),
                },
                register_interface::ResultField {
                    name: "userIds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userGroupId").unwrap(),
            ),
            user_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userIds").unwrap(),
            ),
        }
    }
}
