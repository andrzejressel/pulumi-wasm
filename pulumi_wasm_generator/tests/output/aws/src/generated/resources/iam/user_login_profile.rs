/// Manages an IAM User Login Profile with limited support for password creation during this provider resource creation. Uses PGP to encrypt the password for safe transport to the user. PGP keys can be obtained from Keybase.
///
/// > To reset an IAM User login password via this provider, you can use delete and recreate this resource or change any of the arguments.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder().force_destroy(true).name("example").path("/").build_struct(),
///     );
///     let exampleUserLoginProfile = user_login_profile::create(
///         "exampleUserLoginProfile",
///         UserLoginProfileArgs::builder()
///             .pgp_key("keybase:some_person_that_exists")
///             .user("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM User Login Profiles without password information via the IAM User name. For example:
///
/// ```sh
/// $ pulumi import aws:iam/userLoginProfile:UserLoginProfile example myusername
/// ```
/// Since Pulumi has no method to read the PGP or password information during import, use the resource options `ignore_changes` argument to ignore them (unless you want to recreate a password). For example:
///
pub mod user_login_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserLoginProfileArgs {
        /// The length of the generated password on resource creation. Only applies on resource creation. Drift detection is not possible with this argument. Default value is `20`.
        #[builder(into, default)]
        pub password_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether the user should be forced to reset the generated password on resource creation. Only applies on resource creation.
        #[builder(into, default)]
        pub password_reset_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:username`. Only applies on resource creation. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub pgp_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The IAM user's name.
        #[builder(into)]
        pub user: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserLoginProfileResult {
        /// The encrypted password, base64 encoded. Only available if password was handled on resource creation, not import.
        pub encrypted_password: pulumi_wasm_rust::Output<String>,
        /// The fingerprint of the PGP key used to encrypt the password. Only available if password was handled on this provider resource creation, not import.
        pub key_fingerprint: pulumi_wasm_rust::Output<String>,
        /// The plain text password, only available when `pgp_key` is not provided.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The length of the generated password on resource creation. Only applies on resource creation. Drift detection is not possible with this argument. Default value is `20`.
        pub password_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether the user should be forced to reset the generated password on resource creation. Only applies on resource creation.
        pub password_reset_required: pulumi_wasm_rust::Output<bool>,
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:username`. Only applies on resource creation. Drift detection is not possible with this argument.
        pub pgp_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The IAM user's name.
        pub user: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserLoginProfileArgs) -> UserLoginProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let password_length_binding = args.password_length.get_inner();
        let password_reset_required_binding = args.password_reset_required.get_inner();
        let pgp_key_binding = args.pgp_key.get_inner();
        let user_binding = args.user.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/userLoginProfile:UserLoginProfile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "passwordLength".into(),
                    value: &password_length_binding,
                },
                register_interface::ObjectField {
                    name: "passwordResetRequired".into(),
                    value: &password_reset_required_binding,
                },
                register_interface::ObjectField {
                    name: "pgpKey".into(),
                    value: &pgp_key_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryptedPassword".into(),
                },
                register_interface::ResultField {
                    name: "keyFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "passwordLength".into(),
                },
                register_interface::ResultField {
                    name: "passwordResetRequired".into(),
                },
                register_interface::ResultField {
                    name: "pgpKey".into(),
                },
                register_interface::ResultField {
                    name: "user".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserLoginProfileResult {
            encrypted_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedPassword").unwrap(),
            ),
            key_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyFingerprint").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            password_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passwordLength").unwrap(),
            ),
            password_reset_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passwordResetRequired").unwrap(),
            ),
            pgp_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pgpKey").unwrap(),
            ),
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
        }
    }
}
