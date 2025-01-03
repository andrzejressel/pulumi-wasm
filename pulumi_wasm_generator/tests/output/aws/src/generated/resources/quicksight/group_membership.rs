/// Resource for managing QuickSight Group Membership
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = group_membership::create(
///         "example",
///         GroupMembershipArgs::builder()
///             .group_name("all-access-users")
///             .member_name("john_smith")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight Group membership using the AWS account ID, namespace, group name and member name separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/groupMembership:GroupMembership example 123456789123/default/all-access-users/john_smith
/// ```
pub mod group_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupMembershipArgs {
        /// The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the group in which the member will be added.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the member to add to the group.
        #[builder(into)]
        pub member_name: pulumi_wasm_rust::Output<String>,
        /// The namespace that you want the user to be a part of. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupMembershipResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID for the AWS account that the group is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the group in which the member will be added.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the member to add to the group.
        pub member_name: pulumi_wasm_rust::Output<String>,
        /// The namespace that you want the user to be a part of. Defaults to `default`.
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GroupMembershipArgs) -> GroupMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let group_name_binding = args.group_name.get_inner();
        let member_name_binding = args.member_name.get_inner();
        let namespace_binding = args.namespace.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/groupMembership:GroupMembership".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "memberName".into(),
                    value: &member_name_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "memberName".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
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
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            member_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memberName").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
        }
    }
}
