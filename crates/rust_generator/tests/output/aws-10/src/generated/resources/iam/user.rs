/// Provides an IAM user.
///
/// > *NOTE:* If policies are attached to the user via the `aws.iam.PolicyAttachment` resource and you are modifying the user `name` or `path`, the `force_destroy` argument must be set to `true` and applied before attempting the operation otherwise you will encounter a `DeleteConflict` error. The `aws.iam.UserPolicyAttachment` resource (recommended) does not have this requirement.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   lb:
///     type: aws:iam:User
///     properties:
///       name: loadbalancer
///       path: /system/
///       tags:
///         tag-key: tag-value
///   lbAccessKey:
///     type: aws:iam:AccessKey
///     name: lb
///     properties:
///       user: ${lb.name}
///   lbRoUserPolicy:
///     type: aws:iam:UserPolicy
///     name: lb_ro
///     properties:
///       name: test
///       user: ${lb.name}
///       policy: ${lbRo.json}
/// variables:
///   lbRo:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ec2:Describe*
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Users using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/user:User lb loadbalancer
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// When destroying this user, destroy even if it
        /// has non-provider-managed IAM access keys, login profile or MFA devices. Without `force_destroy`
        /// a user with non-provider-managed access keys and login profile will fail to be destroyed.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The user's name. The name must consist of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: `=,.@-_.`. User names are not distinguished by case. For example, you cannot create users named both "TESTUSER" and "testuser".
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path in which to create the user.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the policy that is used to set the permissions boundary for the user.
        #[builder(into, default)]
        pub permissions_boundary: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of tags for the IAM user. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// The ARN assigned by AWS for this user.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// When destroying this user, destroy even if it
        /// has non-provider-managed IAM access keys, login profile or MFA devices. Without `force_destroy`
        /// a user with non-provider-managed access keys and login profile will fail to be destroyed.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The user's name. The name must consist of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: `=,.@-_.`. User names are not distinguished by case. For example, you cannot create users named both "TESTUSER" and "testuser".
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Path in which to create the user.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the policy that is used to set the permissions boundary for the user.
        pub permissions_boundary: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value mapping of tags for the IAM user. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The [unique ID][1] assigned by AWS.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserArgs,
    ) -> UserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let force_destroy_binding = args.force_destroy.get_output(context);
        let name_binding = args.name.get_output(context);
        let path_binding = args.path.get_output(context);
        let permissions_boundary_binding = args.permissions_boundary.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionsBoundary".into(),
                    value: permissions_boundary_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserResult {
            arn: o.get_field("arn"),
            force_destroy: o.get_field("forceDestroy"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            permissions_boundary: o.get_field("permissionsBoundary"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            unique_id: o.get_field("uniqueId"),
        }
    }
}
