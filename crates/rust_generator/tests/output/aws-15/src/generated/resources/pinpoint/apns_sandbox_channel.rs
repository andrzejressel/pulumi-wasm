/// Provides a Pinpoint APNs Sandbox Channel resource.
///
/// > **Note:** All arguments, including certificates and tokens, will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```yaml
/// resources:
///   apnsSandbox:
///     type: aws:pinpoint:ApnsSandboxChannel
///     name: apns_sandbox
///     properties:
///       applicationId: ${app.applicationId}
///       certificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ./certificate.pem
///           return: result
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ./private_key.key
///           return: result
///   app:
///     type: aws:pinpoint:App
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint APNs Sandbox Channel using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/apnsSandboxChannel:ApnsSandboxChannel apns_sandbox application-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod apns_sandbox_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApnsSandboxChannelArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
        #[builder(into, default)]
        pub bundle_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The pem encoded TLS Certificate from Apple.
        #[builder(into, default)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default authentication method used for APNs Sandbox.
        /// __NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
        /// You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
        /// If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.
        ///
        /// One of the following sets of credentials is also required.
        ///
        /// If you choose to use __Certificate credentials__ you will have to provide:
        #[builder(into, default)]
        pub default_authentication_method: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Certificate Private Key file (ie. `.key` file).
        ///
        /// If you choose to use __Key credentials__ you will have to provide:
        #[builder(into, default)]
        pub private_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID assigned to your Apple developer account team. This value is provided on the Membership page.
        #[builder(into, default)]
        pub team_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The `.p8` file that you download from your Apple developer account when you create an authentication key.
        #[builder(into, default)]
        pub token_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
        #[builder(into, default)]
        pub token_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApnsSandboxChannelResult {
        /// The application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
        pub bundle_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The pem encoded TLS Certificate from Apple.
        pub certificate: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default authentication method used for APNs Sandbox.
        /// __NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
        /// You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
        /// If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.
        ///
        /// One of the following sets of credentials is also required.
        ///
        /// If you choose to use __Certificate credentials__ you will have to provide:
        pub default_authentication_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Certificate Private Key file (ie. `.key` file).
        ///
        /// If you choose to use __Key credentials__ you will have to provide:
        pub private_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID assigned to your Apple developer account team. This value is provided on the Membership page.
        pub team_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The `.p8` file that you download from your Apple developer account when you create an authentication key.
        pub token_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
        pub token_key_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApnsSandboxChannelArgs,
    ) -> ApnsSandboxChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let bundle_id_binding = args.bundle_id.get_output(context);
        let certificate_binding = args.certificate.get_output(context);
        let default_authentication_method_binding = args
            .default_authentication_method
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let private_key_binding = args.private_key.get_output(context);
        let team_id_binding = args.team_id.get_output(context);
        let token_key_binding = args.token_key.get_output(context);
        let token_key_id_binding = args.token_key_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/apnsSandboxChannel:ApnsSandboxChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bundleId".into(),
                    value: bundle_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultAuthenticationMethod".into(),
                    value: default_authentication_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateKey".into(),
                    value: private_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "teamId".into(),
                    value: team_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenKey".into(),
                    value: token_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenKeyId".into(),
                    value: token_key_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApnsSandboxChannelResult {
            application_id: o.get_field("applicationId"),
            bundle_id: o.get_field("bundleId"),
            certificate: o.get_field("certificate"),
            default_authentication_method: o.get_field("defaultAuthenticationMethod"),
            enabled: o.get_field("enabled"),
            private_key: o.get_field("privateKey"),
            team_id: o.get_field("teamId"),
            token_key: o.get_field("tokenKey"),
            token_key_id: o.get_field("tokenKeyId"),
        }
    }
}
