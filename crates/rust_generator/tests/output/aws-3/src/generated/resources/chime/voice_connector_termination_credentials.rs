/// Adds termination SIP credentials for the specified Amazon Chime Voice Connector.
///
/// > **Note:** Voice Connector Termination Credentials requires a Voice Connector Termination to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod voice_connector_termination_credentials {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationCredentialsArgs {
        /// List of termination SIP credentials.
        #[builder(into)]
        pub credentials: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::chime::VoiceConnectorTerminationCredentialsCredential,
            >,
        >,
        /// Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationCredentialsResult {
        /// List of termination SIP credentials.
        pub credentials: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::chime::VoiceConnectorTerminationCredentialsCredential,
            >,
        >,
        /// Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VoiceConnectorTerminationCredentialsArgs,
    ) -> VoiceConnectorTerminationCredentialsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let credentials_binding = args.credentials.get_output(context);
        let voice_connector_id_binding = args.voice_connector_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorTerminationCredentials:VoiceConnectorTerminationCredentials"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credentials".into(),
                    value: credentials_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceConnectorId".into(),
                    value: voice_connector_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VoiceConnectorTerminationCredentialsResult {
            credentials: o.get_field("credentials"),
            voice_connector_id: o.get_field("voiceConnectorId"),
        }
    }
}
