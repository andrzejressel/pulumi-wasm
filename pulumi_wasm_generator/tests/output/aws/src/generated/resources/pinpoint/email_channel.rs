/// Provides a Pinpoint Email Channel resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["pinpoint.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let rolePolicy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["mobileanalytics:PutEvents",
///                     "mobileanalytics:PutItems",]).effect("Allow").resources(vec!["*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let app = app::create("app", AppArgs::builder().build_struct());
///     let email = email_channel::create(
///         "email",
///         EmailChannelArgs::builder()
///             .application_id("${app.applicationId}")
///             .from_address("user@example.com")
///             .role_arn("${role.arn}")
///             .build_struct(),
///     );
///     let identity = domain_identity::create(
///         "identity",
///         DomainIdentityArgs::builder().domain("example.com").build_struct(),
///     );
///     let role = role::create(
///         "role",
///         RoleArgs::builder().assume_role_policy("${assumeRole.json}").build_struct(),
///     );
///     let rolePolicyRolePolicy = role_policy::create(
///         "rolePolicyRolePolicy",
///         RolePolicyArgs::builder()
///             .name("role_policy")
///             .policy("${rolePolicy.json}")
///             .role("${role.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint Email Channel using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/emailChannel:EmailChannel email application-id
/// ```
pub mod email_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailChannelArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Amazon SES configuration set that you want to apply to messages that you send through the channel.
        #[builder(into, default)]
        pub configuration_set: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The email address used to send emails from. You can use email only (`user@example.com`) or friendly address (`User <user@example.com>`). This field comply with [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt).
        #[builder(into)]
        pub from_address: pulumi_wasm_rust::Output<String>,
        /// The ARN of an identity verified with SES.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::Output<String>,
        /// *Deprecated* The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EmailChannelResult {
        /// The application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Amazon SES configuration set that you want to apply to messages that you send through the channel.
        pub configuration_set: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The email address used to send emails from. You can use email only (`user@example.com`) or friendly address (`User <user@example.com>`). This field comply with [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt).
        pub from_address: pulumi_wasm_rust::Output<String>,
        /// The ARN of an identity verified with SES.
        pub identity: pulumi_wasm_rust::Output<String>,
        /// Messages per second that can be sent.
        pub messages_per_second: pulumi_wasm_rust::Output<i32>,
        /// *Deprecated* The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EmailChannelArgs) -> EmailChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let configuration_set_binding = args.configuration_set.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let from_address_binding = args.from_address.get_inner();
        let identity_binding = args.identity.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/emailChannel:EmailChannel".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "configurationSet".into(),
                    value: &configuration_set_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "fromAddress".into(),
                    value: &from_address_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "configurationSet".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "fromAddress".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "messagesPerSecond".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EmailChannelResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            configuration_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationSet").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            from_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fromAddress").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            messages_per_second: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("messagesPerSecond").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
        }
    }
}