/// Provides an OpsWorks permission resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myStackPermission = permission::create(
///         "myStackPermission",
///         PermissionArgs::builder()
///             .allow_ssh(true)
///             .allow_sudo(true)
///             .level("iam_only")
///             .stack_id("${stack.id}")
///             .user_arn("${user.arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod permission {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionArgs {
        /// Whether the user is allowed to use SSH to communicate with the instance
        #[builder(into, default)]
        pub allow_ssh: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the user is allowed to use sudo to elevate privileges
        #[builder(into, default)]
        pub allow_sudo: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The users permission level. Mus be one of `deny`, `show`, `deploy`, `manage`, `iam_only`
        #[builder(into, default)]
        pub level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The stack to set the permissions for
        #[builder(into)]
        pub stack_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user's IAM ARN to set permissions for
        #[builder(into)]
        pub user_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PermissionResult {
        /// Whether the user is allowed to use SSH to communicate with the instance
        pub allow_ssh: pulumi_gestalt_rust::Output<bool>,
        /// Whether the user is allowed to use sudo to elevate privileges
        pub allow_sudo: pulumi_gestalt_rust::Output<bool>,
        /// The users permission level. Mus be one of `deny`, `show`, `deploy`, `manage`, `iam_only`
        pub level: pulumi_gestalt_rust::Output<String>,
        /// The stack to set the permissions for
        pub stack_id: pulumi_gestalt_rust::Output<String>,
        /// The user's IAM ARN to set permissions for
        pub user_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PermissionArgs,
    ) -> PermissionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_ssh_binding = args.allow_ssh.get_output(context);
        let allow_sudo_binding = args.allow_sudo.get_output(context);
        let level_binding = args.level.get_output(context);
        let stack_id_binding = args.stack_id.get_output(context);
        let user_arn_binding = args.user_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opsworks/permission:Permission".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowSsh".into(),
                    value: allow_ssh_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowSudo".into(),
                    value: allow_sudo_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "level".into(),
                    value: level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackId".into(),
                    value: stack_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userArn".into(),
                    value: user_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PermissionResult {
            allow_ssh: o.get_field("allowSsh"),
            allow_sudo: o.get_field("allowSudo"),
            level: o.get_field("level"),
            stack_id: o.get_field("stackId"),
            user_arn: o.get_field("userArn"),
        }
    }
}
