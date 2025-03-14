/// Provides a Cognito User Pool resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pool = user_pool::create(
///         "pool",
///         UserPoolArgs::builder().name("mypool").build_struct(),
///     );
/// }
/// ```
///
/// ### Enabling SMS and Software Token Multi-Factor Authentication
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_pool::create(
///         "example",
///         UserPoolArgs::builder()
///             .mfa_configuration("ON")
///             .sms_authentication_message("Your code is {####}")
///             .sms_configuration(
///                 UserPoolSmsConfiguration::builder()
///                     .externalId("example")
///                     .snsCallerArn("${exampleAwsIamRole.arn}")
///                     .snsRegion("us-east-1")
///                     .build_struct(),
///             )
///             .software_token_mfa_configuration(
///                 UserPoolSoftwareTokenMfaConfiguration::builder()
///                     .enabled(true)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using Account Recovery Setting
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = user_pool::create(
///         "test",
///         UserPoolArgs::builder()
///             .account_recovery_setting(
///                 UserPoolAccountRecoverySetting::builder()
///                     .recoveryMechanisms(
///                         vec![
///                             UserPoolAccountRecoverySettingRecoveryMechanism::builder()
///                             .name("verified_email").priority(1).build_struct(),
///                             UserPoolAccountRecoverySettingRecoveryMechanism::builder()
///                             .name("verified_phone_number").priority(2).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("mypool")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito User Pools using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/userPool:UserPool pool us-west-2_abc123
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPoolArgs {
        /// Configuration block to define which verified available method a user can use to recover their forgotten password. Detailed below.
        #[builder(into, default)]
        pub account_recovery_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolAccountRecoverySetting>,
        >,
        /// Configuration block for creating a new user profile. Detailed below.
        #[builder(into, default)]
        pub admin_create_user_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolAdminCreateUserConfig>,
        >,
        /// Attributes supported as an alias for this user pool. Valid values: `phone_number`, `email`, or `preferred_username`. Conflicts with `username_attributes`.
        #[builder(into, default)]
        pub alias_attributes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Attributes to be auto-verified. Valid values: `email`, `phone_number`.
        #[builder(into, default)]
        pub auto_verified_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// When active, DeletionProtection prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature. Valid values are `ACTIVE` and `INACTIVE`, Default value is `INACTIVE`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the user pool's device tracking. Detailed below.
        #[builder(into, default)]
        pub device_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolDeviceConfiguration>,
        >,
        /// Configuration block for configuring email. Detailed below.
        #[builder(into, default)]
        pub email_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolEmailConfiguration>,
        >,
        /// String representing the email verification message. Conflicts with `verification_message_template` configuration block `email_message` argument.
        #[builder(into, default)]
        pub email_verification_message: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// String representing the email verification subject. Conflicts with `verification_message_template` configuration block `email_subject` argument.
        #[builder(into, default)]
        pub email_verification_subject: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Configuration block for the AWS Lambda triggers associated with the user pool. Detailed below.
        #[builder(into, default)]
        pub lambda_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolLambdaConfig>,
        >,
        /// Multi-Factor Authentication (MFA) configuration for the User Pool. Defaults of `OFF`. Valid values are `OFF` (MFA Tokens are not required), `ON` (MFA is required for all users to sign in; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured), or `OPTIONAL` (MFA Will be required only for individual users who have MFA Enabled; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured).
        #[builder(into, default)]
        pub mfa_configuration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the user pool.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for information about the user pool password policy. Detailed below.
        #[builder(into, default)]
        pub password_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolPasswordPolicy>,
        >,
        /// Configuration block for the schema attributes of a user pool. Detailed below. Schema attributes from the [standard attribute set](https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#cognito-user-pools-standard-attributes) only need to be specified if they are different from the default configuration. Attributes can be added, but not modified or removed. Maximum of 50 attributes.
        #[builder(into, default)]
        pub schemas: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cognito::UserPoolSchema>>,
        >,
        /// String representing the SMS authentication message. The Message must contain the `{####}` placeholder, which will be replaced with the code.
        #[builder(into, default)]
        pub sms_authentication_message: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Configuration block for Short Message Service (SMS) settings. Detailed below. These settings apply to SMS user verification and SMS Multi-Factor Authentication (MFA). Due to Cognito API restrictions, the SMS configuration cannot be removed without recreating the Cognito User Pool. For user data safety, this resource will ignore the removal of this configuration by disabling drift detection. To force resource recreation after this configuration has been applied, see the `taint` command.
        #[builder(into, default)]
        pub sms_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolSmsConfiguration>,
        >,
        /// String representing the SMS verification message. Conflicts with `verification_message_template` configuration block `sms_message` argument.
        #[builder(into, default)]
        pub sms_verification_message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for software token Mult-Factor Authentication (MFA) settings. Detailed below.
        #[builder(into, default)]
        pub software_token_mfa_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolSoftwareTokenMfaConfiguration>,
        >,
        /// Map of tags to assign to the User Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for user attribute update settings. Detailed below.
        #[builder(into, default)]
        pub user_attribute_update_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolUserAttributeUpdateSettings>,
        >,
        /// Configuration block for user pool add-ons to enable user pool advanced security mode features. Detailed below.
        #[builder(into, default)]
        pub user_pool_add_ons: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolUserPoolAddOns>,
        >,
        /// Whether email addresses or phone numbers can be specified as usernames when a user signs up. Conflicts with `alias_attributes`.
        #[builder(into, default)]
        pub username_attributes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration block for username configuration. Detailed below.
        #[builder(into, default)]
        pub username_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolUsernameConfiguration>,
        >,
        /// Configuration block for verification message templates. Detailed below.
        #[builder(into, default)]
        pub verification_message_template: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognito::UserPoolVerificationMessageTemplate>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserPoolResult {
        /// Configuration block to define which verified available method a user can use to recover their forgotten password. Detailed below.
        pub account_recovery_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolAccountRecoverySetting>,
        >,
        /// Configuration block for creating a new user profile. Detailed below.
        pub admin_create_user_config: pulumi_gestalt_rust::Output<
            super::super::types::cognito::UserPoolAdminCreateUserConfig,
        >,
        /// Attributes supported as an alias for this user pool. Valid values: `phone_number`, `email`, or `preferred_username`. Conflicts with `username_attributes`.
        pub alias_attributes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// ARN of the user pool.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Attributes to be auto-verified. Valid values: `email`, `phone_number`.
        pub auto_verified_attributes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Date the user pool was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// A custom domain name that you provide to Amazon Cognito. This parameter applies only if you use a custom domain to host the sign-up and sign-in pages for your application. For example: `auth.example.com`.
        pub custom_domain: pulumi_gestalt_rust::Output<String>,
        /// When active, DeletionProtection prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature. Valid values are `ACTIVE` and `INACTIVE`, Default value is `INACTIVE`.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for the user pool's device tracking. Detailed below.
        pub device_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolDeviceConfiguration>,
        >,
        /// Holds the domain prefix if the user pool has a domain associated with it.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for configuring email. Detailed below.
        pub email_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolEmailConfiguration>,
        >,
        /// String representing the email verification message. Conflicts with `verification_message_template` configuration block `email_message` argument.
        pub email_verification_message: pulumi_gestalt_rust::Output<String>,
        /// String representing the email verification subject. Conflicts with `verification_message_template` configuration block `email_subject` argument.
        pub email_verification_subject: pulumi_gestalt_rust::Output<String>,
        /// Endpoint name of the user pool. Example format: `cognito-idp.REGION.amazonaws.com/xxxx_yyyyy`
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// A number estimating the size of the user pool.
        pub estimated_number_of_users: pulumi_gestalt_rust::Output<i32>,
        /// Configuration block for the AWS Lambda triggers associated with the user pool. Detailed below.
        pub lambda_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolLambdaConfig>,
        >,
        /// Date the user pool was last modified.
        pub last_modified_date: pulumi_gestalt_rust::Output<String>,
        /// Multi-Factor Authentication (MFA) configuration for the User Pool. Defaults of `OFF`. Valid values are `OFF` (MFA Tokens are not required), `ON` (MFA is required for all users to sign in; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured), or `OPTIONAL` (MFA Will be required only for individual users who have MFA Enabled; requires at least one of `sms_configuration` or `software_token_mfa_configuration` to be configured).
        pub mfa_configuration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the user pool.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for information about the user pool password policy. Detailed below.
        pub password_policy: pulumi_gestalt_rust::Output<
            super::super::types::cognito::UserPoolPasswordPolicy,
        >,
        /// Configuration block for the schema attributes of a user pool. Detailed below. Schema attributes from the [standard attribute set](https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-settings-attributes.html#cognito-user-pools-standard-attributes) only need to be specified if they are different from the default configuration. Attributes can be added, but not modified or removed. Maximum of 50 attributes.
        pub schemas: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cognito::UserPoolSchema>>,
        >,
        /// String representing the SMS authentication message. The Message must contain the `{####}` placeholder, which will be replaced with the code.
        pub sms_authentication_message: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block for Short Message Service (SMS) settings. Detailed below. These settings apply to SMS user verification and SMS Multi-Factor Authentication (MFA). Due to Cognito API restrictions, the SMS configuration cannot be removed without recreating the Cognito User Pool. For user data safety, this resource will ignore the removal of this configuration by disabling drift detection. To force resource recreation after this configuration has been applied, see the `taint` command.
        pub sms_configuration: pulumi_gestalt_rust::Output<
            super::super::types::cognito::UserPoolSmsConfiguration,
        >,
        /// String representing the SMS verification message. Conflicts with `verification_message_template` configuration block `sms_message` argument.
        pub sms_verification_message: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for software token Mult-Factor Authentication (MFA) settings. Detailed below.
        pub software_token_mfa_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolSoftwareTokenMfaConfiguration>,
        >,
        /// Map of tags to assign to the User Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for user attribute update settings. Detailed below.
        pub user_attribute_update_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolUserAttributeUpdateSettings>,
        >,
        /// Configuration block for user pool add-ons to enable user pool advanced security mode features. Detailed below.
        pub user_pool_add_ons: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolUserPoolAddOns>,
        >,
        /// Whether email addresses or phone numbers can be specified as usernames when a user signs up. Conflicts with `alias_attributes`.
        pub username_attributes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Configuration block for username configuration. Detailed below.
        pub username_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognito::UserPoolUsernameConfiguration>,
        >,
        /// Configuration block for verification message templates. Detailed below.
        pub verification_message_template: pulumi_gestalt_rust::Output<
            super::super::types::cognito::UserPoolVerificationMessageTemplate,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserPoolArgs,
    ) -> UserPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_recovery_setting_binding = args
            .account_recovery_setting
            .get_output(context);
        let admin_create_user_config_binding = args
            .admin_create_user_config
            .get_output(context);
        let alias_attributes_binding = args.alias_attributes.get_output(context);
        let auto_verified_attributes_binding = args
            .auto_verified_attributes
            .get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let device_configuration_binding = args.device_configuration.get_output(context);
        let email_configuration_binding = args.email_configuration.get_output(context);
        let email_verification_message_binding = args
            .email_verification_message
            .get_output(context);
        let email_verification_subject_binding = args
            .email_verification_subject
            .get_output(context);
        let lambda_config_binding = args.lambda_config.get_output(context);
        let mfa_configuration_binding = args.mfa_configuration.get_output(context);
        let name_binding = args.name.get_output(context);
        let password_policy_binding = args.password_policy.get_output(context);
        let schemas_binding = args.schemas.get_output(context);
        let sms_authentication_message_binding = args
            .sms_authentication_message
            .get_output(context);
        let sms_configuration_binding = args.sms_configuration.get_output(context);
        let sms_verification_message_binding = args
            .sms_verification_message
            .get_output(context);
        let software_token_mfa_configuration_binding = args
            .software_token_mfa_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_attribute_update_settings_binding = args
            .user_attribute_update_settings
            .get_output(context);
        let user_pool_add_ons_binding = args.user_pool_add_ons.get_output(context);
        let username_attributes_binding = args.username_attributes.get_output(context);
        let username_configuration_binding = args
            .username_configuration
            .get_output(context);
        let verification_message_template_binding = args
            .verification_message_template
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/userPool:UserPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountRecoverySetting".into(),
                    value: &account_recovery_setting_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminCreateUserConfig".into(),
                    value: &admin_create_user_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aliasAttributes".into(),
                    value: &alias_attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoVerifiedAttributes".into(),
                    value: &auto_verified_attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceConfiguration".into(),
                    value: &device_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailConfiguration".into(),
                    value: &email_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailVerificationMessage".into(),
                    value: &email_verification_message_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailVerificationSubject".into(),
                    value: &email_verification_subject_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambdaConfig".into(),
                    value: &lambda_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mfaConfiguration".into(),
                    value: &mfa_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passwordPolicy".into(),
                    value: &password_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemas".into(),
                    value: &schemas_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smsAuthenticationMessage".into(),
                    value: &sms_authentication_message_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smsConfiguration".into(),
                    value: &sms_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "smsVerificationMessage".into(),
                    value: &sms_verification_message_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "softwareTokenMfaConfiguration".into(),
                    value: &software_token_mfa_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userAttributeUpdateSettings".into(),
                    value: &user_attribute_update_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolAddOns".into(),
                    value: &user_pool_add_ons_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usernameAttributes".into(),
                    value: &username_attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usernameConfiguration".into(),
                    value: &username_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verificationMessageTemplate".into(),
                    value: &verification_message_template_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserPoolResult {
            account_recovery_setting: o.get_field("accountRecoverySetting"),
            admin_create_user_config: o.get_field("adminCreateUserConfig"),
            alias_attributes: o.get_field("aliasAttributes"),
            arn: o.get_field("arn"),
            auto_verified_attributes: o.get_field("autoVerifiedAttributes"),
            creation_date: o.get_field("creationDate"),
            custom_domain: o.get_field("customDomain"),
            deletion_protection: o.get_field("deletionProtection"),
            device_configuration: o.get_field("deviceConfiguration"),
            domain: o.get_field("domain"),
            email_configuration: o.get_field("emailConfiguration"),
            email_verification_message: o.get_field("emailVerificationMessage"),
            email_verification_subject: o.get_field("emailVerificationSubject"),
            endpoint: o.get_field("endpoint"),
            estimated_number_of_users: o.get_field("estimatedNumberOfUsers"),
            lambda_config: o.get_field("lambdaConfig"),
            last_modified_date: o.get_field("lastModifiedDate"),
            mfa_configuration: o.get_field("mfaConfiguration"),
            name: o.get_field("name"),
            password_policy: o.get_field("passwordPolicy"),
            schemas: o.get_field("schemas"),
            sms_authentication_message: o.get_field("smsAuthenticationMessage"),
            sms_configuration: o.get_field("smsConfiguration"),
            sms_verification_message: o.get_field("smsVerificationMessage"),
            software_token_mfa_configuration: o
                .get_field("softwareTokenMfaConfiguration"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_attribute_update_settings: o.get_field("userAttributeUpdateSettings"),
            user_pool_add_ons: o.get_field("userPoolAddOns"),
            username_attributes: o.get_field("usernameAttributes"),
            username_configuration: o.get_field("usernameConfiguration"),
            verification_message_template: o.get_field("verificationMessageTemplate"),
        }
    }
}
