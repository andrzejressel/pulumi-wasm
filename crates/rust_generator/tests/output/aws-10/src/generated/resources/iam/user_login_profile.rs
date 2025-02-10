/// Manages an IAM User Login Profile with limited support for password creation during this provider resource creation. Uses PGP to encrypt the password for safe transport to the user. PGP keys can be obtained from Keybase.
///
/// > To reset an IAM User login password via this provider, you can use delete and recreate this resource or change any of the arguments.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_login_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserLoginProfileArgs {
        /// The length of the generated password on resource creation. Only applies on resource creation. Drift detection is not possible with this argument. Default value is `20`.
        #[builder(into, default)]
        pub password_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether the user should be forced to reset the generated password on resource creation. Only applies on resource creation.
        #[builder(into, default)]
        pub password_reset_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:username`. Only applies on resource creation. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub pgp_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM user's name.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserLoginProfileResult {
        /// The encrypted password, base64 encoded. Only available if password was handled on resource creation, not import.
        pub encrypted_password: pulumi_gestalt_rust::Output<String>,
        /// The fingerprint of the PGP key used to encrypt the password. Only available if password was handled on this provider resource creation, not import.
        pub key_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The plain text password, only available when `pgp_key` is not provided.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The length of the generated password on resource creation. Only applies on resource creation. Drift detection is not possible with this argument. Default value is `20`.
        pub password_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Whether the user should be forced to reset the generated password on resource creation. Only applies on resource creation.
        pub password_reset_required: pulumi_gestalt_rust::Output<bool>,
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:username`. Only applies on resource creation. Drift detection is not possible with this argument.
        pub pgp_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IAM user's name.
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserLoginProfileArgs,
    ) -> UserLoginProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let password_length_binding = args.password_length.get_output(context);
        let password_reset_required_binding = args
            .password_reset_required
            .get_output(context);
        let pgp_key_binding = args.pgp_key.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/userLoginProfile:UserLoginProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passwordLength".into(),
                    value: password_length_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passwordResetRequired".into(),
                    value: password_reset_required_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pgpKey".into(),
                    value: pgp_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: user_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserLoginProfileResult {
            encrypted_password: o.get_field("encryptedPassword"),
            key_fingerprint: o.get_field("keyFingerprint"),
            password: o.get_field("password"),
            password_length: o.get_field("passwordLength"),
            password_reset_required: o.get_field("passwordResetRequired"),
            pgp_key: o.get_field("pgpKey"),
            user: o.get_field("user"),
        }
    }
}
