/// Resource for managing an AWS Rekognition Stream Processor.
///
/// > This resource must be configured specifically for your use case, and not all options are compatible with one another. See [Stream Processor API documentation](https://docs.aws.amazon.com/rekognition/latest/APIReference/API_CreateStreamProcessor.html#rekognition-CreateStreamProcessor-request-Input) for configuration information.
///
/// > Stream Processors configured for Face Recognition cannot have _any_ properties updated after the fact, and it will result in an AWS API error.
///
/// ## Example Usage
///
/// ### Label Detection
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example-bucket
///   exampleTopic:
///     type: aws:sns:Topic
///     name: example
///     properties:
///       name: example-topic
///   exampleVideoStream:
///     type: aws:kinesis:VideoStream
///     name: example
///     properties:
///       name: example-kinesis-input
///       dataRetentionInHours: 1
///       deviceName: kinesis-video-device-name
///       mediaType: video/h264
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       inlinePolicies:
///         - name: Rekognition-Access
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - s3:PutObject
///                   Effect: Allow
///                   Resource:
///                     - ${example.arn}/*
///                 - Action:
///                     - sns:Publish
///                   Effect: Allow
///                   Resource:
///                     - ${exampleTopic.arn}
///                 - Action:
///                     - kinesis:Get*
///                     - kinesis:DescribeStreamSummary
///                   Effect: Allow
///                   Resource:
///                     - ${exampleVideoStream.arn}
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: rekognition.amazonaws.com
///   exampleStreamProcessor:
///     type: aws:rekognition:StreamProcessor
///     name: example
///     properties:
///       roleArn: ${exampleRole.arn}
///       name: example-processor
///       dataSharingPreference:
///         optIn: false
///       output:
///         s3Destination:
///           bucket: ${example.bucket}
///       settings:
///         connectedHome:
///           labels:
///             - PERSON
///             - PET
///       input:
///         kinesisVideoStream:
///           arn: ${exampleVideoStream.arn}
///       notificationChannel:
///         snsTopicArn: ${exampleTopic.arn}
/// ```
///
/// ### Face Detection Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kinesis:VideoStream
///     properties:
///       name: example-kinesis-input
///       dataRetentionInHours: 1
///       deviceName: kinesis-video-device-name
///       mediaType: video/h264
///   exampleStream:
///     type: aws:kinesis:Stream
///     name: example
///     properties:
///       name: pulumi-kinesis-example
///       shardCount: 1
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       inlinePolicies:
///         - name: Rekognition-Access
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - kinesis:Get*
///                     - kinesis:DescribeStreamSummary
///                   Effect: Allow
///                   Resource:
///                     - ${example.arn}
///                 - Action:
///                     - kinesis:PutRecord
///                   Effect: Allow
///                   Resource:
///                     - ${exampleStream.arn}
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: rekognition.amazonaws.com
///   exampleCollection:
///     type: aws:rekognition:Collection
///     name: example
///     properties:
///       collectionId: example-collection
///   exampleStreamProcessor:
///     type: aws:rekognition:StreamProcessor
///     name: example
///     properties:
///       roleArn: ${exampleRole.arn}
///       name: example-processor
///       dataSharingPreference:
///         optIn: false
///       regionsOfInterests:
///         - polygons:
///             - x: 0.5
///               y: 0.5
///             - x: 0.5
///               y: 0.5
///             - x: 0.5
///               y: 0.5
///       input:
///         kinesisVideoStream:
///           arn: ${example.arn}
///       output:
///         kinesisDataStream:
///           arn: ${exampleStream.arn}
///       settings:
///         faceSearch:
///           collectionId: ${exampleCollection.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Rekognition Stream Processor using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:rekognition/streamProcessor:StreamProcessor example my-stream
/// ```
pub mod stream_processor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamProcessorArgs {
        /// See `data_sharing_preference`.
        #[builder(into, default)]
        pub data_sharing_preference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::rekognition::StreamProcessorDataSharingPreference,
            >,
        >,
        /// Input video stream. See `input`.
        #[builder(into, default)]
        pub input: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorInput>,
        >,
        /// Optional parameter for label detection stream processors.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Processor.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the completion status. See `notification_channel`.
        #[builder(into, default)]
        pub notification_channel: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorNotificationChannel>,
        >,
        /// Kinesis data stream stream or Amazon S3 bucket location to which Amazon Rekognition Video puts the analysis results. See `output`.
        #[builder(into, default)]
        pub output: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorOutput>,
        >,
        /// Specifies locations in the frames where Amazon Rekognition checks for objects or people. See `regions_of_interest`.
        #[builder(into, default)]
        pub regions_of_interests: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::rekognition::StreamProcessorRegionsOfInterest>,
            >,
        >,
        /// The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor. The IAM role provides Rekognition read permissions for a Kinesis stream. It also provides write permissions to an Amazon S3 bucket and Amazon Simple Notification Service topic for a label detection stream processor. This is required for both face search and label detection stream processors.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Input parameters used in a streaming video analyzed by a stream processor. See `settings`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorSettings>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct StreamProcessorResult {
        /// See `data_sharing_preference`.
        pub data_sharing_preference: pulumi_wasm_rust::Output<
            Option<
                super::super::types::rekognition::StreamProcessorDataSharingPreference,
            >,
        >,
        /// Input video stream. See `input`.
        pub input: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorInput>,
        >,
        /// Optional parameter for label detection stream processors.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Processor.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the completion status. See `notification_channel`.
        pub notification_channel: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorNotificationChannel>,
        >,
        /// Kinesis data stream stream or Amazon S3 bucket location to which Amazon Rekognition Video puts the analysis results. See `output`.
        pub output: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorOutput>,
        >,
        /// Specifies locations in the frames where Amazon Rekognition checks for objects or people. See `regions_of_interest`.
        pub regions_of_interests: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::rekognition::StreamProcessorRegionsOfInterest>,
            >,
        >,
        /// The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor. The IAM role provides Rekognition read permissions for a Kinesis stream. It also provides write permissions to an Amazon S3 bucket and Amazon Simple Notification Service topic for a label detection stream processor. This is required for both face search and label detection stream processors.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Input parameters used in a streaming video analyzed by a stream processor. See `settings`.
        ///
        /// The following arguments are optional:
        pub settings: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorSettings>,
        >,
        /// ARN of the Stream Processor.
        pub stream_processor_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StreamProcessorArgs) -> StreamProcessorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_sharing_preference_binding = args.data_sharing_preference.get_inner();
        let input_binding = args.input.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let name_binding = args.name.get_inner();
        let notification_channel_binding = args.notification_channel.get_inner();
        let output_binding = args.output.get_inner();
        let regions_of_interests_binding = args.regions_of_interests.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let settings_binding = args.settings.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rekognition/streamProcessor:StreamProcessor".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataSharingPreference".into(),
                    value: &data_sharing_preference_binding,
                },
                register_interface::ObjectField {
                    name: "input".into(),
                    value: &input_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationChannel".into(),
                    value: &notification_channel_binding,
                },
                register_interface::ObjectField {
                    name: "output".into(),
                    value: &output_binding,
                },
                register_interface::ObjectField {
                    name: "regionsOfInterests".into(),
                    value: &regions_of_interests_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataSharingPreference".into(),
                },
                register_interface::ResultField {
                    name: "input".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationChannel".into(),
                },
                register_interface::ResultField {
                    name: "output".into(),
                },
                register_interface::ResultField {
                    name: "regionsOfInterests".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
                register_interface::ResultField {
                    name: "streamProcessorArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StreamProcessorResult {
            data_sharing_preference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSharingPreference").unwrap(),
            ),
            input: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("input").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_channel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationChannel").unwrap(),
            ),
            output: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("output").unwrap(),
            ),
            regions_of_interests: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionsOfInterests").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
            stream_processor_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamProcessorArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
