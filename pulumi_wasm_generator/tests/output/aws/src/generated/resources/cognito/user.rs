/// Provides a Cognito User Resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_pool::create(
///         "example",
///         UserPoolArgs::builder().name("MyExamplePool").build_struct(),
///     );
///     let exampleUser = user::create(
///         "exampleUser",
///         UserArgs::builder()
///             .user_pool_id("${example.id}")
///             .username("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Setting user attributes
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cognito:UserPool
///     properties:
///       name: mypool
///       schemas:
///         - name: example
///           attributeDataType: Boolean
///           mutable: false
///           required: false
///           developerOnlyAttribute: false
///         - name: foo
///           attributeDataType: String
///           mutable: false
///           required: false
///           developerOnlyAttribute: false
///           stringAttributeConstraints: {}
///   exampleUser:
///     type: aws:cognito:User
///     name: example
///     properties:
///       userPoolId: ${example.id}
///       username: example
///       attributes:
///         example: true
///         foo: bar
///         email: no-reply@example.com
///         email_verified: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito User using the `user_pool_id`/`name` attributes concatenated. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/user:User user us-east-1_vG78M4goG/user
/// ```
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// A map that contains user attributes and attribute values to be set for the user.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of custom key-value pairs that you can provide as input for any custom workflows that user creation triggers. Amazon Cognito does not store the `client_metadata` value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).
        #[builder(into, default)]
        pub client_metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of mediums to the welcome message will be sent through. Allowed values are `EMAIL` and `SMS`. If it's provided, make sure you have also specified `email` attribute for the `EMAIL` medium and `phone_number` for the `SMS`. More than one value can be specified. Amazon Cognito does not store the `desired_delivery_mediums` value. Defaults to `["SMS"]`.
        #[builder(into, default)]
        pub desired_delivery_mediums: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies whether the user should be enabled after creation. The welcome message will be sent regardless of the `enabled` value. The behavior can be changed with `message_action` argument. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// If this parameter is set to True and the `phone_number` or `email` address specified in the `attributes` parameter already exists as an alias with a different user, Amazon Cognito will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias. Amazon Cognito does not store the `force_alias_creation` value. Defaults to `false`.
        #[builder(into, default)]
        pub force_alias_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set to `RESEND` to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to `SUPPRESS` to suppress sending the message. Only one value can be specified. Amazon Cognito does not store the `message_action` value.
        #[builder(into, default)]
        pub message_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The user's permanent password. This password must conform to the password policy specified by user pool the user belongs to. The welcome message always contains only `temporary_password` value. You can suppress sending the welcome message with the `message_action` argument. Amazon Cognito does not store the `password` value. Conflicts with `temporary_password`.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The user's temporary password. Conflicts with `password`.
        #[builder(into, default)]
        pub temporary_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The user pool ID for the user pool where the user will be created.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username cannot be changed.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub username: pulumi_wasm_rust::Output<String>,
        /// The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. Amazon Cognito does not store the `validation_data` value. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).
        ///
        /// > **NOTE:** Clearing `password` or `temporary_password` does not reset user's password in Cognito.
        #[builder(into, default)]
        pub validation_data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// A map that contains user attributes and attribute values to be set for the user.
        pub attributes: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of custom key-value pairs that you can provide as input for any custom workflows that user creation triggers. Amazon Cognito does not store the `client_metadata` value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).
        pub client_metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// A list of mediums to the welcome message will be sent through. Allowed values are `EMAIL` and `SMS`. If it's provided, make sure you have also specified `email` attribute for the `EMAIL` medium and `phone_number` for the `SMS`. More than one value can be specified. Amazon Cognito does not store the `desired_delivery_mediums` value. Defaults to `["SMS"]`.
        pub desired_delivery_mediums: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies whether the user should be enabled after creation. The welcome message will be sent regardless of the `enabled` value. The behavior can be changed with `message_action` argument. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// If this parameter is set to True and the `phone_number` or `email` address specified in the `attributes` parameter already exists as an alias with a different user, Amazon Cognito will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias. Amazon Cognito does not store the `force_alias_creation` value. Defaults to `false`.
        pub force_alias_creation: pulumi_wasm_rust::Output<Option<bool>>,
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        /// Set to `RESEND` to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to `SUPPRESS` to suppress sending the message. Only one value can be specified. Amazon Cognito does not store the `message_action` value.
        pub message_action: pulumi_wasm_rust::Output<Option<String>>,
        pub mfa_setting_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The user's permanent password. This password must conform to the password policy specified by user pool the user belongs to. The welcome message always contains only `temporary_password` value. You can suppress sending the welcome message with the `message_action` argument. Amazon Cognito does not store the `password` value. Conflicts with `temporary_password`.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        pub preferred_mfa_setting: pulumi_wasm_rust::Output<String>,
        /// current user status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// unique user id that is never reassignable to another user.
        pub sub: pulumi_wasm_rust::Output<String>,
        /// The user's temporary password. Conflicts with `password`.
        pub temporary_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The user pool ID for the user pool where the user will be created.
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
        /// The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username cannot be changed.
        ///
        /// The following arguments are optional:
        pub username: pulumi_wasm_rust::Output<String>,
        /// The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. Amazon Cognito does not store the `validation_data` value. For more information, see [Customizing User Pool Workflows with Lambda Triggers](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html).
        ///
        /// > **NOTE:** Clearing `password` or `temporary_password` does not reset user's password in Cognito.
        pub validation_data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserArgs) -> UserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_inner();
        let client_metadata_binding = args.client_metadata.get_inner();
        let desired_delivery_mediums_binding = args.desired_delivery_mediums.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let force_alias_creation_binding = args.force_alias_creation.get_inner();
        let message_action_binding = args.message_action.get_inner();
        let password_binding = args.password.get_inner();
        let temporary_password_binding = args.temporary_password.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let username_binding = args.username.get_inner();
        let validation_data_binding = args.validation_data.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/user:User".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "clientMetadata".into(),
                    value: &client_metadata_binding,
                },
                register_interface::ObjectField {
                    name: "desiredDeliveryMediums".into(),
                    value: &desired_delivery_mediums_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "forceAliasCreation".into(),
                    value: &force_alias_creation_binding,
                },
                register_interface::ObjectField {
                    name: "messageAction".into(),
                    value: &message_action_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "temporaryPassword".into(),
                    value: &temporary_password_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
                register_interface::ObjectField {
                    name: "validationData".into(),
                    value: &validation_data_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "clientMetadata".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "desiredDeliveryMediums".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "forceAliasCreation".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedDate".into(),
                },
                register_interface::ResultField {
                    name: "messageAction".into(),
                },
                register_interface::ResultField {
                    name: "mfaSettingLists".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "preferredMfaSetting".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "sub".into(),
                },
                register_interface::ResultField {
                    name: "temporaryPassword".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
                register_interface::ResultField {
                    name: "validationData".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserResult {
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            client_metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientMetadata").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            desired_delivery_mediums: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredDeliveryMediums").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            force_alias_creation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceAliasCreation").unwrap(),
            ),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedDate").unwrap(),
            ),
            message_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("messageAction").unwrap(),
            ),
            mfa_setting_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mfaSettingLists").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            preferred_mfa_setting: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMfaSetting").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            sub: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sub").unwrap(),
            ),
            temporary_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("temporaryPassword").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
            validation_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationData").unwrap(),
            ),
        }
    }
}
