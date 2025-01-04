/// Provides a resource for adding an IAM User to IAM Groups. This
/// resource can be used multiple times with the same user for non-overlapping
/// groups.
///
/// To exclusively manage the users in a group, see the
/// `aws.iam.GroupMembership` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example1 = user_group_membership::create(
///         "example1",
///         UserGroupMembershipArgs::builder()
///             .groups(vec!["${group1.name}", "${group2.name}",])
///             .user("${user1.name}")
///             .build_struct(),
///     );
///     let example2 = user_group_membership::create(
///         "example2",
///         UserGroupMembershipArgs::builder()
///             .groups(vec!["${group3.name}",])
///             .user("${user1.name}")
///             .build_struct(),
///     );
///     let group1 = group::create(
///         "group1",
///         GroupArgs::builder().name("group1").build_struct(),
///     );
///     let group2 = group::create(
///         "group2",
///         GroupArgs::builder().name("group2").build_struct(),
///     );
///     let group3 = group::create(
///         "group3",
///         GroupArgs::builder().name("group3").build_struct(),
///     );
///     let user1 = user::create("user1", UserArgs::builder().name("user1").build_struct());
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM user group membership using the user name and group names separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/userGroupMembership:UserGroupMembership example1 user1/group1/group2
/// ```
pub mod user_group_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupMembershipArgs {
        /// A list of IAM Groups to add the user to
        #[builder(into)]
        pub groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the IAM User to add to groups
        #[builder(into)]
        pub user: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserGroupMembershipResult {
        /// A list of IAM Groups to add the user to
        pub groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the IAM User to add to groups
        pub user: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: UserGroupMembershipArgs,
    ) -> UserGroupMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let groups_binding = args.groups.get_inner();
        let user_binding = args.user.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/userGroupMembership:UserGroupMembership".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groups".into(),
                    value: &groups_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groups".into(),
                },
                register_interface::ResultField {
                    name: "user".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserGroupMembershipResult {
            groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groups").unwrap(),
            ),
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
        }
    }
}
