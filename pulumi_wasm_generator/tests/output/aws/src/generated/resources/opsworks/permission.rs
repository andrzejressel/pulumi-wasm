/// Provides an OpsWorks permission resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myStackPermission = permission::create(
///         "myStackPermission",
///         PermissionArgs::builder()
///             .allow_ssh(true)
///             .allow_sudo(true)
///             .level("iam_only")
///             .stack_id("${stack.id}")
///             .user_arn("${user.arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod permission {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionArgs {
        /// Whether the user is allowed to use SSH to communicate with the instance
        #[builder(into, default)]
        pub allow_ssh: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the user is allowed to use sudo to elevate privileges
        #[builder(into, default)]
        pub allow_sudo: pulumi_wasm_rust::Output<Option<bool>>,
        /// The users permission level. Mus be one of `deny`, `show`, `deploy`, `manage`, `iam_only`
        #[builder(into, default)]
        pub level: pulumi_wasm_rust::Output<Option<String>>,
        /// The stack to set the permissions for
        #[builder(into)]
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// The user's IAM ARN to set permissions for
        #[builder(into)]
        pub user_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PermissionResult {
        /// Whether the user is allowed to use SSH to communicate with the instance
        pub allow_ssh: pulumi_wasm_rust::Output<bool>,
        /// Whether the user is allowed to use sudo to elevate privileges
        pub allow_sudo: pulumi_wasm_rust::Output<bool>,
        /// The users permission level. Mus be one of `deny`, `show`, `deploy`, `manage`, `iam_only`
        pub level: pulumi_wasm_rust::Output<String>,
        /// The stack to set the permissions for
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// The user's IAM ARN to set permissions for
        pub user_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PermissionArgs) -> PermissionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_ssh_binding = args.allow_ssh.get_inner();
        let allow_sudo_binding = args.allow_sudo.get_inner();
        let level_binding = args.level.get_inner();
        let stack_id_binding = args.stack_id.get_inner();
        let user_arn_binding = args.user_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/permission:Permission".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowSsh".into(),
                    value: &allow_ssh_binding,
                },
                register_interface::ObjectField {
                    name: "allowSudo".into(),
                    value: &allow_sudo_binding,
                },
                register_interface::ObjectField {
                    name: "level".into(),
                    value: &level_binding,
                },
                register_interface::ObjectField {
                    name: "stackId".into(),
                    value: &stack_id_binding,
                },
                register_interface::ObjectField {
                    name: "userArn".into(),
                    value: &user_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowSsh".into(),
                },
                register_interface::ResultField {
                    name: "allowSudo".into(),
                },
                register_interface::ResultField {
                    name: "level".into(),
                },
                register_interface::ResultField {
                    name: "stackId".into(),
                },
                register_interface::ResultField {
                    name: "userArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PermissionResult {
            allow_ssh: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowSsh").unwrap(),
            ),
            allow_sudo: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowSudo").unwrap(),
            ),
            level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("level").unwrap(),
            ),
            stack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackId").unwrap(),
            ),
            user_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userArn").unwrap(),
            ),
        }
    }
}
