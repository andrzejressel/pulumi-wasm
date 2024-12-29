/// Provides a Cognito User Group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let groupRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRoleWithWebIdentity",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals")
///                     .values(vec!["us-east-1:12345678-dead-beef-cafe-123456790ab",])
///                     .variable("cognito-identity.amazonaws.com:aud").build_struct(),
///                     GetPolicyDocumentStatementCondition::builder()
///                     .test("ForAnyValue:StringLike").values(vec!["authenticated",])
///                     .variable("cognito-identity.amazonaws.com:amr").build_struct(),])
///                     .effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["cognito-identity.amazonaws.com",]). type
///                     ("Federated").build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let groupRoleRole = role::create(
///         "groupRoleRole",
///         RoleArgs::builder()
///             .assume_role_policy("${groupRole.json}")
///             .name("user-group-role")
///             .build_struct(),
///     );
///     let main = user_pool::create(
///         "main",
///         UserPoolArgs::builder().name("identity pool").build_struct(),
///     );
///     let mainUserGroup = user_group::create(
///         "mainUserGroup",
///         UserGroupArgs::builder()
///             .description("Managed by Pulumi")
///             .name("user-group")
///             .precedence(42)
///             .role_arn("${groupRoleRole.arn}")
///             .user_pool_id("${main.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito User Groups using the `user_pool_id`/`name` attributes concatenated. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/userGroup:UserGroup group us-east-1_vG78M4goG/user-group
/// ```
pub mod user_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupArgs {
        /// The description of the user group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the user group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The precedence of the user group.
        #[builder(into, default)]
        pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ARN of the IAM role to be associated with the user group.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserGroupResult {
        /// The description of the user group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the user group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The precedence of the user group.
        pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ARN of the IAM role to be associated with the user group.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The user pool ID.
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserGroupArgs) -> UserGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let precedence_binding = args.precedence.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/userGroup:UserGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "precedence".into(),
                    value: &precedence_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "precedence".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserGroupResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            precedence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("precedence").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
        }
    }
}
