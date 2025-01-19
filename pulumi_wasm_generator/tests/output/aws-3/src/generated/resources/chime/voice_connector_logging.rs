/// Adds a logging configuration for the specified Amazon Chime Voice Connector. The logging configuration specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.
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
pub mod voice_connector_logging {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorLoggingArgs {
        /// When true, enables logging of detailed media metrics for Voice Connectors to Amazon CloudWatch logs.
        #[builder(into, default)]
        pub enable_media_metric_logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// When true, enables SIP message logs for sending to Amazon CloudWatch Logs.
        #[builder(into, default)]
        pub enable_sip_logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorLoggingResult {
        /// When true, enables logging of detailed media metrics for Voice Connectors to Amazon CloudWatch logs.
        pub enable_media_metric_logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// When true, enables SIP message logs for sending to Amazon CloudWatch Logs.
        pub enable_sip_logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VoiceConnectorLoggingArgs,
    ) -> VoiceConnectorLoggingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enable_media_metric_logs_binding = args.enable_media_metric_logs.get_inner();
        let enable_sip_logs_binding = args.enable_sip_logs.get_inner();
        let voice_connector_id_binding = args.voice_connector_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorLogging:VoiceConnectorLogging".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enableMediaMetricLogs".into(),
                    value: &enable_media_metric_logs_binding,
                },
                register_interface::ObjectField {
                    name: "enableSipLogs".into(),
                    value: &enable_sip_logs_binding,
                },
                register_interface::ObjectField {
                    name: "voiceConnectorId".into(),
                    value: &voice_connector_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enableMediaMetricLogs".into(),
                },
                register_interface::ResultField {
                    name: "enableSipLogs".into(),
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
        VoiceConnectorLoggingResult {
            enable_media_metric_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableMediaMetricLogs").unwrap(),
            ),
            enable_sip_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableSipLogs").unwrap(),
            ),
            voice_connector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceConnectorId").unwrap(),
            ),
        }
    }
}
