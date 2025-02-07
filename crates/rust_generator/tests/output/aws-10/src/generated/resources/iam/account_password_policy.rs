/// > **Note:** There is only a single policy allowed per AWS account. An existing policy will be lost when using this resource as an effect of this limitation.
///
/// Manages Password Policy for the AWS Account.
/// See more about [Account Password Policy](http://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_passwords_account-policy.html)
/// in the official AWS docs.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let strict = account_password_policy::create(
///         "strict",
///         AccountPasswordPolicyArgs::builder()
///             .allow_users_to_change_password(true)
///             .minimum_password_length(8)
///             .require_lowercase_characters(true)
///             .require_numbers(true)
///             .require_symbols(true)
///             .require_uppercase_characters(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Account Password Policy using the word `iam-account-password-policy`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/accountPasswordPolicy:AccountPasswordPolicy strict iam-account-password-policy
/// ```
pub mod account_password_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountPasswordPolicyArgs {
        /// Whether to allow users to change their own password
        #[builder(into, default)]
        pub allow_users_to_change_password: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether users are prevented from setting a new password after their password has expired (i.e., require administrator reset)
        #[builder(into, default)]
        pub hard_expiry: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The number of days that an user password is valid.
        #[builder(into, default)]
        pub max_password_age: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum length to require for user passwords.
        #[builder(into, default)]
        pub minimum_password_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of previous passwords that users are prevented from reusing.
        #[builder(into, default)]
        pub password_reuse_prevention: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether to require lowercase characters for user passwords.
        #[builder(into, default)]
        pub require_lowercase_characters: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to require numbers for user passwords.
        #[builder(into, default)]
        pub require_numbers: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to require symbols for user passwords.
        #[builder(into, default)]
        pub require_symbols: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to require uppercase characters for user passwords.
        #[builder(into, default)]
        pub require_uppercase_characters: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountPasswordPolicyResult {
        /// Whether to allow users to change their own password
        pub allow_users_to_change_password: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates whether passwords in the account expire. Returns `true` if `max_password_age` contains a value greater than `0`. Returns `false` if it is `0` or _not present_.
        pub expire_passwords: pulumi_gestalt_rust::Output<bool>,
        /// Whether users are prevented from setting a new password after their password has expired (i.e., require administrator reset)
        pub hard_expiry: pulumi_gestalt_rust::Output<bool>,
        /// The number of days that an user password is valid.
        pub max_password_age: pulumi_gestalt_rust::Output<i32>,
        /// Minimum length to require for user passwords.
        pub minimum_password_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of previous passwords that users are prevented from reusing.
        pub password_reuse_prevention: pulumi_gestalt_rust::Output<i32>,
        /// Whether to require lowercase characters for user passwords.
        pub require_lowercase_characters: pulumi_gestalt_rust::Output<bool>,
        /// Whether to require numbers for user passwords.
        pub require_numbers: pulumi_gestalt_rust::Output<bool>,
        /// Whether to require symbols for user passwords.
        pub require_symbols: pulumi_gestalt_rust::Output<bool>,
        /// Whether to require uppercase characters for user passwords.
        pub require_uppercase_characters: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccountPasswordPolicyArgs,
    ) -> AccountPasswordPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allow_users_to_change_password_binding = args
            .allow_users_to_change_password
            .get_output(context)
            .get_inner();
        let hard_expiry_binding = args.hard_expiry.get_output(context).get_inner();
        let max_password_age_binding = args
            .max_password_age
            .get_output(context)
            .get_inner();
        let minimum_password_length_binding = args
            .minimum_password_length
            .get_output(context)
            .get_inner();
        let password_reuse_prevention_binding = args
            .password_reuse_prevention
            .get_output(context)
            .get_inner();
        let require_lowercase_characters_binding = args
            .require_lowercase_characters
            .get_output(context)
            .get_inner();
        let require_numbers_binding = args
            .require_numbers
            .get_output(context)
            .get_inner();
        let require_symbols_binding = args
            .require_symbols
            .get_output(context)
            .get_inner();
        let require_uppercase_characters_binding = args
            .require_uppercase_characters
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/accountPasswordPolicy:AccountPasswordPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowUsersToChangePassword".into(),
                    value: &allow_users_to_change_password_binding,
                },
                register_interface::ObjectField {
                    name: "hardExpiry".into(),
                    value: &hard_expiry_binding,
                },
                register_interface::ObjectField {
                    name: "maxPasswordAge".into(),
                    value: &max_password_age_binding,
                },
                register_interface::ObjectField {
                    name: "minimumPasswordLength".into(),
                    value: &minimum_password_length_binding,
                },
                register_interface::ObjectField {
                    name: "passwordReusePrevention".into(),
                    value: &password_reuse_prevention_binding,
                },
                register_interface::ObjectField {
                    name: "requireLowercaseCharacters".into(),
                    value: &require_lowercase_characters_binding,
                },
                register_interface::ObjectField {
                    name: "requireNumbers".into(),
                    value: &require_numbers_binding,
                },
                register_interface::ObjectField {
                    name: "requireSymbols".into(),
                    value: &require_symbols_binding,
                },
                register_interface::ObjectField {
                    name: "requireUppercaseCharacters".into(),
                    value: &require_uppercase_characters_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountPasswordPolicyResult {
            allow_users_to_change_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowUsersToChangePassword"),
            ),
            expire_passwords: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirePasswords"),
            ),
            hard_expiry: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hardExpiry"),
            ),
            max_password_age: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxPasswordAge"),
            ),
            minimum_password_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumPasswordLength"),
            ),
            password_reuse_prevention: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("passwordReusePrevention"),
            ),
            require_lowercase_characters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requireLowercaseCharacters"),
            ),
            require_numbers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requireNumbers"),
            ),
            require_symbols: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requireSymbols"),
            ),
            require_uppercase_characters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requireUppercaseCharacters"),
            ),
        }
    }
}
