/// Provides a customer managed policy attachment for a Single Sign-On (SSO) Permission Set resource
///
/// > **NOTE:** Creating this resource will automatically [Provision the Permission Set](https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html) to apply the corresponding updates to all assigned accounts.
///
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
///   examplePolicy:
///     type: aws:iam:Policy
///     name: example
///     properties:
///       name: TestPolicy
///       description: My test policy
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - ec2:Describe*
///               Effect: Allow
///               Resource: '*'
///   exampleCustomerManagedPolicyAttachment:
///     type: aws:ssoadmin:CustomerManagedPolicyAttachment
///     name: example
///     properties:
///       instanceArn: ${examplePermissionSet.instanceArn}
///       permissionSetArn: ${examplePermissionSet.arn}
///       customerManagedPolicyReference:
///         name: ${examplePolicy.name}
///         path: /
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Managed Policy Attachments using the `name`, `path`, `permission_set_arn`, and `instance_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/customerManagedPolicyAttachment:CustomerManagedPolicyAttachment example TestPolicy,/,arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod customer_managed_policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomerManagedPolicyAttachmentArgs {
        /// Specifies the name and path of a customer managed policy. See below.
        #[builder(into)]
        pub customer_managed_policy_reference: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ssoadmin::CustomerManagedPolicyAttachmentCustomerManagedPolicyReference,
        >,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        #[builder(into)]
        pub permission_set_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomerManagedPolicyAttachmentResult {
        /// Specifies the name and path of a customer managed policy. See below.
        pub customer_managed_policy_reference: pulumi_gestalt_rust::Output<
            super::super::types::ssoadmin::CustomerManagedPolicyAttachmentCustomerManagedPolicyReference,
        >,
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
        args: CustomerManagedPolicyAttachmentArgs,
    ) -> CustomerManagedPolicyAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let customer_managed_policy_reference_binding = args
            .customer_managed_policy_reference
            .get_output(context);
        let instance_arn_binding = args.instance_arn.get_output(context);
        let permission_set_arn_binding = args.permission_set_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/customerManagedPolicyAttachment:CustomerManagedPolicyAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedPolicyReference".into(),
                    value: &customer_managed_policy_reference_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionSetArn".into(),
                    value: &permission_set_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomerManagedPolicyAttachmentResult {
            customer_managed_policy_reference: o
                .get_field("customerManagedPolicyReference"),
            instance_arn: o.get_field("instanceArn"),
            permission_set_arn: o.get_field("permissionSetArn"),
        }
    }
}
