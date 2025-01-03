/// Adds the specified user to the specified group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_pool::create(
///         "example",
///         UserPoolArgs::builder()
///             .name("example")
///             .password_policy(
///                 UserPoolPasswordPolicy::builder()
///                     .minimumLength(6)
///                     .requireNumbers(false)
///                     .requireSymbols(false)
///                     .requireUppercase(false)
///                     .temporaryPasswordValidityDays(7)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleUser = user::create(
///         "exampleUser",
///         UserArgs::builder()
///             .user_pool_id("${example.id}")
///             .username("example")
///             .build_struct(),
///     );
///     let exampleUserGroup = user_group::create(
///         "exampleUserGroup",
///         UserGroupArgs::builder()
///             .name("example")
///             .user_pool_id("${example.id}")
///             .build_struct(),
///     );
///     let exampleUserInGroup = user_in_group::create(
///         "exampleUserInGroup",
///         UserInGroupArgs::builder()
///             .group_name("${exampleUserGroup.name}")
///             .user_pool_id("${example.id}")
///             .username("${exampleUser.username}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod user_in_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserInGroupArgs {
        /// The name of the group to which the user is to be added.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The user pool ID of the user and group.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// The username of the user to be added to the group.
        #[builder(into)]
        pub username: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserInGroupResult {
        /// The name of the group to which the user is to be added.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The user pool ID of the user and group.
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// The username of the user to be added to the group.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserInGroupArgs) -> UserInGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_name_binding = args.group_name.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let username_binding = args.username.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/userInGroup:UserInGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserInGroupResult {
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
        }
    }
}
