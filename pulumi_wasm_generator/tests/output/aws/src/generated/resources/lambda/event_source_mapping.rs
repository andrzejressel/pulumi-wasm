/// Provides a Lambda event source mapping. This allows Lambda functions to get events from Kinesis, DynamoDB, SQS, Amazon MQ and Managed Streaming for Apache Kafka (MSK).
///
/// For information about Lambda and how to use it, see [What is AWS Lambda?](http://docs.aws.amazon.com/lambda/latest/dg/welcome.html).
/// For information about event source mappings, see [CreateEventSourceMapping](http://docs.aws.amazon.com/lambda/latest/dg/API_CreateEventSourceMapping.html) in the API docs.
///
/// ## Example Usage
///
/// ### DynamoDB
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lambda:EventSourceMapping
///     properties:
///       eventSourceArn: ${exampleAwsDynamodbTable.streamArn}
///       functionName: ${exampleAwsLambdaFunction.arn}
///       startingPosition: LATEST
///       tags:
///         Name: dynamodb
/// ```
///
/// ### Kinesis
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = event_source_mapping::create(
///         "example",
///         EventSourceMappingArgs::builder()
///             .event_source_arn("${exampleAwsKinesisStream.arn}")
///             .function_name("${exampleAwsLambdaFunction.arn}")
///             .starting_position("LATEST")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Managed Streaming for Apache Kafka (MSK)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = event_source_mapping::create(
///         "example",
///         EventSourceMappingArgs::builder()
///             .event_source_arn("${exampleAwsMskCluster.arn}")
///             .function_name("${exampleAwsLambdaFunction.arn}")
///             .starting_position("TRIM_HORIZON")
///             .topics(vec!["Example",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Self Managed Apache Kafka
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lambda:EventSourceMapping
///     properties:
///       functionName: ${exampleAwsLambdaFunction.arn}
///       topics:
///         - Example
///       startingPosition: TRIM_HORIZON
///       selfManagedEventSource:
///         endpoints:
///           KAFKA_BOOTSTRAP_SERVERS: kafka1.example.com:9092,kafka2.example.com:9092
///       sourceAccessConfigurations:
///         - type: VPC_SUBNET
///           uri: subnet:subnet-example1
///         - type: VPC_SUBNET
///           uri: subnet:subnet-example2
///         - type: VPC_SECURITY_GROUP
///           uri: security_group:sg-example
/// ```
///
/// ### SQS
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = event_source_mapping::create(
///         "example",
///         EventSourceMappingArgs::builder()
///             .event_source_arn("${sqsQueueTest.arn}")
///             .function_name("${exampleAwsLambdaFunction.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### SQS with event filter
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lambda:EventSourceMapping
///     properties:
///       eventSourceArn: ${sqsQueueTest.arn}
///       functionName: ${exampleAwsLambdaFunction.arn}
///       filterCriteria:
///         filters:
///           - pattern:
///               fn::toJSON:
///                 body:
///                   Temperature:
///                     - numeric:
///                         - '>'
///                         - 0
///                         - <=
///                         - 100
///                   Location:
///                     - New York
/// ```
///
/// ### Amazon MQ (ActiveMQ)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = event_source_mapping::create(
///         "example",
///         EventSourceMappingArgs::builder()
///             .batch_size(10)
///             .enabled(true)
///             .event_source_arn("${exampleAwsMqBroker.arn}")
///             .function_name("${exampleAwsLambdaFunction.arn}")
///             .queues("example")
///             .source_access_configurations(
///                 vec![
///                     EventSourceMappingSourceAccessConfiguration::builder(). type
///                     ("BASIC_AUTH").uri("${exampleAwsSecretsmanagerSecretVersion.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Amazon MQ (RabbitMQ)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = event_source_mapping::create(
///         "example",
///         EventSourceMappingArgs::builder()
///             .batch_size(1)
///             .enabled(true)
///             .event_source_arn("${exampleAwsMqBroker.arn}")
///             .function_name("${exampleAwsLambdaFunction.arn}")
///             .queues("example")
///             .source_access_configurations(
///                 vec![
///                     EventSourceMappingSourceAccessConfiguration::builder(). type
///                     ("VIRTUAL_HOST").uri("/example").build_struct(),
///                     EventSourceMappingSourceAccessConfiguration::builder(). type
///                     ("BASIC_AUTH").uri("${exampleAwsSecretsmanagerSecretVersion.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lambda event source mappings using the `UUID` (event source mapping identifier). For example:
///
/// ```sh
/// $ pulumi import aws:lambda/eventSourceMapping:EventSourceMapping event_source_mapping 12345kxodurf3443
/// ```
pub mod event_source_mapping {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSourceMappingArgs {
        /// Additional configuration block for Amazon Managed Kafka sources. Incompatible with "self_managed_event_source" and "self_managed_kafka_event_source_config". Detailed below.
        #[builder(into, default)]
        pub amazon_managed_kafka_event_source_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::lambda::EventSourceMappingAmazonManagedKafkaEventSourceConfig,
            >,
        >,
        /// The largest number of records that Lambda will retrieve from your event source at the time of invocation. Defaults to `100` for DynamoDB, Kinesis, MQ and MSK, `10` for SQS.
        #[builder(into, default)]
        pub batch_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// - (Optional) If the function returns an error, split the batch in two and retry. Only available for stream sources (DynamoDB and Kinesis). Defaults to `false`.
        #[builder(into, default)]
        pub bisect_batch_on_function_error: pulumi_wasm_rust::Output<Option<bool>>,
        /// - (Optional) An Amazon SQS queue, Amazon SNS topic or Amazon S3 bucket (only available for Kafka sources) destination for failed records. Only available for stream sources (DynamoDB and Kinesis) and Kafka sources (Amazon MSK and Self-managed Apache Kafka). Detailed below.
        #[builder(into, default)]
        pub destination_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingDestinationConfig>,
        >,
        /// - (Optional) Configuration settings for a DocumentDB event source. Detailed below.
        #[builder(into, default)]
        pub document_db_event_source_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::lambda::EventSourceMappingDocumentDbEventSourceConfig,
            >,
        >,
        /// Determines if the mapping will be enabled on creation. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The event source ARN - this is required for Kinesis stream, DynamoDB stream, SQS queue, MQ broker, MSK cluster or DocumentDB change stream.  It is incompatible with a Self Managed Kafka source.
        #[builder(into, default)]
        pub event_source_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The criteria to use for [event filtering](https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html) Kinesis stream, DynamoDB stream, SQS queue event sources. Detailed below.
        #[builder(into, default)]
        pub filter_criteria: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingFilterCriteria>,
        >,
        /// The name or the ARN of the Lambda function that will be subscribing to events.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// A list of current response type enums applied to the event source mapping for [AWS Lambda checkpointing](https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html#services-ddb-batchfailurereporting). Only available for SQS and stream sources (DynamoDB and Kinesis). Valid values: `ReportBatchItemFailures`.
        #[builder(into, default)]
        pub function_response_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ARN of the Key Management Service (KMS) customer managed key that Lambda uses to encrypt your function's filter criteria.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum amount of time to gather records before invoking the function, in seconds (between 0 and 300). Records will continue to buffer (or accumulate in the case of an SQS queue event source) until either `maximum_batching_window_in_seconds` expires or `batch_size` has been met. For streaming event sources, defaults to as soon as records are available in the stream. If the batch it reads from the stream/queue only has one record in it, Lambda only sends one record to the function. Only available for stream sources (DynamoDB and Kinesis) and SQS standard queues.
        #[builder(into, default)]
        pub maximum_batching_window_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// - (Optional) The maximum age of a record that Lambda sends to a function for processing. Only available for stream sources (DynamoDB and Kinesis). Must be either -1 (forever, and the default value) or between 60 and 604800 (inclusive).
        #[builder(into, default)]
        pub maximum_record_age_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// - (Optional) The maximum number of times to retry when the function returns an error. Only available for stream sources (DynamoDB and Kinesis). Minimum and default of -1 (forever), maximum of 10000.
        #[builder(into, default)]
        pub maximum_retry_attempts: pulumi_wasm_rust::Output<Option<i32>>,
        /// - (Optional) The number of batches to process from each shard concurrently. Only available for stream sources (DynamoDB and Kinesis). Minimum and default of 1, maximum of 10.
        #[builder(into, default)]
        pub parallelization_factor: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Amazon MQ broker destination queue to consume. Only available for MQ sources. The list must contain exactly one queue name.
        #[builder(into, default)]
        pub queues: pulumi_wasm_rust::Output<Option<String>>,
        /// Scaling configuration of the event source. Only available for SQS queues. Detailed below.
        #[builder(into, default)]
        pub scaling_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingScalingConfig>,
        >,
        /// - (Optional) For Self Managed Kafka sources, the location of the self managed cluster. If set, configuration must also include `source_access_configuration`. Detailed below.
        #[builder(into, default)]
        pub self_managed_event_source: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingSelfManagedEventSource>,
        >,
        /// Additional configuration block for Self Managed Kafka sources. Incompatible with "event_source_arn" and "amazon_managed_kafka_event_source_config". Detailed below.
        #[builder(into, default)]
        pub self_managed_kafka_event_source_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::lambda::EventSourceMappingSelfManagedKafkaEventSourceConfig,
            >,
        >,
        /// For Self Managed Kafka sources, the access configuration for the source. If set, configuration must also include `self_managed_event_source`. Detailed below.
        #[builder(into, default)]
        pub source_access_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::lambda::EventSourceMappingSourceAccessConfiguration,
                >,
            >,
        >,
        /// The position in the stream where AWS Lambda should start reading. Must be one of `AT_TIMESTAMP` (Kinesis only), `LATEST` or `TRIM_HORIZON` if getting events from Kinesis, DynamoDB, MSK or Self Managed Apache Kafka. Must not be provided if getting events from SQS. More information about these positions can be found in the [AWS DynamoDB Streams API Reference](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_streams_GetShardIterator.html) and [AWS Kinesis API Reference](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetShardIterator.html#Kinesis-GetShardIterator-request-ShardIteratorType).
        #[builder(into, default)]
        pub starting_position: pulumi_wasm_rust::Output<Option<String>>,
        /// A timestamp in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) of the data record which to start reading when using `starting_position` set to `AT_TIMESTAMP`. If a record with this exact timestamp does not exist, the next later record is chosen. If the timestamp is older than the current trim horizon, the oldest available record is chosen.
        #[builder(into, default)]
        pub starting_position_timestamp: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Kafka topics. Only available for MSK sources. A single topic name must be specified.
        #[builder(into, default)]
        pub topics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The duration in seconds of a processing window for [AWS Lambda streaming analytics](https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html#services-kinesis-windows). The range is between 1 second up to 900 seconds. Only available for stream sources (DynamoDB and Kinesis).
        #[builder(into, default)]
        pub tumbling_window_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct EventSourceMappingResult {
        /// Additional configuration block for Amazon Managed Kafka sources. Incompatible with "self_managed_event_source" and "self_managed_kafka_event_source_config". Detailed below.
        pub amazon_managed_kafka_event_source_config: pulumi_wasm_rust::Output<
            super::super::types::lambda::EventSourceMappingAmazonManagedKafkaEventSourceConfig,
        >,
        /// The event source mapping ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The largest number of records that Lambda will retrieve from your event source at the time of invocation. Defaults to `100` for DynamoDB, Kinesis, MQ and MSK, `10` for SQS.
        pub batch_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// - (Optional) If the function returns an error, split the batch in two and retry. Only available for stream sources (DynamoDB and Kinesis). Defaults to `false`.
        pub bisect_batch_on_function_error: pulumi_wasm_rust::Output<Option<bool>>,
        /// - (Optional) An Amazon SQS queue, Amazon SNS topic or Amazon S3 bucket (only available for Kafka sources) destination for failed records. Only available for stream sources (DynamoDB and Kinesis) and Kafka sources (Amazon MSK and Self-managed Apache Kafka). Detailed below.
        pub destination_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingDestinationConfig>,
        >,
        /// - (Optional) Configuration settings for a DocumentDB event source. Detailed below.
        pub document_db_event_source_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::lambda::EventSourceMappingDocumentDbEventSourceConfig,
            >,
        >,
        /// Determines if the mapping will be enabled on creation. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The event source ARN - this is required for Kinesis stream, DynamoDB stream, SQS queue, MQ broker, MSK cluster or DocumentDB change stream.  It is incompatible with a Self Managed Kafka source.
        pub event_source_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The criteria to use for [event filtering](https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html) Kinesis stream, DynamoDB stream, SQS queue event sources. Detailed below.
        pub filter_criteria: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingFilterCriteria>,
        >,
        /// The ARN of the Lambda function the event source mapping is sending events to. (Note: this is a computed value that differs from `function_name` above.)
        pub function_arn: pulumi_wasm_rust::Output<String>,
        /// The name or the ARN of the Lambda function that will be subscribing to events.
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// A list of current response type enums applied to the event source mapping for [AWS Lambda checkpointing](https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html#services-ddb-batchfailurereporting). Only available for SQS and stream sources (DynamoDB and Kinesis). Valid values: `ReportBatchItemFailures`.
        pub function_response_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ARN of the Key Management Service (KMS) customer managed key that Lambda uses to encrypt your function's filter criteria.
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The date this resource was last modified.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// The result of the last AWS Lambda invocation of your Lambda function.
        pub last_processing_result: pulumi_wasm_rust::Output<String>,
        /// The maximum amount of time to gather records before invoking the function, in seconds (between 0 and 300). Records will continue to buffer (or accumulate in the case of an SQS queue event source) until either `maximum_batching_window_in_seconds` expires or `batch_size` has been met. For streaming event sources, defaults to as soon as records are available in the stream. If the batch it reads from the stream/queue only has one record in it, Lambda only sends one record to the function. Only available for stream sources (DynamoDB and Kinesis) and SQS standard queues.
        pub maximum_batching_window_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// - (Optional) The maximum age of a record that Lambda sends to a function for processing. Only available for stream sources (DynamoDB and Kinesis). Must be either -1 (forever, and the default value) or between 60 and 604800 (inclusive).
        pub maximum_record_age_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// - (Optional) The maximum number of times to retry when the function returns an error. Only available for stream sources (DynamoDB and Kinesis). Minimum and default of -1 (forever), maximum of 10000.
        pub maximum_retry_attempts: pulumi_wasm_rust::Output<i32>,
        /// - (Optional) The number of batches to process from each shard concurrently. Only available for stream sources (DynamoDB and Kinesis). Minimum and default of 1, maximum of 10.
        pub parallelization_factor: pulumi_wasm_rust::Output<i32>,
        /// The name of the Amazon MQ broker destination queue to consume. Only available for MQ sources. The list must contain exactly one queue name.
        pub queues: pulumi_wasm_rust::Output<Option<String>>,
        /// Scaling configuration of the event source. Only available for SQS queues. Detailed below.
        pub scaling_config: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingScalingConfig>,
        >,
        /// - (Optional) For Self Managed Kafka sources, the location of the self managed cluster. If set, configuration must also include `source_access_configuration`. Detailed below.
        pub self_managed_event_source: pulumi_wasm_rust::Output<
            Option<super::super::types::lambda::EventSourceMappingSelfManagedEventSource>,
        >,
        /// Additional configuration block for Self Managed Kafka sources. Incompatible with "event_source_arn" and "amazon_managed_kafka_event_source_config". Detailed below.
        pub self_managed_kafka_event_source_config: pulumi_wasm_rust::Output<
            super::super::types::lambda::EventSourceMappingSelfManagedKafkaEventSourceConfig,
        >,
        /// For Self Managed Kafka sources, the access configuration for the source. If set, configuration must also include `self_managed_event_source`. Detailed below.
        pub source_access_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::lambda::EventSourceMappingSourceAccessConfiguration,
                >,
            >,
        >,
        /// The position in the stream where AWS Lambda should start reading. Must be one of `AT_TIMESTAMP` (Kinesis only), `LATEST` or `TRIM_HORIZON` if getting events from Kinesis, DynamoDB, MSK or Self Managed Apache Kafka. Must not be provided if getting events from SQS. More information about these positions can be found in the [AWS DynamoDB Streams API Reference](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_streams_GetShardIterator.html) and [AWS Kinesis API Reference](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetShardIterator.html#Kinesis-GetShardIterator-request-ShardIteratorType).
        pub starting_position: pulumi_wasm_rust::Output<Option<String>>,
        /// A timestamp in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) of the data record which to start reading when using `starting_position` set to `AT_TIMESTAMP`. If a record with this exact timestamp does not exist, the next later record is chosen. If the timestamp is older than the current trim horizon, the oldest available record is chosen.
        pub starting_position_timestamp: pulumi_wasm_rust::Output<Option<String>>,
        /// The state of the event source mapping.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The reason the event source mapping is in its current state.
        pub state_transition_reason: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Kafka topics. Only available for MSK sources. A single topic name must be specified.
        pub topics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The duration in seconds of a processing window for [AWS Lambda streaming analytics](https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html#services-kinesis-windows). The range is between 1 second up to 900 seconds. Only available for stream sources (DynamoDB and Kinesis).
        pub tumbling_window_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The UUID of the created event source mapping.
        pub uuid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventSourceMappingArgs) -> EventSourceMappingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let amazon_managed_kafka_event_source_config_binding = args
            .amazon_managed_kafka_event_source_config
            .get_inner();
        let batch_size_binding = args.batch_size.get_inner();
        let bisect_batch_on_function_error_binding = args
            .bisect_batch_on_function_error
            .get_inner();
        let destination_config_binding = args.destination_config.get_inner();
        let document_db_event_source_config_binding = args
            .document_db_event_source_config
            .get_inner();
        let enabled_binding = args.enabled.get_inner();
        let event_source_arn_binding = args.event_source_arn.get_inner();
        let filter_criteria_binding = args.filter_criteria.get_inner();
        let function_name_binding = args.function_name.get_inner();
        let function_response_types_binding = args.function_response_types.get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_inner();
        let maximum_batching_window_in_seconds_binding = args
            .maximum_batching_window_in_seconds
            .get_inner();
        let maximum_record_age_in_seconds_binding = args
            .maximum_record_age_in_seconds
            .get_inner();
        let maximum_retry_attempts_binding = args.maximum_retry_attempts.get_inner();
        let parallelization_factor_binding = args.parallelization_factor.get_inner();
        let queues_binding = args.queues.get_inner();
        let scaling_config_binding = args.scaling_config.get_inner();
        let self_managed_event_source_binding = args
            .self_managed_event_source
            .get_inner();
        let self_managed_kafka_event_source_config_binding = args
            .self_managed_kafka_event_source_config
            .get_inner();
        let source_access_configurations_binding = args
            .source_access_configurations
            .get_inner();
        let starting_position_binding = args.starting_position.get_inner();
        let starting_position_timestamp_binding = args
            .starting_position_timestamp
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let topics_binding = args.topics.get_inner();
        let tumbling_window_in_seconds_binding = args
            .tumbling_window_in_seconds
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/eventSourceMapping:EventSourceMapping".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amazonManagedKafkaEventSourceConfig".into(),
                    value: &amazon_managed_kafka_event_source_config_binding,
                },
                register_interface::ObjectField {
                    name: "batchSize".into(),
                    value: &batch_size_binding,
                },
                register_interface::ObjectField {
                    name: "bisectBatchOnFunctionError".into(),
                    value: &bisect_batch_on_function_error_binding,
                },
                register_interface::ObjectField {
                    name: "destinationConfig".into(),
                    value: &destination_config_binding,
                },
                register_interface::ObjectField {
                    name: "documentDbEventSourceConfig".into(),
                    value: &document_db_event_source_config_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "eventSourceArn".into(),
                    value: &event_source_arn_binding,
                },
                register_interface::ObjectField {
                    name: "filterCriteria".into(),
                    value: &filter_criteria_binding,
                },
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "functionResponseTypes".into(),
                    value: &function_response_types_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "maximumBatchingWindowInSeconds".into(),
                    value: &maximum_batching_window_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "maximumRecordAgeInSeconds".into(),
                    value: &maximum_record_age_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "maximumRetryAttempts".into(),
                    value: &maximum_retry_attempts_binding,
                },
                register_interface::ObjectField {
                    name: "parallelizationFactor".into(),
                    value: &parallelization_factor_binding,
                },
                register_interface::ObjectField {
                    name: "queues".into(),
                    value: &queues_binding,
                },
                register_interface::ObjectField {
                    name: "scalingConfig".into(),
                    value: &scaling_config_binding,
                },
                register_interface::ObjectField {
                    name: "selfManagedEventSource".into(),
                    value: &self_managed_event_source_binding,
                },
                register_interface::ObjectField {
                    name: "selfManagedKafkaEventSourceConfig".into(),
                    value: &self_managed_kafka_event_source_config_binding,
                },
                register_interface::ObjectField {
                    name: "sourceAccessConfigurations".into(),
                    value: &source_access_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "startingPosition".into(),
                    value: &starting_position_binding,
                },
                register_interface::ObjectField {
                    name: "startingPositionTimestamp".into(),
                    value: &starting_position_timestamp_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "topics".into(),
                    value: &topics_binding,
                },
                register_interface::ObjectField {
                    name: "tumblingWindowInSeconds".into(),
                    value: &tumbling_window_in_seconds_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "amazonManagedKafkaEventSourceConfig".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "batchSize".into(),
                },
                register_interface::ResultField {
                    name: "bisectBatchOnFunctionError".into(),
                },
                register_interface::ResultField {
                    name: "destinationConfig".into(),
                },
                register_interface::ResultField {
                    name: "documentDbEventSourceConfig".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "eventSourceArn".into(),
                },
                register_interface::ResultField {
                    name: "filterCriteria".into(),
                },
                register_interface::ResultField {
                    name: "functionArn".into(),
                },
                register_interface::ResultField {
                    name: "functionName".into(),
                },
                register_interface::ResultField {
                    name: "functionResponseTypes".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "lastProcessingResult".into(),
                },
                register_interface::ResultField {
                    name: "maximumBatchingWindowInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "maximumRecordAgeInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "maximumRetryAttempts".into(),
                },
                register_interface::ResultField {
                    name: "parallelizationFactor".into(),
                },
                register_interface::ResultField {
                    name: "queues".into(),
                },
                register_interface::ResultField {
                    name: "scalingConfig".into(),
                },
                register_interface::ResultField {
                    name: "selfManagedEventSource".into(),
                },
                register_interface::ResultField {
                    name: "selfManagedKafkaEventSourceConfig".into(),
                },
                register_interface::ResultField {
                    name: "sourceAccessConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "startingPosition".into(),
                },
                register_interface::ResultField {
                    name: "startingPositionTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateTransitionReason".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "topics".into(),
                },
                register_interface::ResultField {
                    name: "tumblingWindowInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "uuid".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventSourceMappingResult {
            amazon_managed_kafka_event_source_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amazonManagedKafkaEventSourceConfig").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            batch_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchSize").unwrap(),
            ),
            bisect_batch_on_function_error: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bisectBatchOnFunctionError").unwrap(),
            ),
            destination_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationConfig").unwrap(),
            ),
            document_db_event_source_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentDbEventSourceConfig").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            event_source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventSourceArn").unwrap(),
            ),
            filter_criteria: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filterCriteria").unwrap(),
            ),
            function_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionArn").unwrap(),
            ),
            function_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionName").unwrap(),
            ),
            function_response_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionResponseTypes").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            last_processing_result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastProcessingResult").unwrap(),
            ),
            maximum_batching_window_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumBatchingWindowInSeconds").unwrap(),
            ),
            maximum_record_age_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumRecordAgeInSeconds").unwrap(),
            ),
            maximum_retry_attempts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumRetryAttempts").unwrap(),
            ),
            parallelization_factor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parallelizationFactor").unwrap(),
            ),
            queues: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queues").unwrap(),
            ),
            scaling_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingConfig").unwrap(),
            ),
            self_managed_event_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfManagedEventSource").unwrap(),
            ),
            self_managed_kafka_event_source_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfManagedKafkaEventSourceConfig").unwrap(),
            ),
            source_access_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceAccessConfigurations").unwrap(),
            ),
            starting_position: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startingPosition").unwrap(),
            ),
            starting_position_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startingPositionTimestamp").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_transition_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateTransitionReason").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            topics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topics").unwrap(),
            ),
            tumbling_window_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tumblingWindowInSeconds").unwrap(),
            ),
            uuid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uuid").unwrap(),
            ),
        }
    }
}