/// Provides a Pinpoint APNs VoIP Sandbox Channel resource.
///
/// > **Note:** All arguments, including certificates and tokens, will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```yaml
/// resources:
///   apnsVoipSandbox:
///     type: aws:pinpoint:ApnsVoipSandboxChannel
///     name: apns_voip_sandbox
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
/// Using `pulumi import`, import Pinpoint APNs VoIP Sandbox Channel using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/apnsVoipSandboxChannel:ApnsVoipSandboxChannel apns_voip_sandbox application-id
/// ```
pub mod apns_voip_sandbox_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApnsVoipSandboxChannelArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
        #[builder(into, default)]
        pub bundle_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The pem encoded TLS Certificate from Apple.
        #[builder(into, default)]
        pub certificate: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The default authentication method used for APNs.
        /// __NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
        /// You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
        /// If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.
        ///
        /// One of the following sets of credentials is also required.
        ///
        /// If you choose to use __Certificate credentials__ you will have to provide:
        #[builder(into, default)]
        pub default_authentication_method: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The Certificate Private Key file (ie. `.key` file).
        ///
        /// If you choose to use __Key credentials__ you will have to provide:
        #[builder(into, default)]
        pub private_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID assigned to your Apple developer account team. This value is provided on the Membership page.
        #[builder(into, default)]
        pub team_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The `.p8` file that you download from your Apple developer account when you create an authentication key.
        #[builder(into, default)]
        pub token_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
        #[builder(into, default)]
        pub token_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApnsVoipSandboxChannelResult {
        /// The application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// The ID assigned to your iOS app. To find this value, choose Certificates, IDs & Profiles, choose App IDs in the Identifiers section, and choose your app.
        pub bundle_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The pem encoded TLS Certificate from Apple.
        pub certificate: pulumi_wasm_rust::Output<Option<String>>,
        /// The default authentication method used for APNs.
        /// __NOTE__: Amazon Pinpoint uses this default for every APNs push notification that you send using the console.
        /// You can override the default when you send a message programmatically using the Amazon Pinpoint API, the AWS CLI, or an AWS SDK.
        /// If your default authentication type fails, Amazon Pinpoint doesn't attempt to use the other authentication type.
        ///
        /// One of the following sets of credentials is also required.
        ///
        /// If you choose to use __Certificate credentials__ you will have to provide:
        pub default_authentication_method: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Certificate Private Key file (ie. `.key` file).
        ///
        /// If you choose to use __Key credentials__ you will have to provide:
        pub private_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID assigned to your Apple developer account team. This value is provided on the Membership page.
        pub team_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The `.p8` file that you download from your Apple developer account when you create an authentication key.
        pub token_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID assigned to your signing key. To find this value, choose Certificates, IDs & Profiles, and choose your key in the Keys section.
        pub token_key_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ApnsVoipSandboxChannelArgs,
    ) -> ApnsVoipSandboxChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let bundle_id_binding = args.bundle_id.get_output(context).get_inner();
        let certificate_binding = args.certificate.get_output(context).get_inner();
        let default_authentication_method_binding = args
            .default_authentication_method
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let private_key_binding = args.private_key.get_output(context).get_inner();
        let team_id_binding = args.team_id.get_output(context).get_inner();
        let token_key_binding = args.token_key.get_output(context).get_inner();
        let token_key_id_binding = args.token_key_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/apnsVoipSandboxChannel:ApnsVoipSandboxChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAuthenticationMethod".into(),
                    value: &default_authentication_method_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
                },
                register_interface::ObjectField {
                    name: "teamId".into(),
                    value: &team_id_binding,
                },
                register_interface::ObjectField {
                    name: "tokenKey".into(),
                    value: &token_key_binding,
                },
                register_interface::ObjectField {
                    name: "tokenKeyId".into(),
                    value: &token_key_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "defaultAuthenticationMethod".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "privateKey".into(),
                },
                register_interface::ResultField {
                    name: "teamId".into(),
                },
                register_interface::ResultField {
                    name: "tokenKey".into(),
                },
                register_interface::ResultField {
                    name: "tokenKeyId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApnsVoipSandboxChannelResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            default_authentication_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAuthenticationMethod").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateKey").unwrap(),
            ),
            team_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("teamId").unwrap(),
            ),
            token_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenKey").unwrap(),
            ),
            token_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenKeyId").unwrap(),
            ),
        }
    }
}
