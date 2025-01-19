/// Adds termination SIP credentials for the specified Amazon Chime Voice Connector.
///
/// > **Note:** Voice Connector Termination Credentials requires a Voice Connector Termination to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = voice_connector::create(
///         "default",
///         VoiceConnectorArgs::builder()
///             .name("test")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let defaultVoiceConnectorTermination = voice_connector_termination::create(
///         "defaultVoiceConnectorTermination",
///         VoiceConnectorTerminationArgs::builder()
///             .calling_regions(vec!["US", "CA",])
///             .cidr_allow_lists(vec!["50.35.78.96/31",])
///             .cps_limit(1)
///             .disabled(true)
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
///     let defaultVoiceConnectorTerminationCredentials = voice_connector_termination_credentials::create(
///         "defaultVoiceConnectorTerminationCredentials",
///         VoiceConnectorTerminationCredentialsArgs::builder()
///             .credentials(
///                 vec![
///                     VoiceConnectorTerminationCredentialsCredential::builder()
///                     .password("test!").username("test").build_struct(),
///                 ],
///             )
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chime Voice Connector Termination Credentials using the `voice_connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorTerminationCredentials:VoiceConnectorTerminationCredentials default abcdef1ghij2klmno3pqr4
/// ```
pub mod voice_connector_termination_credentials {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationCredentialsArgs {
        /// List of termination SIP credentials.
        #[builder(into)]
        pub credentials: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::chime::VoiceConnectorTerminationCredentialsCredential,
            >,
        >,
        /// Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationCredentialsResult {
        /// List of termination SIP credentials.
        pub credentials: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::chime::VoiceConnectorTerminationCredentialsCredential,
            >,
        >,
        /// Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VoiceConnectorTerminationCredentialsArgs,
    ) -> VoiceConnectorTerminationCredentialsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let credentials_binding = args.credentials.get_inner();
        let voice_connector_id_binding = args.voice_connector_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorTerminationCredentials:VoiceConnectorTerminationCredentials"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "credentials".into(),
                    value: &credentials_binding,
                },
                register_interface::ObjectField {
                    name: "voiceConnectorId".into(),
                    value: &voice_connector_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "credentials".into(),
                },
                register_interface::ResultField {
                    name: "voiceConnectorId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VoiceConnectorTerminationCredentialsResult {
            credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentials").unwrap(),
            ),
            voice_connector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceConnectorId").unwrap(),
            ),
        }
    }
}
