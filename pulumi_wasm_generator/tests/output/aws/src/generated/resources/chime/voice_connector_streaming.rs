/// Adds a streaming configuration for the specified Amazon Chime Voice Connector. The streaming configuration specifies whether media streaming is enabled for sending to Amazon Kinesis.
/// It also sets the retention period, in hours, for the Amazon Kinesis data.
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
///     let defaultVoiceConnectorStreaming = voice_connector_streaming::create(
///         "defaultVoiceConnectorStreaming",
///         VoiceConnectorStreamingArgs::builder()
///             .data_retention(7)
///             .disabled(false)
///             .streaming_notification_targets(vec!["SQS",])
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example Usage With Media Insights
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
///                     .identifiers(vec!["mediapipelines.chime.amazonaws.com",]). type
///                     ("Service").build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let default = voice_connector::create(
///         "default",
///         VoiceConnectorArgs::builder()
///             .name("vc-name-test")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let defaultVoiceConnectorStreaming = voice_connector_streaming::create(
///         "defaultVoiceConnectorStreaming",
///         VoiceConnectorStreamingArgs::builder()
///             .data_retention(7)
///             .disabled(false)
///             .media_insights_configuration(
///                 VoiceConnectorStreamingMediaInsightsConfiguration::builder()
///                     .configurationArn("${example.arn}")
///                     .disabled(false)
///                     .build_struct(),
///             )
///             .streaming_notification_targets(vec!["SQS",])
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
///     let example = media_insights_pipeline_configuration::create(
///         "example",
///         MediaInsightsPipelineConfigurationArgs::builder()
///             .elements(
///                 vec![
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .amazonTranscribeCallAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration::builder()
///                     .languageCode("en-US").build_struct()). type
///                     ("AmazonTranscribeCallAnalyticsProcessor").build_struct(),
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration::builder()
///                     .insightsTarget("${exampleStream.arn}").build_struct()). type
///                     ("KinesisDataStreamSink").build_struct(),
///                 ],
///             )
///             .name("ExampleConfig")
///             .resource_access_role_arn("${exampleRole.arn}")
///             .build_struct(),
///     );
///     let exampleRole = role::create(
///         "exampleRole",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("ExampleResourceAccessRole")
///             .build_struct(),
///     );
///     let exampleStream = stream::create(
///         "exampleStream",
///         StreamArgs::builder().name("ExampleStream").shard_count(2).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chime Voice Connector Streaming using the `voice_connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorStreaming:VoiceConnectorStreaming default abcdef1ghij2klmno3pqr4
/// ```
pub mod voice_connector_streaming {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorStreamingArgs {
        /// The retention period, in hours, for the Amazon Kinesis data.
        #[builder(into)]
        pub data_retention: pulumi_wasm_rust::Output<i32>,
        /// When true, media streaming to Amazon Kinesis is turned off. Default: `false`
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The media insights configuration. See `media_insights_configuration`.
        #[builder(into, default)]
        pub media_insights_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::chime::VoiceConnectorStreamingMediaInsightsConfiguration,
            >,
        >,
        /// The streaming notification targets. Valid Values: `EventBridge | SNS | SQS`
        #[builder(into, default)]
        pub streaming_notification_targets: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorStreamingResult {
        /// The retention period, in hours, for the Amazon Kinesis data.
        pub data_retention: pulumi_wasm_rust::Output<i32>,
        /// When true, media streaming to Amazon Kinesis is turned off. Default: `false`
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The media insights configuration. See `media_insights_configuration`.
        pub media_insights_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::chime::VoiceConnectorStreamingMediaInsightsConfiguration,
            >,
        >,
        /// The streaming notification targets. Valid Values: `EventBridge | SNS | SQS`
        pub streaming_notification_targets: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VoiceConnectorStreamingArgs,
    ) -> VoiceConnectorStreamingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_retention_binding = args.data_retention.get_inner();
        let disabled_binding = args.disabled.get_inner();
        let media_insights_configuration_binding = args
            .media_insights_configuration
            .get_inner();
        let streaming_notification_targets_binding = args
            .streaming_notification_targets
            .get_inner();
        let voice_connector_id_binding = args.voice_connector_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorStreaming:VoiceConnectorStreaming".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataRetention".into(),
                    value: &data_retention_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "mediaInsightsConfiguration".into(),
                    value: &media_insights_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "streamingNotificationTargets".into(),
                    value: &streaming_notification_targets_binding,
                },
                register_interface::ObjectField {
                    name: "voiceConnectorId".into(),
                    value: &voice_connector_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataRetention".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "mediaInsightsConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "streamingNotificationTargets".into(),
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
        VoiceConnectorStreamingResult {
            data_retention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataRetention").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            media_insights_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mediaInsightsConfiguration").unwrap(),
            ),
            streaming_notification_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamingNotificationTargets").unwrap(),
            ),
            voice_connector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceConnectorId").unwrap(),
            ),
        }
    }
}