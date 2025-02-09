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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod auth_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthConfigArgs {
        /// Raw client certificate
        /// Structure is documented below.
        #[builder(into, default)]
        pub client_certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::applicationintegration::AuthConfigClientCertificate,
            >,
        >,
        /// Raw auth credentials.
        /// Structure is documented below.
        #[builder(into, default)]
        pub decrypted_credential: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::applicationintegration::AuthConfigDecryptedCredential,
            >,
        >,
        /// A description of the auth config.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the auth config.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub expiry_notification_durations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Location in which client needs to be provisioned.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        #[builder(into, default)]
        pub override_valid_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The visibility of the auth config.
        /// Possible values are: `PRIVATE`, `CLIENT_VISIBLE`.
        #[builder(into, default)]
        pub visibility: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthConfigResult {
        /// Certificate id for client certificate.
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
        /// Raw client certificate
        /// Structure is documented below.
        pub client_certificate: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::applicationintegration::AuthConfigClientCertificate,
            >,
        >,
        /// The timestamp when the auth config is created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The creator's email address. Generated based on the End User Credentials/LOAS role of the user making the call.
        pub creator_email: pulumi_gestalt_rust::Output<String>,
        /// Credential type of the encrypted credential.
        pub credential_type: pulumi_gestalt_rust::Output<String>,
        /// Raw auth credentials.
        /// Structure is documented below.
        pub decrypted_credential: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::applicationintegration::AuthConfigDecryptedCredential,
            >,
        >,
        /// A description of the auth config.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the auth config.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Auth credential encrypted by Cloud KMS. Can be decrypted as Credential with proper KMS key.
        /// A base64-encoded string.
        pub encrypted_credential: pulumi_gestalt_rust::Output<String>,
        /// User can define the time to receive notification after which the auth config becomes invalid. Support up to 30 days. Support granularity in hours.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub expiry_notification_durations: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The last modifier's email address. Generated based on the End User Credentials/LOAS role of the user making the call.
        pub last_modifier_email: pulumi_gestalt_rust::Output<String>,
        /// Location in which client needs to be provisioned.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Resource name of the auth config.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// User provided expiry time to override. For the example of Salesforce, username/password credentials can be valid for 6 months depending on the instance settings.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub override_valid_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The reason / details of the current status.
        pub reason: pulumi_gestalt_rust::Output<String>,
        /// The status of the auth config.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the auth config is modified.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The time until the auth config is valid. Empty or max value is considered the auth config won't expire.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub valid_time: pulumi_gestalt_rust::Output<String>,
        /// The visibility of the auth config.
        /// Possible values are: `PRIVATE`, `CLIENT_VISIBLE`.
        pub visibility: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthConfigArgs,
    ) -> AuthConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let client_certificate_binding = args.client_certificate.get_output(context);
        let decrypted_credential_binding = args.decrypted_credential.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let expiry_notification_durations_binding = args
            .expiry_notification_durations
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let override_valid_time_binding = args.override_valid_time.get_output(context);
        let project_binding = args.project.get_output(context);
        let visibility_binding = args.visibility.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:applicationintegration/authConfig:AuthConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificate".into(),
                    value: client_certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "decryptedCredential".into(),
                    value: decrypted_credential_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiryNotificationDurations".into(),
                    value: expiry_notification_durations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrideValidTime".into(),
                    value: override_valid_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "visibility".into(),
                    value: visibility_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthConfigResult {
            certificate_id: o.get_field("certificateId"),
            client_certificate: o.get_field("clientCertificate"),
            create_time: o.get_field("createTime"),
            creator_email: o.get_field("creatorEmail"),
            credential_type: o.get_field("credentialType"),
            decrypted_credential: o.get_field("decryptedCredential"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            encrypted_credential: o.get_field("encryptedCredential"),
            expiry_notification_durations: o.get_field("expiryNotificationDurations"),
            last_modifier_email: o.get_field("lastModifierEmail"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            override_valid_time: o.get_field("overrideValidTime"),
            project: o.get_field("project"),
            reason: o.get_field("reason"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
            valid_time: o.get_field("validTime"),
            visibility: o.get_field("visibility"),
        }
    }
}
