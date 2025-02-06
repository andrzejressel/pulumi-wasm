/// Provides a Cognito User Group resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:cognito:UserPool
///     properties:
///       name: identity pool
///   groupRoleRole:
///     type: aws:iam:Role
///     name: group_role
///     properties:
///       name: user-group-role
///       assumeRolePolicy: ${groupRole.json}
///   mainUserGroup:
///     type: aws:cognito:UserGroup
///     name: main
///     properties:
///       name: user-group
///       userPoolId: ${main.id}
///       description: Managed by Pulumi
///       precedence: 42
///       roleArn: ${groupRoleRole.arn}
/// variables:
///   groupRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Federated
///                 identifiers:
///                   - cognito-identity.amazonaws.com
///             actions:
///               - sts:AssumeRoleWithWebIdentity
///             conditions:
///               - test: StringEquals
///                 variable: cognito-identity.amazonaws.com:aud
///                 values:
///                   - us-east-1:12345678-dead-beef-cafe-123456790ab
///               - test: ForAnyValue:StringLike
///                 variable: cognito-identity.amazonaws.com:amr
///                 values:
///                   - authenticated
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserGroupArgs {
        /// The description of the user group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the user group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The precedence of the user group.
        #[builder(into, default)]
        pub precedence: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ARN of the IAM role to be associated with the user group.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UserGroupArgs,
    ) -> UserGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let precedence_binding = args.precedence.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/userGroup:UserGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserGroupResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            precedence: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("precedence"),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
        }
    }
}
