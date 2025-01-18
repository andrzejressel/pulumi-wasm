pub mod get_user_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserPoolArgs {
        /// The cognito pool ID
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetUserPoolArgs) -> GetUserPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserPool:getUserPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountRecoverySettings".into(),
                },
                register_interface::ResultField {
                    name: "adminCreateUserConfigs".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoVerifiedAttributes".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "customDomain".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "deviceConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "emailConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "estimatedNumberOfUsers".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lambdaConfigs".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedDate".into(),
                },
                register_interface::ResultField {
                    name: "mfaConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "schemaAttributes".into(),
                },
                register_interface::ResultField {
                    name: "smsAuthenticationMessage".into(),
                },
                register_interface::ResultField {
                    name: "smsConfigurationFailure".into(),
                },
                register_interface::ResultField {
                    name: "smsVerificationMessage".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
                register_interface::ResultField {
                    name: "userPoolTags".into(),
                },
                register_interface::ResultField {
                    name: "usernameAttributes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserPoolResult {
            account_recovery_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountRecoverySettings").unwrap(),
            ),
            admin_create_user_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminCreateUserConfigs").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_verified_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoVerifiedAttributes").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            custom_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomain").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            device_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceConfigurations").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            email_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailConfigurations").unwrap(),
            ),
            estimated_number_of_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("estimatedNumberOfUsers").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            lambda_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaConfigs").unwrap(),
            ),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedDate").unwrap(),
            ),
            mfa_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mfaConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            schema_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaAttributes").unwrap(),
            ),
            sms_authentication_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smsAuthenticationMessage").unwrap(),
            ),
            sms_configuration_failure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smsConfigurationFailure").unwrap(),
            ),
            sms_verification_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("smsVerificationMessage").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
            user_pool_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolTags").unwrap(),
            ),
            username_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usernameAttributes").unwrap(),
            ),
        }
    }
}
