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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PermissionSetInlinePolicyArgs,
    ) -> PermissionSetInlinePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let inline_policy_binding_1 = args.inline_policy.get_output(context);
        let inline_policy_binding = inline_policy_binding_1.get_inner();
        let instance_arn_binding_1 = args.instance_arn.get_output(context);
        let instance_arn_binding = instance_arn_binding_1.get_inner();
        let permission_set_arn_binding_1 = args.permission_set_arn.get_output(context);
        let permission_set_arn_binding = permission_set_arn_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/permissionSetInlinePolicy:PermissionSetInlinePolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "inlinePolicy".into(),
                    value: &inline_policy_binding,
                },
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "permissionSetArn".into(),
                    value: &permission_set_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PermissionSetInlinePolicyResult {
            inline_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inlinePolicy"),
            ),
            instance_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceArn"),
            ),
            permission_set_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissionSetArn"),
            ),
        }
    }
}
