/// Resource for managing an AWS IdentityStore Group Membership.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder()
///             .description("Some group name")
///             .display_name("MyGroup")
///             .identity_store_id("${example.identityStoreIds[0]}")
///             .build_struct(),
///     );
///     let exampleGroupMembership = group_membership::create(
///         "exampleGroupMembership",
///         GroupMembershipArgs::builder()
///             .group_id("${exampleGroup.groupId}")
///             .identity_store_id("${example.identityStoreIds[0]}")
///             .member_id("${exampleUser.userId}")
///             .build_struct(),
///     );
///     let exampleUser = user::create(
///         "exampleUser",
///         UserArgs::builder()
///             .display_name("John Doe")
///             .identity_store_id("${example.identityStoreIds[0]}")
///             .name(UserName::builder().familyName("Doe").givenName("John").build_struct())
///             .user_name("john.doe@example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_identitystore_group_membership` using the `identity_store_id/membership_id`. For example:
///
/// ```sh
/// $ pulumi import aws:identitystore/groupMembership:GroupMembership example d-0000000000/00000000-0000-0000-0000-000000000000
/// ```
pub mod group_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupMembershipArgs {
        /// The identifier for a group in the Identity Store.
        #[builder(into)]
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// Identity Store ID associated with the Single Sign-On Instance.
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
        /// The identifier for a user in the Identity Store.
        #[builder(into)]
        pub member_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GroupMembershipResult {
        /// The identifier for a group in the Identity Store.
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// Identity Store ID associated with the Single Sign-On Instance.
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
        /// The identifier for a user in the Identity Store.
        pub member_id: pulumi_wasm_rust::Output<String>,
        /// The identifier of the newly created group membership in the Identity Store.
        pub membership_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GroupMembershipArgs) -> GroupMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_id_binding = args.group_id.get_inner();
        let identity_store_id_binding = args.identity_store_id.get_inner();
        let member_id_binding = args.member_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:identitystore/groupMembership:GroupMembership".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupId".into(),
                    value: &group_id_binding,
                },
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "memberId".into(),
                    value: &member_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupId".into(),
                },
                register_interface::ResultField {
                    name: "identityStoreId".into(),
                },
                register_interface::ResultField {
                    name: "memberId".into(),
                },
                register_interface::ResultField {
                    name: "membershipId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupMembershipResult {
            group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupId").unwrap(),
            ),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityStoreId").unwrap(),
            ),
            member_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memberId").unwrap(),
            ),
            membership_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("membershipId").unwrap(),
            ),
        }
    }
}
