pub mod get_user_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserPoolArgs {
        /// The cognito pool ID
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserPoolResult {
        pub account_recovery_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognito::GetUserPoolAccountRecoverySetting>,
        >,
        pub admin_create_user_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognito::GetUserPoolAdminCreateUserConfig>,
        >,
        /// ARN of the User Pool.
        /// * account_recovery_setting - The available verified method a user can use to recover their password when they call ForgotPassword. You can use this setting to define a preferred method when a user has more than one method available. With this setting, SMS doesn't qualify for a valid password recovery mechanism if the user also has SMS multi-factor authentication (MFA) activated. In the absence of this setting, Amazon Cognito uses the legacy behavior to determine the recovery method where SMS is preferred through email.
        /// * admin_create_user_config - The configuration for AdminCreateUser requests.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The attributes that are auto-verified in a user pool.
        pub auto_verified_attributes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The date and time, in ISO 8601 format, when the item was created.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// A custom domain name that you provide to Amazon Cognito. This parameter applies only if you use a custom domain to host the sign-up and sign-in pages for your application. An example of a custom domain name might be auth.example.com.
        pub custom_domain: pulumi_wasm_rust::Output<String>,
        /// When active, DeletionProtection prevents accidental deletion of your user pool. Before you can delete a user pool that you have protected against deletion, you must deactivate this feature.
        /// * device_configuration - The device-remembering configuration for a user pool. A null value indicates that you have deactivated device remembering in your user pool.
        pub deletion_protection: pulumi_wasm_rust::Output<String>,
        pub device_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognito::GetUserPoolDeviceConfiguration>,
        >,
        /// The domain prefix, if the user pool has a domain associated with it.
        /// * email_configuration - The email configuration of your user pool. The email configuration type sets your preferred sending method, AWS Region, and sender for messages from your user pool.
        pub domain: pulumi_wasm_rust::Output<String>,
        pub email_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognito::GetUserPoolEmailConfiguration>,
        >,
        /// A number estimating the size of the user pool.
        /// * lambda_config - The AWS Lambda triggers associated with the user pool.
        pub estimated_number_of_users: pulumi_wasm_rust::Output<i32>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub lambda_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognito::GetUserPoolLambdaConfig>,
        >,
        /// The date and time, in ISO 8601 format, when the item was modified.
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        /// Can be one of the following values: `OFF` | `ON` | `OPTIONAL`
        pub mfa_configuration: pulumi_wasm_rust::Output<String>,
        /// - Name of the attribute.
        pub name: pulumi_wasm_rust::Output<String>,
        pub schema_attributes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cognito::GetUserPoolSchemaAttribute>,
        >,
        /// The contents of the SMS authentication message.
        pub sms_authentication_message: pulumi_wasm_rust::Output<String>,
        /// The reason why the SMS configuration can't send the messages to your users.
        pub sms_configuration_failure: pulumi_wasm_rust::Output<String>,
        /// The contents of the SMS authentication message.
        pub sms_verification_message: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// (Deprecated) Map of tags assigned to the resource.
        pub user_pool_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies whether a user can use an email address or phone number as a username when they sign up.
        pub username_attributes: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserPoolArgs,
    ) -> GetUserPoolResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserPool:getUserPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserPoolResult {
            account_recovery_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountRecoverySettings"),
            ),
            admin_create_user_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminCreateUserConfigs"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            auto_verified_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoVerifiedAttributes"),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationDate"),
            ),
            custom_domain: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDomain"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            device_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deviceConfigurations"),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            email_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("emailConfigurations"),
            ),
            estimated_number_of_users: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("estimatedNumberOfUsers"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            lambda_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lambdaConfigs"),
            ),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifiedDate"),
            ),
            mfa_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mfaConfiguration"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            schema_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("schemaAttributes"),
            ),
            sms_authentication_message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("smsAuthenticationMessage"),
            ),
            sms_configuration_failure: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("smsConfigurationFailure"),
            ),
            sms_verification_message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("smsVerificationMessage"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
            user_pool_tags: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userPoolTags"),
            ),
            username_attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("usernameAttributes"),
            ),
        }
    }
}
