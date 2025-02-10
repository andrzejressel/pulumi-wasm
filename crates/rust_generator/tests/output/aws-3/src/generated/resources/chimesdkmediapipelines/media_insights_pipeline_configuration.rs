/// Resource for managing an AWS Chime SDK Media Pipelines Media Insights Pipeline Configuration.
/// Consult the [Call analytics developer guide](https://docs.aws.amazon.com/chime-sdk/latest/dg/call-analytics.html) for more detailed information about usage.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   myConfiguration:
///     type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
///     name: my_configuration
///     properties:
///       name: MyBasicConfiguration
///       resourceAccessRoleArn: ${callAnalyticsRole.arn}
///       elements:
///         - type: AmazonTranscribeCallAnalyticsProcessor
///           amazonTranscribeCallAnalyticsProcessorConfiguration:
///             languageCode: en-US
///         - type: KinesisDataStreamSink
///           kinesisDataStreamSinkConfiguration:
///             insightsTarget: ${example.arn}
///       tags:
///         Key1: Value1
///         Key2: Value2
///   example:
///     type: aws:kinesis:Stream
///     properties:
///       name: example
///       shardCount: 2
///   callAnalyticsRole:
///     type: aws:iam:Role
///     name: call_analytics_role
///     properties:
///       name: CallAnalyticsRole
///       assumeRolePolicy: ${mediaPipelinesAssumeRole.json}
/// variables:
///   mediaPipelinesAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - mediapipelines.chime.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// - The required policies on `call_analytics_role` will vary based on the selected processors. See [Call analytics resource access role](https://docs.aws.amazon.com/chime-sdk/latest/dg/ca-resource-access-role.html) for directions on choosing appropriate policies.
///
/// ### Transcribe Call Analytics processor usage
///
/// ```yaml
/// resources:
///   myConfiguration:
///     type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
///     name: my_configuration
///     properties:
///       name: MyCallAnalyticsConfiguration
///       resourceAccessRoleArn: ${exampleAwsIamRole.arn}
///       elements:
///         - type: AmazonTranscribeCallAnalyticsProcessor
///           amazonTranscribeCallAnalyticsProcessorConfiguration:
///             callAnalyticsStreamCategories:
///               - category_1
///               - category_2
///             contentRedactionType: PII
///             enablePartialResultsStabilization: true
///             filterPartialResults: true
///             languageCode: en-US
///             languageModelName: MyLanguageModel
///             partialResultsStability: high
///             piiEntityTypes: ADDRESS,BANK_ACCOUNT_NUMBER
///             postCallAnalyticsSettings:
///               contentRedactionOutput: redacted
///               dataAccessRoleArn: ${postCallRole.arn}
///               outputEncryptionKmsKeyId: MyKmsKeyId
///               outputLocation: s3://MyBucket
///             vocabularyFilterMethod: mask
///             vocabularyFilterName: MyVocabularyFilter
///             vocabularyName: MyVocabulary
///         - type: KinesisDataStreamSink
///           kinesisDataStreamSinkConfiguration:
///             insightsTarget: ${example.arn}
///   postCallRole:
///     type: aws:iam:Role
///     name: post_call_role
///     properties:
///       name: PostCallAccessRole
///       assumeRolePolicy: ${transcribeAssumeRole.json}
/// variables:
///   transcribeAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - transcribe.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Real time alerts usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myConfiguration = media_insights_pipeline_configuration::create(
///         "myConfiguration",
///         MediaInsightsPipelineConfigurationArgs::builder()
///             .elements(
///                 vec![
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .amazonTranscribeCallAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration::builder()
///                     .languageCode("en-US").build_struct()). type
///                     ("AmazonTranscribeCallAnalyticsProcessor").build_struct(),
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration::builder()
///                     .insightsTarget("${example.arn}").build_struct()). type
///                     ("KinesisDataStreamSink").build_struct(),
///                 ],
///             )
///             .name("MyRealTimeAlertConfiguration")
///             .real_time_alert_configuration(
///                 MediaInsightsPipelineConfigurationRealTimeAlertConfiguration::builder()
///                     .disabled(false)
///                     .rules(
///                         vec![
///                             MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule::builder()
///                             .issueDetectionConfiguration(MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration::builder()
///                             .ruleName("MyIssueDetectionRule").build_struct()). type
///                             ("IssueDetection").build_struct(),
///                             MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule::builder()
///                             .keywordMatchConfiguration(MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration::builder()
///                             .keywords(vec!["keyword1", "keyword2",]).negate(false)
///                             .ruleName("MyKeywordMatchRule").build_struct()). type
///                             ("KeywordMatch").build_struct(),
///                             MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule::builder()
///                             .sentimentConfiguration(MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration::builder()
///                             .ruleName("MySentimentRule").sentimentType("NEGATIVE")
///                             .timePeriod(60).build_struct()). type ("Sentiment")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .resource_access_role_arn("${callAnalyticsRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Transcribe processor usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myConfiguration = media_insights_pipeline_configuration::create(
///         "myConfiguration",
///         MediaInsightsPipelineConfigurationArgs::builder()
///             .elements(
///                 vec![
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .amazonTranscribeProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration::builder()
///                     .contentIdentificationType("PII")
///                     .enablePartialResultsStabilization(true).filterPartialResults(true)
///                     .languageCode("en-US").languageModelName("MyLanguageModel")
///                     .partialResultsStability("high")
///                     .piiEntityTypes("ADDRESS,BANK_ACCOUNT_NUMBER").showSpeakerLabel(true)
///                     .vocabularyFilterMethod("mask")
///                     .vocabularyFilterName("MyVocabularyFilter")
///                     .vocabularyName("MyVocabulary").build_struct()). type
///                     ("AmazonTranscribeProcessor").build_struct(),
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration::builder()
///                     .insightsTarget("${example.arn}").build_struct()). type
///                     ("KinesisDataStreamSink").build_struct(),
///                 ],
///             )
///             .name("MyTranscribeConfiguration")
///             .resource_access_role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Voice analytics processor usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myConfiguration = media_insights_pipeline_configuration::create(
///         "myConfiguration",
///         MediaInsightsPipelineConfigurationArgs::builder()
///             .elements(
///                 vec![
///                     MediaInsightsPipelineConfigurationElement::builder(). type
///                     ("VoiceAnalyticsProcessor")
///                     .voiceAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration::builder()
///                     .speakerSearchStatus("Enabled").voiceToneAnalysisStatus("Enabled")
///                     .build_struct()).build_struct(),
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .lambdaFunctionSinkConfiguration(MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration::builder()
///                     .insightsTarget("arn:aws:lambda:us-west-2:1111111111:function:MyFunction")
///                     .build_struct()). type ("LambdaFunctionSink").build_struct(),
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .snsTopicSinkConfiguration(MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration::builder()
///                     .insightsTarget("arn:aws:sns:us-west-2:1111111111:topic/MyTopic")
///                     .build_struct()). type ("SnsTopicSink").build_struct(),
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .sqsQueueSinkConfiguration(MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration::builder()
///                     .insightsTarget("arn:aws:sqs:us-west-2:1111111111:queue/MyQueue")
///                     .build_struct()). type ("SqsQueueSink").build_struct(),
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration::builder()
///                     .insightsTarget("${test.arn}").build_struct()). type
///                     ("KinesisDataStreamSink").build_struct(),
///                 ],
///             )
///             .name("MyVoiceAnalyticsConfiguration")
///             .resource_access_role_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### S3 Recording sink usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myConfiguration = media_insights_pipeline_configuration::create(
///         "myConfiguration",
///         MediaInsightsPipelineConfigurationArgs::builder()
///             .elements(
///                 vec![
///                     MediaInsightsPipelineConfigurationElement::builder()
///                     .s3RecordingSinkConfiguration(MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration::builder()
///                     .destination("arn:aws:s3:::MyBucket").build_struct()). type
///                     ("S3RecordingSink").build_struct(),
///                 ],
///             )
///             .name("MyS3RecordingConfiguration")
///             .resource_access_role_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chime SDK Media Pipelines Media Insights Pipeline Configuration using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:chimesdkmediapipelines/mediaInsightsPipelineConfiguration:MediaInsightsPipelineConfiguration example abcdef123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod media_insights_pipeline_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MediaInsightsPipelineConfigurationArgs {
        /// Collection of processors and sinks to transform media and deliver data.
        #[builder(into)]
        pub elements: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElement,
            >,
        >,
        /// Configuration name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for real-time alert rules to send EventBridge notifications when certain conditions are met.
        #[builder(into, default)]
        pub real_time_alert_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfiguration,
            >,
        >,
        /// ARN of IAM Role used by service to invoke processors and sinks specified by configuration elements.
        #[builder(into)]
        pub resource_access_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MediaInsightsPipelineConfigurationResult {
        /// ARN of the Media Insights Pipeline Configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Collection of processors and sinks to transform media and deliver data.
        pub elements: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElement,
            >,
        >,
        /// Configuration name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for real-time alert rules to send EventBridge notifications when certain conditions are met.
        pub real_time_alert_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfiguration,
            >,
        >,
        /// ARN of IAM Role used by service to invoke processors and sinks specified by configuration elements.
        pub resource_access_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MediaInsightsPipelineConfigurationArgs,
    ) -> MediaInsightsPipelineConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let elements_binding = args.elements.get_output(context);
        let name_binding = args.name.get_output(context);
        let real_time_alert_configuration_binding = args
            .real_time_alert_configuration
            .get_output(context);
        let resource_access_role_arn_binding = args
            .resource_access_role_arn
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chimesdkmediapipelines/mediaInsightsPipelineConfiguration:MediaInsightsPipelineConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elements".into(),
                    value: elements_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "realTimeAlertConfiguration".into(),
                    value: real_time_alert_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceAccessRoleArn".into(),
                    value: resource_access_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MediaInsightsPipelineConfigurationResult {
            arn: o.get_field("arn"),
            elements: o.get_field("elements"),
            name: o.get_field("name"),
            real_time_alert_configuration: o.get_field("realTimeAlertConfiguration"),
            resource_access_role_arn: o.get_field("resourceAccessRoleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
