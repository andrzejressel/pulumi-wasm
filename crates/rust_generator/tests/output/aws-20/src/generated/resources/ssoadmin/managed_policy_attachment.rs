/// Provides an IAM managed policy for a Single Sign-On (SSO) Permission Set resource
///
/// > **NOTE:** Creating this resource will automatically [Provision the Permission Set](https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html) to apply the corresponding updates to all assigned accounts.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   examplePermissionSet:
///     type: aws:ssoadmin:PermissionSet
///     name: example
///     properties:
///       name: Example
///       instanceArn: ${example.arns[0]}
///   exampleManagedPolicyAttachment:
///     type: aws:ssoadmin:ManagedPolicyAttachment
///     name: example
///     properties:
///       instanceArn: ${example.arns[0]}
///       managedPolicyArn: arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup
///       permissionSetArn: ${examplePermissionSet.arn}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ### With Account Assignment
///
/// > Because destruction of a managed policy attachment resource also re-provisions the associated permission set to all accounts, explicitly indicating the dependency with the account assignment resource via the `depends_on` meta argument is necessary to ensure proper deletion order when these resources are used together.
///
/// ```yaml
/// resources:
///   examplePermissionSet:
///     type: aws:ssoadmin:PermissionSet
///     name: example
///     properties:
///       name: Example
///       instanceArn: ${example.arns[0]}
///   exampleGroup:
///     type: aws:identitystore:Group
///     name: example
///     properties:
///       identityStoreId: ${example.identityStoreIds[0]}
///       displayName: Admin
///       description: Admin Group
///   accountAssignment:
///     type: aws:ssoadmin:AccountAssignment
///     name: account_assignment
///     properties:
///       instanceArn: ${example.arns[0]}
///       permissionSetArn: ${examplePermissionSet.arn}
///       principalId: ${exampleGroup.groupId}
///       principalType: GROUP
///       targetId: '123456789012'
///       targetType: AWS_ACCOUNT
///   exampleManagedPolicyAttachment:
///     type: aws:ssoadmin:ManagedPolicyAttachment
///     name: example
///     properties:
///       instanceArn: ${example.arns[0]}
///       managedPolicyArn: arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup
///       permissionSetArn: ${examplePermissionSet.arn}
///     options:
///       dependsOn:
///         - ${exampleAwsSsoadminAccountAssignment}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Managed Policy Attachments using the `managed_policy_arn`, `permission_set_arn`, and `instance_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/managedPolicyAttachment:ManagedPolicyAttachment example arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup,arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPolicyAttachmentArgs {
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IAM managed policy Amazon Resource Name (ARN) to be attached to the Permission Set.
        #[builder(into)]
        pub managed_policy_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        #[builder(into)]
        pub permission_set_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedPolicyAttachmentResult {
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        /// The IAM managed policy Amazon Resource Name (ARN) to be attached to the Permission Set.
        pub managed_policy_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the IAM Managed Policy.
        pub managed_policy_name: pulumi_gestalt_rust::Output<String>,
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
        args: ManagedPolicyAttachmentArgs,
    ) -> ManagedPolicyAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_arn_binding = args.instance_arn.get_output(context);
        let managed_policy_arn_binding = args.managed_policy_arn.get_output(context);
        let permission_set_arn_binding = args.permission_set_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/managedPolicyAttachment:ManagedPolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedPolicyArn".into(),
                    value: &managed_policy_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionSetArn".into(),
                    value: &permission_set_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedPolicyAttachmentResult {
            instance_arn: o.get_field("instanceArn"),
            managed_policy_arn: o.get_field("managedPolicyArn"),
            managed_policy_name: o.get_field("managedPolicyName"),
            permission_set_arn: o.get_field("permissionSetArn"),
        }
    }
}
