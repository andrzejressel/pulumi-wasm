/// The AuthConfig resource use to hold channels and connection config data.
///
///
/// To get more information about AuthConfig, see:
///
/// * [API documentation](https://cloud.google.com/application-integration/docs/reference/rest/v1/projects.locations.authConfigs)
/// * How-to Guides
///     * [Manage authentication profiles](https://cloud.google.com/application-integration/docs/configure-authentication-profiles)
///     * [Official Documentation](https://cloud.google.com/application-integration/docs/overview)
///
/// ## Example Usage
///
/// ### Integrations Auth Config Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicExample = auth_config::create(
///         "basicExample",
///         AuthConfigArgs::builder()
///             .decrypted_credential(
///                 AuthConfigDecryptedCredential::builder()
///                     .credentialType("USERNAME_AND_PASSWORD")
///                     .usernameAndPassword(
///                         AuthConfigDecryptedCredentialUsernameAndPassword::builder()
///                             .password("test-password")
///                             .username("test-username")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("Test auth config created via terraform")
///             .display_name("test-authconfig")
///             .location("us-west1")
///             .build_struct(),
///     );
///     let client = client::create(
///         "client",
///         ClientArgs::builder().location("us-west1").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AuthConfig can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AuthConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:applicationintegration/authConfig:AuthConfig default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:applicationintegration/authConfig:AuthConfig default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:applicationintegration/authConfig:AuthConfig default {{name}}
/// ```
///
pub mod auth_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthConfigArgs {
        /// Raw client certificate
        /// Structure is documented below.
        #[builder(into, default)]
        pub client_certificate: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::applicationintegration::AuthConfigClientCertificate,
            >,
        >,
        /// Raw auth credentials.
        /// Structure is documented below.
        #[builder(into, default)]
        pub decrypted_credential: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::applicationintegration::AuthConfigDecryptedCredential,
            >,
        >,
        /// A description of the auth config.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the auth config.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub expiry_notification_durations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Location in which client needs to be provisioned.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        #[builder(into, default)]
        pub override_valid_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The visibility of the auth config.
        /// Possible values are: `PRIVATE`, `CLIENT_VISIBLE`.
        #[builder(into, default)]
        pub visibility: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthConfigResult {
        /// Certificate id for client certificate.
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        /// Raw client certificate
        /// Structure is documented below.
        pub client_certificate: pulumi_wasm_rust::Output<
            Option<
                super::super::types::applicationintegration::AuthConfigClientCertificate,
            >,
        >,
        /// The timestamp when the auth config is created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call.
        pub creator_email: pulumi_wasm_rust::Output<String>,
        /// Credential type of the encrypted credential.
        pub credential_type: pulumi_wasm_rust::Output<String>,
        /// Raw auth credentials.
        /// Structure is documented below.
        pub decrypted_credential: pulumi_wasm_rust::Output<
            Option<
                super::super::types::applicationintegration::AuthConfigDecryptedCredential,
            >,
        >,
        /// A description of the auth config.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the auth config.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Auth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key.
        /// A base64-encoded string.
        pub encrypted_credential: pulumi_wasm_rust::Output<String>,
        /// User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub expiry_notification_durations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call.
        pub last_modifier_email: pulumi_wasm_rust::Output<String>,
        /// Location in which client needs to be provisioned.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Resource name of the auth config.
        pub name: pulumi_wasm_rust::Output<String>,
        /// User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub override_valid_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The reason / details of the current status.
        pub reason: pulumi_wasm_rust::Output<String>,
        /// The status of the auth config.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the auth config is modified.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The time until the auth config is valid. Empty or max value is considered the auth config won't expire.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub valid_time: pulumi_wasm_rust::Output<String>,
        /// The visibility of the auth config.
        /// Possible values are: `PRIVATE`, `CLIENT_VISIBLE`.
        pub visibility: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AuthConfigArgs,
    ) -> AuthConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_certificate_binding = args
            .client_certificate
            .get_output(context)
            .get_inner();
        let decrypted_credential_binding = args
            .decrypted_credential
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let expiry_notification_durations_binding = args
            .expiry_notification_durations
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let override_valid_time_binding = args
            .override_valid_time
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let visibility_binding = args.visibility.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:applicationintegration/authConfig:AuthConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientCertificate".into(),
                    value: &client_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "decryptedCredential".into(),
                    value: &decrypted_credential_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "expiryNotificationDurations".into(),
                    value: &expiry_notification_durations_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "overrideValidTime".into(),
                    value: &override_valid_time_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "visibility".into(),
                    value: &visibility_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateId".into(),
                },
                register_interface::ResultField {
                    name: "clientCertificate".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "creatorEmail".into(),
                },
                register_interface::ResultField {
                    name: "credentialType".into(),
                },
                register_interface::ResultField {
                    name: "decryptedCredential".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "encryptedCredential".into(),
                },
                register_interface::ResultField {
                    name: "expiryNotificationDurations".into(),
                },
                register_interface::ResultField {
                    name: "lastModifierEmail".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "overrideValidTime".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "reason".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "validTime".into(),
                },
                register_interface::ResultField {
                    name: "visibility".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthConfigResult {
            certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateId").unwrap(),
            ),
            client_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientCertificate").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            creator_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creatorEmail").unwrap(),
            ),
            credential_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialType").unwrap(),
            ),
            decrypted_credential: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("decryptedCredential").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            encrypted_credential: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedCredential").unwrap(),
            ),
            expiry_notification_durations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiryNotificationDurations").unwrap(),
            ),
            last_modifier_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifierEmail").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            override_valid_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overrideValidTime").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reason").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            valid_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validTime").unwrap(),
            ),
            visibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibility").unwrap(),
            ),
        }
    }
}
