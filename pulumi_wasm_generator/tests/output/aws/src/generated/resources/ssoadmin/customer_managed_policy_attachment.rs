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
///       Function: aws:ssoadmin:getInstances
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Managed Policy Attachments using the `name`, `path`, `permission_set_arn`, and `instance_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/customerManagedPolicyAttachment:CustomerManagedPolicyAttachment example TestPolicy,/,arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
pub mod customer_managed_policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomerManagedPolicyAttachmentArgs {
        /// Specifies the name and path of a customer managed policy. See below.
        #[builder(into)]
        pub customer_managed_policy_reference: pulumi_wasm_rust::Output<
            super::super::types::ssoadmin::CustomerManagedPolicyAttachmentCustomerManagedPolicyReference,
        >,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        #[builder(into)]
        pub permission_set_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CustomerManagedPolicyAttachmentResult {
        /// Specifies the name and path of a customer managed policy. See below.
        pub customer_managed_policy_reference: pulumi_wasm_rust::Output<
            super::super::types::ssoadmin::CustomerManagedPolicyAttachmentCustomerManagedPolicyReference,
        >,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        pub permission_set_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CustomerManagedPolicyAttachmentArgs,
    ) -> CustomerManagedPolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let customer_managed_policy_reference_binding = args
            .customer_managed_policy_reference
            .get_inner();
        let instance_arn_binding = args.instance_arn.get_inner();
        let permission_set_arn_binding = args.permission_set_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/customerManagedPolicyAttachment:CustomerManagedPolicyAttachment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customerManagedPolicyReference".into(),
                    value: &customer_managed_policy_reference_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "customerManagedPolicyReference".into(),
                },
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
                register_interface::ResultField {
                    name: "permissionSetArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomerManagedPolicyAttachmentResult {
            customer_managed_policy_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedPolicyReference").unwrap(),
            ),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            permission_set_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionSetArn").unwrap(),
            ),
        }
    }
}