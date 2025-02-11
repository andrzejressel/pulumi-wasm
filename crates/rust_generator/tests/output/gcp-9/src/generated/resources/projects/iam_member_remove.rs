/// Ensures that a member:role pairing does not exist in a project's IAM policy.
///
/// On create, this resource will modify the policy to remove the `member` from the
/// `role`. If the membership is ever re-added, the next refresh will clear this
/// resource from state, proposing re-adding it to correct the membership. Import is
/// not supported- this resource will acquire the current policy and modify it as
/// part of creating the resource.
///
/// This resource will conflict with `gcp.projects.IAMPolicy` and
/// `gcp.projects.IAMBinding` resources that share a role, as well as
/// `gcp.projects.IAMMember` resources that target the same membership. When
/// multiple resources conflict the final state is not guaranteed to include or omit
/// the membership. Subsequent `pulumi up` calls will always show a diff
/// until the configuration is corrected.
///
/// For more information see
/// [the official documentation](https://cloud.google.com/iam/docs/granting-changing-revoking-access)
/// and
/// [API reference](https://cloud.google.com/resource-manager/reference/rest/v1/projects/setIamPolicy).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: gcp:projects:IamMemberRemove
///     properties:
///       role: roles/editor
///       project: ${targetProjectGoogleProject.projectId}
///       member: serviceAccount:${targetProjectGoogleProject.number}-compute@developer.gserviceaccount.com
/// variables:
///   targetProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iam_member_remove {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IamMemberRemoveArgs {
        /// The IAM principal that should not have the target role.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project id of the target project.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The target role that should be removed.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IamMemberRemoveResult {
        /// The IAM principal that should not have the target role.
        /// Each entry can have one of the following values:
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The project id of the target project.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The target role that should be removed.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IamMemberRemoveArgs,
    ) -> IamMemberRemoveResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let member_binding = args.member.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:projects/iamMemberRemove:IamMemberRemove".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "member".into(),
                    value: &member_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IamMemberRemoveResult {
            member: o.get_field("member"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
