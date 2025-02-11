/// A Membership defines a relationship between a Group and an entity belonging to that Group, referred to as a "member".
///
///
/// To get more information about GroupMembership, see:
///
/// * [API documentation](https://cloud.google.com/identity/docs/reference/rest/v1/groups.memberships)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/identity/docs/how-to/memberships-google-groups)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the Cloud Identity API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Cloud Identity Group Membership
///
///
/// ```yaml
/// resources:
///   group:
///     type: gcp:cloudidentity:Group
///     properties:
///       displayName: my-identity-group
///       parent: customers/A01b123xz
///       groupKey:
///         id: my-identity-group@example.com
///       labels:
///         cloudidentity.googleapis.com/groups.discussion_forum: ""
///   child-group:
///     type: gcp:cloudidentity:Group
///     properties:
///       displayName: my-identity-group-child
///       parent: customers/A01b123xz
///       groupKey:
///         id: my-identity-group-child@example.com
///       labels:
///         cloudidentity.googleapis.com/groups.discussion_forum: ""
///   cloudIdentityGroupMembershipBasic:
///     type: gcp:cloudidentity:GroupMembership
///     name: cloud_identity_group_membership_basic
///     properties:
///       group: ${group.id}
///       preferredMemberKey:
///         id: ${["child-group"].groupKey.id}
///       roles:
///         - name: MEMBER
/// ```
/// ### Cloud Identity Group Membership User
///
///
/// ```yaml
/// resources:
///   group:
///     type: gcp:cloudidentity:Group
///     properties:
///       displayName: my-identity-group
///       parent: customers/A01b123xz
///       groupKey:
///         id: my-identity-group@example.com
///       labels:
///         cloudidentity.googleapis.com/groups.discussion_forum: ""
///   cloudIdentityGroupMembershipBasic:
///     type: gcp:cloudidentity:GroupMembership
///     name: cloud_identity_group_membership_basic
///     properties:
///       group: ${group.id}
///       preferredMemberKey:
///         id: cloud_identity_user@example.com
///       roles:
///         - name: MEMBER
///         - name: MANAGER
/// ```
///
/// ## Import
///
/// GroupMembership can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, GroupMembership can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudidentity/groupMembership:GroupMembership default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupMembershipArgs {
        /// The name of the Group to create this membership in.
        #[builder(into)]
        pub group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// EntityKey of the member.
        #[builder(into, default)]
        pub member_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudidentity::GroupMembershipMemberKey>,
        >,
        /// EntityKey of the member.
        #[builder(into, default)]
        pub preferred_member_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudidentity::GroupMembershipPreferredMemberKey>,
        >,
        /// The MembershipRoles that apply to the Membership.
        /// Must not contain duplicate MembershipRoles with the same name.
        /// Structure is documented below.
        #[builder(into)]
        pub roles: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::cloudidentity::GroupMembershipRole>,
        >,
    }
    #[allow(dead_code)]
    pub struct GroupMembershipResult {
        /// The time when the Membership was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Group to create this membership in.
        pub group: pulumi_gestalt_rust::Output<String>,
        /// EntityKey of the member.
        pub member_key: pulumi_gestalt_rust::Output<
            super::super::types::cloudidentity::GroupMembershipMemberKey,
        >,
        /// The resource name of the Membership, of the form groups/{group_id}/memberships/{membership_id}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// EntityKey of the member.
        pub preferred_member_key: pulumi_gestalt_rust::Output<
            super::super::types::cloudidentity::GroupMembershipPreferredMemberKey,
        >,
        /// The MembershipRoles that apply to the Membership.
        /// Must not contain duplicate MembershipRoles with the same name.
        /// Structure is documented below.
        pub roles: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudidentity::GroupMembershipRole>,
        >,
        /// The type of the membership.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The time when the Membership was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
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
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_binding = args.group.get_output(context);
        let member_key_binding = args.member_key.get_output(context);
        let preferred_member_key_binding = args.preferred_member_key.get_output(context);
        let roles_binding = args.roles.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudidentity/groupMembership:GroupMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: &group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memberKey".into(),
                    value: &member_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMemberKey".into(),
                    value: &preferred_member_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roles".into(),
                    value: &roles_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupMembershipResult {
            create_time: o.get_field("createTime"),
            group: o.get_field("group"),
            member_key: o.get_field("memberKey"),
            name: o.get_field("name"),
            preferred_member_key: o.get_field("preferredMemberKey"),
            roles: o.get_field("roles"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
        }
    }
}
