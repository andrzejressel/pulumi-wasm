/// Provides a Single Sign-On (SSO) Account Assignment resource
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleAccountAssignment:
///     type: aws:ssoadmin:AccountAssignment
///     name: example
///     properties:
///       instanceArn: ${example.arns[0]}
///       permissionSetArn: ${exampleGetPermissionSet.arn}
///       principalId: ${exampleGetGroup.groupId}
///       principalType: GROUP
///       targetId: '123456789012'
///       targetType: AWS_ACCOUNT
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
///   exampleGetPermissionSet:
///     fn::invoke:
///       function: aws:ssoadmin:getPermissionSet
///       arguments:
///         instanceArn: ${example.arns[0]}
///         name: AWSReadOnlyAccess
///   exampleGetGroup:
///     fn::invoke:
///       function: aws:identitystore:getGroup
///       arguments:
///         identityStoreId: ${example.identityStoreIds[0]}
///         alternateIdentifier:
///           uniqueAttribute:
///             attributePath: DisplayName
///             attributeValue: ExampleGroup
/// ```
///
/// ### With Managed Policy Attachment
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
/// Using `pulumi import`, import SSO Account Assignments using the `principal_id`, `principal_type`, `target_id`, `target_type`, `permission_set_arn`, `instance_arn` separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/accountAssignment:AccountAssignment example f81d4fae-7dec-11d0-a765-00a0c91e6bf6,GROUP,1234567890,AWS_ACCOUNT,arn:aws:sso:::permissionSet/ssoins-0123456789abcdef/ps-0123456789abcdef,arn:aws:sso:::instance/ssoins-0123456789abcdef
/// ```
pub mod account_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountAssignmentArgs {
        /// The Amazon Resource Name (ARN) of the SSO Instance.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set that the admin wants to grant the principal access to.
        #[builder(into)]
        pub permission_set_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// An identifier for an object in SSO, such as a user or group. PrincipalIds are GUIDs (For example, `f81d4fae-7dec-11d0-a765-00a0c91e6bf6`).
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The entity type for which the assignment will be created. Valid values: `USER`, `GROUP`.
        #[builder(into)]
        pub principal_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// An AWS account identifier, typically a 10-12 digit string.
        #[builder(into)]
        pub target_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The entity type for which the assignment will be created. Valid values: `AWS_ACCOUNT`.
        #[builder(into, default)]
        pub target_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountAssignmentResult {
        /// The Amazon Resource Name (ARN) of the SSO Instance.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Permission Set that the admin wants to grant the principal access to.
        pub permission_set_arn: pulumi_wasm_rust::Output<String>,
        /// An identifier for an object in SSO, such as a user or group. PrincipalIds are GUIDs (For example, `f81d4fae-7dec-11d0-a765-00a0c91e6bf6`).
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// The entity type for which the assignment will be created. Valid values: `USER`, `GROUP`.
        pub principal_type: pulumi_wasm_rust::Output<String>,
        /// An AWS account identifier, typically a 10-12 digit string.
        pub target_id: pulumi_wasm_rust::Output<String>,
        /// The entity type for which the assignment will be created. Valid values: `AWS_ACCOUNT`.
        pub target_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountAssignmentArgs,
    ) -> AccountAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_arn_binding = args.instance_arn.get_output(context).get_inner();
        let permission_set_arn_binding = args
            .permission_set_arn
            .get_output(context)
            .get_inner();
        let principal_id_binding = args.principal_id.get_output(context).get_inner();
        let principal_type_binding = args.principal_type.get_output(context).get_inner();
        let target_id_binding = args.target_id.get_output(context).get_inner();
        let target_type_binding = args.target_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/accountAssignment:AccountAssignment".into(),
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
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
                register_interface::ObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetType".into(),
                    value: &target_type_binding,
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
                    name: "principalId".into(),
                },
                register_interface::ResultField {
                    name: "principalType".into(),
                },
                register_interface::ResultField {
                    name: "targetId".into(),
                },
                register_interface::ResultField {
                    name: "targetType".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountAssignmentResult {
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            permission_set_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionSetArn").unwrap(),
            ),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalId").unwrap(),
            ),
            principal_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalType").unwrap(),
            ),
            target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetId").unwrap(),
            ),
            target_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetType").unwrap(),
            ),
        }
    }
}
