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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_group_membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupMembershipArgs {
        /// A list of IAM Groups to add the user to
        #[builder(into)]
        pub groups: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name of the IAM User to add to groups
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserGroupMembershipResult {
        /// A list of IAM Groups to add the user to
        pub groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the IAM User to add to groups
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserGroupMembershipArgs,
    ) -> UserGroupMembershipResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let groups_binding = args.groups.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/userGroupMembership:UserGroupMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groups".into(),
                    value: groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: user_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserGroupMembershipResult {
            groups: o.get_field("groups"),
            user: o.get_field("user"),
        }
    }
}
