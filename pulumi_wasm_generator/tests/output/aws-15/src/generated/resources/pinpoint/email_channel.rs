/// Provides a Pinpoint Email Channel resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   email:
///     type: aws:pinpoint:EmailChannel
///     properties:
///       applicationId: ${app.applicationId}
///       fromAddress: user@example.com
///       roleArn: ${role.arn}
///   app:
///     type: aws:pinpoint:App
///   identity:
///     type: aws:ses:DomainIdentity
///     properties:
///       domain: example.com
///   role:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy: ${assumeRole.json}
///   rolePolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: role_policy
///     properties:
///       name: role_policy
///       role: ${role.id}
///       policy: ${rolePolicy.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - pinpoint.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   rolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - mobileanalytics:PutEvents
///               - mobileanalytics:PutItems
///             resources:
///               - '*'
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailChannelArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ARN of the Amazon SES configuration set that you want to apply to messages that you send through the channel.
        #[builder(into, default)]
        pub configuration_set: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The email address used to send emails from. You can use email only (`user@example.com`) or friendly address (`User <user@example.com>`). This field comply with [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt).
        #[builder(into)]
        pub from_address: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ARN of an identity verified with SES.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::InputOrOutput<String>,
        /// *Deprecated* The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EmailChannelArgs,
    ) -> EmailChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let configuration_set_binding = args
            .configuration_set
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let from_address_binding = args.from_address.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/emailChannel:EmailChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EmailChannelResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            configuration_set: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationSet"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            from_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fromAddress"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            messages_per_second: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("messagesPerSecond"),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
        }
    }
}
