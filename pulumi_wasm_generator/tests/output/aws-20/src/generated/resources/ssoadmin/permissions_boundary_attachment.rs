/// Attaches a permissions boundary policy to a Single Sign-On (SSO) Permission Set resource.
///
/// > **NOTE:** A permission set can have at most one permissions boundary attached; using more than one `aws.ssoadmin.PermissionsBoundaryAttachment` references the same permission set will show a permanent difference.
///
/// ## Example Usage
///
/// ### Attaching a customer-managed policy
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
///   examplePermissionsBoundaryAttachment:
///     type: aws:ssoadmin:PermissionsBoundaryAttachment
///     name: example
///     properties:
///       instanceArn: ${examplePermissionSet.instanceArn}
///       permissionSetArn: ${examplePermissionSet.arn}
///       permissionsBoundary:
///         customerManagedPolicyReference:
///           name: ${examplePolicy.name}
///           path: /
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ### Attaching an AWS-managed policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = permissions_boundary_attachment::create(
///         "example",
///         PermissionsBoundaryAttachmentArgs::builder()
///             .instance_arn("${exampleAwsSsoadminPermissionSet.instanceArn}")
///             .permission_set_arn("${exampleAwsSsoadminPermissionSet.arn}")
///             .permissions_boundary(
///                 PermissionsBoundaryAttachmentPermissionsBoundary::builder()
///                     .managedPolicyArn("arn:aws:iam::aws:policy/ReadOnlyAccess")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Permissions Boundary Attachments using the `permission_set_arn` and `instance_arn`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/permissionsBoundaryAttachment:PermissionsBoundaryAttachment example arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
pub mod permissions_boundary_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionsBoundaryAttachmentArgs {
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        #[builder(into)]
        pub permission_set_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The permissions boundary policy. See below.
        #[builder(into)]
        pub permissions_boundary: pulumi_wasm_rust::InputOrOutput<
            super::super::types::ssoadmin::PermissionsBoundaryAttachmentPermissionsBoundary,
        >,
    }
    #[allow(dead_code)]
    pub struct PermissionsBoundaryAttachmentResult {
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        pub permission_set_arn: pulumi_wasm_rust::Output<String>,
        /// The permissions boundary policy. See below.
        pub permissions_boundary: pulumi_wasm_rust::Output<
            super::super::types::ssoadmin::PermissionsBoundaryAttachmentPermissionsBoundary,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PermissionsBoundaryAttachmentArgs,
    ) -> PermissionsBoundaryAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_arn_binding = args.instance_arn.get_output(context).get_inner();
        let permission_set_arn_binding = args
            .permission_set_arn
            .get_output(context)
            .get_inner();
        let permissions_boundary_binding = args
            .permissions_boundary
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/permissionsBoundaryAttachment:PermissionsBoundaryAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "permissionSetArn".into(),
                    value: &permission_set_arn_binding,
                },
                register_interface::ObjectField {
                    name: "permissionsBoundary".into(),
                    value: &permissions_boundary_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
                register_interface::ResultField {
                    name: "permissionSetArn".into(),
                },
                register_interface::ResultField {
                    name: "permissionsBoundary".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PermissionsBoundaryAttachmentResult {
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            permission_set_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionSetArn").unwrap(),
            ),
            permissions_boundary: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionsBoundary").unwrap(),
            ),
        }
    }
}
