/// ## Example Usage
///
/// ```yaml
/// resources:
///   examplePermissionSet:
///     type: aws:ssoadmin:PermissionSet
///     name: example
///     properties:
///       name: Example
///       instanceArn: ${example.arns[0]}
///   examplePermissionSetInlinePolicy:
///     type: aws:ssoadmin:PermissionSetInlinePolicy
///     name: example
///     properties:
///       inlinePolicy: ${exampleGetPolicyDocument.json}
///       instanceArn: ${example.arns[0]}
///       permissionSetArn: ${examplePermissionSet.arn}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
///   exampleGetPolicyDocument:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: '1'
///             actions:
///               - s3:ListAllMyBuckets
///               - s3:GetBucketLocation
///             resources:
///               - arn:aws:s3:::*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Permission Set Inline Policies using the `permission_set_arn` and `instance_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/permissionSetInlinePolicy:PermissionSetInlinePolicy example arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod permission_set_inline_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionSetInlinePolicyArgs {
        /// The IAM inline policy to attach to a Permission Set.
        #[builder(into)]
        pub inline_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        #[builder(into)]
        pub permission_set_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PermissionSetInlinePolicyResult {
        /// The IAM inline policy to attach to a Permission Set.
        pub inline_policy: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        pub permission_set_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PermissionSetInlinePolicyArgs,
    ) -> PermissionSetInlinePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let inline_policy_binding = args.inline_policy.get_output(context);
        let instance_arn_binding = args.instance_arn.get_output(context);
        let permission_set_arn_binding = args.permission_set_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/permissionSetInlinePolicy:PermissionSetInlinePolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inlinePolicy".into(),
                    value: inline_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: instance_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionSetArn".into(),
                    value: permission_set_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PermissionSetInlinePolicyResult {
            inline_policy: o.get_field("inlinePolicy"),
            instance_arn: o.get_field("instanceArn"),
            permission_set_arn: o.get_field("permissionSetArn"),
        }
    }
}
