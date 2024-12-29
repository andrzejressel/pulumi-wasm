/// Provides an IAM managed policy for a Single Sign-On (SSO) Permission Set resource
///
/// > **NOTE:** Creating this resource will automatically [Provision the Permission Set](https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html) to apply the corresponding updates to all assigned accounts.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let exampleManagedPolicyAttachment = managed_policy_attachment::create(
///         "exampleManagedPolicyAttachment",
///         ManagedPolicyAttachmentArgs::builder()
///             .instance_arn("${example.arns[0]}")
///             .managed_policy_arn("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup")
///             .permission_set_arn("${examplePermissionSet.arn}")
///             .build_struct(),
///     );
///     let examplePermissionSet = permission_set::create(
///         "examplePermissionSet",
///         PermissionSetArgs::builder()
///             .instance_arn("${example.arns[0]}")
///             .name("Example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Account Assignment
///
/// > Because destruction of a managed policy attachment resource also re-provisions the associated permission set to all accounts, explicitly indicating the dependency with the account assignment resource via the `depends_on` meta argument is necessary to ensure proper deletion order when these resources are used together.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let accountAssignment = account_assignment::create(
///         "accountAssignment",
///         AccountAssignmentArgs::builder()
///             .instance_arn("${example.arns[0]}")
///             .permission_set_arn("${examplePermissionSet.arn}")
///             .principal_id("${exampleGroup.groupId}")
///             .principal_type("GROUP")
///             .target_id("123456789012")
///             .target_type("AWS_ACCOUNT")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder()
///             .description("Admin Group")
///             .display_name("Admin")
///             .identity_store_id("${example.identityStoreIds[0]}")
///             .build_struct(),
///     );
///     let exampleManagedPolicyAttachment = managed_policy_attachment::create(
///         "exampleManagedPolicyAttachment",
///         ManagedPolicyAttachmentArgs::builder()
///             .instance_arn("${example.arns[0]}")
///             .managed_policy_arn("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup")
///             .permission_set_arn("${examplePermissionSet.arn}")
///             .build_struct(),
///     );
///     let examplePermissionSet = permission_set::create(
///         "examplePermissionSet",
///         PermissionSetArgs::builder()
///             .instance_arn("${example.arns[0]}")
///             .name("Example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Managed Policy Attachments using the `managed_policy_arn`, `permission_set_arn`, and `instance_arn` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/managedPolicyAttachment:ManagedPolicyAttachment example arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup,arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
/// ```
pub mod managed_policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPolicyAttachmentArgs {
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// The IAM managed policy Amazon Resource Name (ARN) to be attached to the Permission Set.
        #[builder(into)]
        pub managed_policy_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        #[builder(into)]
        pub permission_set_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedPolicyAttachmentResult {
        /// The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// The IAM managed policy Amazon Resource Name (ARN) to be attached to the Permission Set.
        pub managed_policy_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the IAM Managed Policy.
        pub managed_policy_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set.
        pub permission_set_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedPolicyAttachmentArgs,
    ) -> ManagedPolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_arn_binding = args.instance_arn.get_inner();
        let managed_policy_arn_binding = args.managed_policy_arn.get_inner();
        let permission_set_arn_binding = args.permission_set_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/managedPolicyAttachment:ManagedPolicyAttachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "managedPolicyArn".into(),
                    value: &managed_policy_arn_binding,
                },
                register_interface::ObjectField {
                    name: "permissionSetArn".into(),
                    value: &permission_set_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
                register_interface::ResultField {
                    name: "managedPolicyArn".into(),
                },
                register_interface::ResultField {
                    name: "managedPolicyName".into(),
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
        ManagedPolicyAttachmentResult {
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            managed_policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedPolicyArn").unwrap(),
            ),
            managed_policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedPolicyName").unwrap(),
            ),
            permission_set_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionSetArn").unwrap(),
            ),
        }
    }
}
