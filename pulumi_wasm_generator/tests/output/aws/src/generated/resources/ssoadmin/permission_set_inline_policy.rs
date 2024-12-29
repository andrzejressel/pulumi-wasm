/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let exampleGetPolicyDocument = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["s3:ListAllMyBuckets", "s3:GetBucketLocation",])
///                     .resources(vec!["arn:aws:s3:::*",]).sid("1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let examplePermissionSet = permission_set::create(
///         "examplePermissionSet",
///         PermissionSetArgs::builder()
///             .instance_arn("${example.arns[0]}")
///             .name("Example")
///             .build_struct(),
///     );
///     let examplePermissionSetInlinePolicy = permission_set_inline_policy::create(
///         "examplePermissionSetInlinePolicy",
///         PermissionSetInlinePolicyArgs::builder()
///             .inline_policy("${exampleGetPolicyDocument.json}")
///             .instance_arn("${example.arns[0]}")
///             .permission_set_arn("${examplePermissionSet.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Permission Set Inline Policies using the `permission_set_arn` and `instance_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/permissionSetInlinePolicy:PermissionSetInlinePolicy example arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
pub mod permission_set_inline_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionSetInlinePolicyArgs {
        /// The IAM inline policy to attach to a Permission Set.
        #[builder(into)]
        pub inline_policy: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        #[builder(into)]
        pub permission_set_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PermissionSetInlinePolicyResult {
        /// The IAM inline policy to attach to a Permission Set.
        pub inline_policy: pulumi_wasm_rust::Output<String>,
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
        args: PermissionSetInlinePolicyArgs,
    ) -> PermissionSetInlinePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let inline_policy_binding = args.inline_policy.get_inner();
        let instance_arn_binding = args.instance_arn.get_inner();
        let permission_set_arn_binding = args.permission_set_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/permissionSetInlinePolicy:PermissionSetInlinePolicy"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "inlinePolicy".into(),
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
        PermissionSetInlinePolicyResult {
            inline_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inlinePolicy").unwrap(),
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
