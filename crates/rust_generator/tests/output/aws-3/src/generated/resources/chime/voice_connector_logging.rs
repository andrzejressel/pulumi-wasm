/// Adds a logging configuration for the specified Amazon Chime Voice Connector. The logging configuration specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.
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
///             .name("vc-name-test")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let defaultVoiceConnectorLogging = voice_connector_logging::create(
///         "defaultVoiceConnectorLogging",
///         VoiceConnectorLoggingArgs::builder()
///             .enable_media_metric_logs(true)
///             .enable_sip_logs(true)
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chime Voice Connector Logging using the `voice_connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorLogging:VoiceConnectorLogging default abcdef1ghij2klmno3pqr4
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod voice_connector_logging {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorLoggingArgs {
        /// When true, enables logging of detailed media metrics for Voice Connectors to Amazon CloudWatch logs.
        #[builder(into, default)]
        pub enable_media_metric_logs: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When true, enables SIP message logs for sending to Amazon CloudWatch Logs.
        #[builder(into, default)]
        pub enable_sip_logs: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorLoggingResult {
        /// When true, enables logging of detailed media metrics for Voice Connectors to Amazon CloudWatch logs.
        pub enable_media_metric_logs: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When true, enables SIP message logs for sending to Amazon CloudWatch Logs.
        pub enable_sip_logs: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VoiceConnectorLoggingArgs,
    ) -> VoiceConnectorLoggingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enable_media_metric_logs_binding = args
            .enable_media_metric_logs
            .get_output(context);
        let enable_sip_logs_binding = args.enable_sip_logs.get_output(context);
        let voice_connector_id_binding = args.voice_connector_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorLogging:VoiceConnectorLogging".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableMediaMetricLogs".into(),
                    value: enable_media_metric_logs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableSipLogs".into(),
                    value: enable_sip_logs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceConnectorId".into(),
                    value: voice_connector_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VoiceConnectorLoggingResult {
            enable_media_metric_logs: o.get_field("enableMediaMetricLogs"),
            enable_sip_logs: o.get_field("enableSipLogs"),
            voice_connector_id: o.get_field("voiceConnectorId"),
        }
    }
}
