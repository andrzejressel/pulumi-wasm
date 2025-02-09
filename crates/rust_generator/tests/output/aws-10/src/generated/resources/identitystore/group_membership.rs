/// Resource for managing an AWS IdentityStore Group Membership.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleUser:
///     type: aws:identitystore:User
///     name: example
///     properties:
///       identityStoreId: ${example.identityStoreIds[0]}
///       displayName: John Doe
///       userName: john.doe@example.com
///       name:
///         familyName: Doe
///         givenName: John
///   exampleGroup:
///     type: aws:identitystore:Group
///     name: example
///     properties:
///       identityStoreId: ${example.identityStoreIds[0]}
///       displayName: MyGroup
///       description: Some group name
///   exampleGroupMembership:
///     type: aws:identitystore:GroupMembership
///     name: example
///     properties:
///       identityStoreId: ${example.identityStoreIds[0]}
///       groupId: ${exampleGroup.groupId}
///       memberId: ${exampleUser.userId}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_identitystore_group_membership` using the `identity_store_id/membership_id`. For example:
///
/// ```sh
/// $ pulumi import aws:identitystore/groupMembership:GroupMembership example d-0000000000/00000000-0000-0000-0000-000000000000
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupMembershipArgs {
        /// The identifier for a group in the Identity Store.
        #[builder(into)]
        pub group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identity Store ID associated with the Single Sign-On Instance.
        #[builder(into)]
        pub identity_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identifier for a user in the Identity Store.
        #[builder(into)]
        pub member_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupMembershipResult {
        /// The identifier for a group in the Identity Store.
        pub group_id: pulumi_gestalt_rust::Output<String>,
        /// Identity Store ID associated with the Single Sign-On Instance.
        pub identity_store_id: pulumi_gestalt_rust::Output<String>,
        /// The identifier for a user in the Identity Store.
        pub member_id: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the newly created group membership in the Identity Store.
        pub membership_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupMembershipArgs,
    ) -> GroupMembershipResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_id_binding = args.group_id.get_output(context);
        let identity_store_id_binding = args.identity_store_id.get_output(context);
        let member_id_binding = args.member_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:identitystore/groupMembership:GroupMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupId".into(),
                    value: group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityStoreId".into(),
                    value: identity_store_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memberId".into(),
                    value: member_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupMembershipResult {
            group_id: o.get_field("groupId"),
            identity_store_id: o.get_field("identityStoreId"),
            member_id: o.get_field("memberId"),
            membership_id: o.get_field("membershipId"),
        }
    }
}
