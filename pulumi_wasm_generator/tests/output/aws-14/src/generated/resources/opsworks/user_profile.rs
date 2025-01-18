/// Provides an OpsWorks User Profile resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProfile = user_profile::create(
///         "myProfile",
///         UserProfileArgs::builder()
///             .ssh_username("my_user")
///             .user_arn("${user.arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod user_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserProfileArgs {
        /// Whether users can specify their own SSH public key through the My Settings page
        #[builder(into, default)]
        pub allow_self_management: pulumi_wasm_rust::Output<Option<bool>>,
        /// The users public key
        #[builder(into, default)]
        pub ssh_public_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The ssh username, with witch this user wants to log in
        #[builder(into)]
        pub ssh_username: pulumi_wasm_rust::Output<String>,
        /// The user's IAM ARN
        #[builder(into)]
        pub user_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserProfileResult {
        /// Whether users can specify their own SSH public key through the My Settings page
        pub allow_self_management: pulumi_wasm_rust::Output<Option<bool>>,
        /// The users public key
        pub ssh_public_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The ssh username, with witch this user wants to log in
        pub ssh_username: pulumi_wasm_rust::Output<String>,
        /// The user's IAM ARN
        pub user_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserProfileArgs) -> UserProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_self_management_binding = args.allow_self_management.get_inner();
        let ssh_public_key_binding = args.ssh_public_key.get_inner();
        let ssh_username_binding = args.ssh_username.get_inner();
        let user_arn_binding = args.user_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowSelfManagement".into(),
                    value: &allow_self_management_binding,
                },
                register_interface::ObjectField {
                    name: "sshPublicKey".into(),
                    value: &ssh_public_key_binding,
                },
                register_interface::ObjectField {
                    name: "sshUsername".into(),
                    value: &ssh_username_binding,
                },
                register_interface::ObjectField {
                    name: "userArn".into(),
                    value: &user_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowSelfManagement".into(),
                },
                register_interface::ResultField {
                    name: "sshPublicKey".into(),
                },
                register_interface::ResultField {
                    name: "sshUsername".into(),
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
        UserProfileResult {
            allow_self_management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowSelfManagement").unwrap(),
            ),
            ssh_public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshPublicKey").unwrap(),
            ),
            ssh_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshUsername").unwrap(),
            ),
            user_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userArn").unwrap(),
            ),
        }
    }
}
