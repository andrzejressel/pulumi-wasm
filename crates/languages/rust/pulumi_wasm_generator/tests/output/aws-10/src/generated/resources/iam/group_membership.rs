/// > **WARNING:** Multiple aws.iam.GroupMembership resources with the same group name will produce inconsistent behavior!
///
/// Provides a top level resource to manage IAM Group membership for IAM Users. For
/// more information on managing IAM Groups or IAM Users, see IAM Groups or
/// IAM Users
///
/// > **Note:** `aws.iam.GroupMembership` will conflict with itself if used more than once with the same group. To non-exclusively manage the users in a group, see the
/// `aws.iam.UserGroupMembership` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let group = group::create(
///         "group",
///         GroupArgs::builder().name("test-group").build_struct(),
///     );
///     let team = group_membership::create(
///         "team",
///         GroupMembershipArgs::builder()
///             .group("${group.name}")
///             .name("tf-testing-group-membership")
///             .users(vec!["${userOne.name}", "${userTwo.name}",])
///             .build_struct(),
///     );
///     let userOne = user::create(
///         "userOne",
///         UserArgs::builder().name("test-user").build_struct(),
///     );
///     let userTwo = user::create(
///         "userTwo",
///         UserArgs::builder().name("test-user-two").build_struct(),
///     );
/// }
/// ```
pub mod group_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupMembershipArgs {
        /// The IAM Group name to attach the list of `users` to
        #[builder(into)]
        pub group: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name to identify the Group Membership
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of IAM User names to associate with the Group
        #[builder(into)]
        pub users: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupMembershipResult {
        /// The IAM Group name to attach the list of `users` to
        pub group: pulumi_wasm_rust::Output<String>,
        /// The name to identify the Group Membership
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of IAM User names to associate with the Group
        pub users: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GroupMembershipArgs,
    ) -> GroupMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_binding = args.group.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let users_binding = args.users.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/groupMembership:GroupMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "users".into(),
                    value: &users_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupMembershipResult {
            group: pulumi_wasm_rust::__private::into_domain(o.extract_field("group")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            users: pulumi_wasm_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}
