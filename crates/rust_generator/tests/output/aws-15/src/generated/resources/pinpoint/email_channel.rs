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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailChannelArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the Amazon SES configuration set that you want to apply to messages that you send through the channel.
        #[builder(into, default)]
        pub configuration_set: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The email address used to send emails from. You can use email only (`user@example.com`) or friendly address (`User <user@example.com>`). This field comply with [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt).
        #[builder(into)]
        pub from_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of an identity verified with SES.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// *Deprecated* The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EmailChannelResult {
        /// The application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Amazon SES configuration set that you want to apply to messages that you send through the channel.
        pub configuration_set: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The email address used to send emails from. You can use email only (`user@example.com`) or friendly address (`User <user@example.com>`). This field comply with [RFC 5322](https://www.ietf.org/rfc/rfc5322.txt).
        pub from_address: pulumi_gestalt_rust::Output<String>,
        /// The ARN of an identity verified with SES.
        pub identity: pulumi_gestalt_rust::Output<String>,
        /// Messages per second that can be sent.
        pub messages_per_second: pulumi_gestalt_rust::Output<i32>,
        /// *Deprecated* The ARN of an IAM Role used to submit events to Mobile Analytics' event ingestion service.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailChannelArgs,
    ) -> EmailChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let configuration_set_binding = args.configuration_set.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let from_address_binding = args.from_address.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/emailChannel:EmailChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationSet".into(),
                    value: configuration_set_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fromAddress".into(),
                    value: from_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailChannelResult {
            application_id: o.get_field("applicationId"),
            configuration_set: o.get_field("configurationSet"),
            enabled: o.get_field("enabled"),
            from_address: o.get_field("fromAddress"),
            identity: o.get_field("identity"),
            messages_per_second: o.get_field("messagesPerSecond"),
            role_arn: o.get_field("roleArn"),
        }
    }
}
