/// Manages a Kinesis Analytics v2 Application.
/// This resource can be used to manage both Kinesis Data Analytics for SQL applications and Kinesis Data Analytics for Apache Flink applications.
///
/// > **Note:** Kinesis Data Analytics for SQL applications created using this resource cannot currently be viewed in the AWS Console. To manage Kinesis Data Analytics for SQL applications that can also be viewed in the AWS Console, use the `aws.kinesis.AnalyticsApplication` resource.
///
/// ## Example Usage
///
/// ### Apache Flink Application
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example-flink-application
///   exampleBucketObjectv2:
///     type: aws:s3:BucketObjectv2
///     name: example
///     properties:
///       bucket: ${example.id}
///       key: example-flink-application
///       source:
///         fn::FileAsset: flink-app.jar
///   exampleApplication:
///     type: aws:kinesisanalyticsv2:Application
///     name: example
///     properties:
///       name: example-flink-application
///       runtimeEnvironment: FLINK-1_8
///       serviceExecutionRole: ${exampleAwsIamRole.arn}
///       applicationConfiguration:
///         applicationCodeConfiguration:
///           codeContent:
///             s3ContentLocation:
///               bucketArn: ${example.arn}
///               fileKey: ${exampleBucketObjectv2.key}
///           codeContentType: ZIPFILE
///         environmentProperties:
///           propertyGroups:
///             - propertyGroupId: PROPERTY-GROUP-1
///               propertyMap:
///                 Key1: Value1
///             - propertyGroupId: PROPERTY-GROUP-2
///               propertyMap:
///                 KeyA: ValueA
///                 KeyB: ValueB
///         flinkApplicationConfiguration:
///           checkpointConfiguration:
///             configurationType: DEFAULT
///           monitoringConfiguration:
///             configurationType: CUSTOM
///             logLevel: DEBUG
///             metricsLevel: TASK
///           parallelismConfiguration:
///             autoScalingEnabled: true
///             configurationType: CUSTOM
///             parallelism: 10
///             parallelismPerKpu: 4
///       tags:
///         Environment: test
/// ```
///
/// ### SQL Application
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = log_group::create(
///         "example",
///         LogGroupArgs::builder().name("example-sql-application").build_struct(),
///     );
///     let exampleApplication = application::create(
///         "exampleApplication",
///         ApplicationArgs::builder()
///             .application_configuration(
///                 ApplicationApplicationConfiguration::builder()
///                     .applicationCodeConfiguration(
///                         ApplicationApplicationConfigurationApplicationCodeConfiguration::builder()
///                             .codeContent(
///                                 ApplicationApplicationConfigurationApplicationCodeConfigurationCodeContent::builder()
///                                     .textContent("SELECT 1;\n")
///                                     .build_struct(),
///                             )
///                             .codeContentType("PLAINTEXT")
///                             .build_struct(),
///                     )
///                     .sqlApplicationConfiguration(
///                         ApplicationApplicationConfigurationSqlApplicationConfiguration::builder()
///                             .input(
///                                 ApplicationApplicationConfigurationSqlApplicationConfigurationInput::builder()
///                                     .inputParallelism(
///                                         ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputParallelism::builder()
///                                             .count(3)
///                                             .build_struct(),
///                                     )
///                                     .inputSchema(
///                                         ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchema::builder()
///                                             .recordColumns(
///                                                 vec![
///                                                     ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordColumn::builder()
///                                                     .mapping("MAPPING-1").name("COLUMN_1").sqlType("VARCHAR(8)")
///                                                     .build_struct(),
///                                                     ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordColumn::builder()
///                                                     .name("COLUMN_2").sqlType("DOUBLE").build_struct(),
///                                                 ],
///                                             )
///                                             .recordEncoding("UTF-8")
///                                             .recordFormat(
///                                                 ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordFormat::builder()
///                                                     .mappingParameters(
///                                                         ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordFormatMappingParameters::builder()
///                                                             .csvMappingParameters(
///                                                                 ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordFormatMappingParametersCsvMappingParameters::builder()
///                                                                     .recordColumnDelimiter(",")
///                                                                     .recordRowDelimiter("")
///                                                                     .build_struct(),
///                                                             )
///                                                             .build_struct(),
///                                                     )
///                                                     .recordFormatType("CSV")
///                                                     .build_struct(),
///                                             )
///                                             .build_struct(),
///                                     )
///                                     .kinesisStreamsInput(
///                                         ApplicationApplicationConfigurationSqlApplicationConfigurationInputKinesisStreamsInput::builder()
///                                             .resourceArn("${exampleAwsKinesisStream.arn}")
///                                             .build_struct(),
///                                     )
///                                     .namePrefix("PREFIX_1")
///                                     .build_struct(),
///                             )
///                             .outputs(
///                                 vec![
///                                     ApplicationApplicationConfigurationSqlApplicationConfigurationOutput::builder()
///                                     .destinationSchema(ApplicationApplicationConfigurationSqlApplicationConfigurationOutputDestinationSchema::builder()
///                                     .recordFormatType("JSON").build_struct())
///                                     .lambdaOutput(ApplicationApplicationConfigurationSqlApplicationConfigurationOutputLambdaOutput::builder()
///                                     .resourceArn("${exampleAwsLambdaFunction.arn}")
///                                     .build_struct()).name("OUTPUT_1").build_struct(),
///                                     ApplicationApplicationConfigurationSqlApplicationConfigurationOutput::builder()
///                                     .destinationSchema(ApplicationApplicationConfigurationSqlApplicationConfigurationOutputDestinationSchema::builder()
///                                     .recordFormatType("CSV").build_struct())
///                                     .kinesisFirehoseOutput(ApplicationApplicationConfigurationSqlApplicationConfigurationOutputKinesisFirehoseOutput::builder()
///                                     .resourceArn("${exampleAwsKinesisFirehoseDeliveryStream.arn}")
///                                     .build_struct()).name("OUTPUT_2").build_struct(),
///                                 ],
///                             )
///                             .referenceDataSource(
///                                 ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSource::builder()
///                                     .referenceSchema(
///                                         ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchema::builder()
///                                             .recordColumns(
///                                                 vec![
///                                                     ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordColumn::builder()
///                                                     .name("COLUMN_1").sqlType("INTEGER").build_struct(),
///                                                 ],
///                                             )
///                                             .recordFormat(
///                                                 ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormat::builder()
///                                                     .mappingParameters(
///                                                         ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormatMappingParameters::builder()
///                                                             .jsonMappingParameters(
///                                                                 ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceReferenceSchemaRecordFormatMappingParametersJsonMappingParameters::builder()
///                                                                     .recordRowPath("$")
///                                                                     .build_struct(),
///                                                             )
///                                                             .build_struct(),
///                                                     )
///                                                     .recordFormatType("JSON")
///                                                     .build_struct(),
///                                             )
///                                             .build_struct(),
///                                     )
///                                     .s3ReferenceDataSource(
///                                         ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSourceS3ReferenceDataSource::builder()
///                                             .bucketArn("${exampleAwsS3Bucket.arn}")
///                                             .fileKey("KEY-1")
///                                             .build_struct(),
///                                     )
///                                     .tableName("TABLE-1")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .cloudwatch_logging_options(
///                 ApplicationCloudwatchLoggingOptions::builder()
///                     .logStreamArn("${exampleLogStream.arn}")
///                     .build_struct(),
///             )
///             .name("example-sql-application")
///             .runtime_environment("SQL-1_0")
///             .service_execution_role("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
///     let exampleLogStream = log_stream::create(
///         "exampleLogStream",
///         LogStreamArgs::builder()
///             .log_group_name("${example.name}")
///             .name("example-sql-application")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### VPC Configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example-flink-application
///   exampleBucketObjectv2:
///     type: aws:s3:BucketObjectv2
///     name: example
///     properties:
///       bucket: ${example.id}
///       key: example-flink-application
///       source:
///         fn::FileAsset: flink-app.jar
///   exampleApplication:
///     type: aws:kinesisanalyticsv2:Application
///     name: example
///     properties:
///       name: example-flink-application
///       runtimeEnvironment: FLINK-1_8
///       serviceExecutionRole: ${exampleAwsIamRole.arn}
///       applicationConfiguration:
///         applicationCodeConfiguration:
///           codeContent:
///             s3ContentLocation:
///               bucketArn: ${example.arn}
///               fileKey: ${exampleBucketObjectv2.key}
///           codeContentType: ZIPFILE
///         vpcConfiguration:
///           securityGroupIds:
///             - ${exampleAwsSecurityGroup[0].id}
///             - ${exampleAwsSecurityGroup[1].id}
///           subnetIds:
///             - ${exampleAwsSubnet.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_kinesisanalyticsv2_application` using the application ARN. For example:
///
/// ```sh
/// $ pulumi import aws:kinesisanalyticsv2/application:Application example arn:aws:kinesisanalytics:us-west-2:123456789012:application/example-sql-application
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The application's configuration
        #[builder(into, default)]
        pub application_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::kinesisanalyticsv2::ApplicationApplicationConfiguration,
            >,
        >,
        /// The application's mode. Valid values are `STREAMING`, `INTERACTIVE`.
        #[builder(into, default)]
        pub application_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A CloudWatch log stream to monitor application configuration errors.
        #[builder(into, default)]
        pub cloudwatch_logging_options: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::kinesisanalyticsv2::ApplicationCloudwatchLoggingOptions,
            >,
        >,
        /// A summary description of the application.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to force stop an unresponsive Flink-based application.
        #[builder(into, default)]
        pub force_stop: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the application.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The runtime environment for the application. Valid values: `SQL-1_0`, `FLINK-1_6`, `FLINK-1_8`, `FLINK-1_11`, `FLINK-1_13`, `FLINK-1_15`, `FLINK-1_18`, `FLINK-1_19`.
        #[builder(into)]
        pub runtime_environment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the IAM role used by the application to access Kinesis data streams, Kinesis Data Firehose delivery streams, Amazon S3 objects, and other external resources.
        #[builder(into)]
        pub service_execution_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to start or stop the application.
        #[builder(into, default)]
        pub start_application: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The application's configuration
        pub application_configuration: pulumi_gestalt_rust::Output<
            super::super::types::kinesisanalyticsv2::ApplicationApplicationConfiguration,
        >,
        /// The application's mode. Valid values are `STREAMING`, `INTERACTIVE`.
        pub application_mode: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the application.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A CloudWatch log stream to monitor application configuration errors.
        pub cloudwatch_logging_options: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::kinesisanalyticsv2::ApplicationCloudwatchLoggingOptions,
            >,
        >,
        /// The current timestamp when the application was created.
        pub create_timestamp: pulumi_gestalt_rust::Output<String>,
        /// A summary description of the application.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to force stop an unresponsive Flink-based application.
        pub force_stop: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The current timestamp when the application was last updated.
        pub last_update_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The name of the application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The runtime environment for the application. Valid values: `SQL-1_0`, `FLINK-1_6`, `FLINK-1_8`, `FLINK-1_11`, `FLINK-1_13`, `FLINK-1_15`, `FLINK-1_18`, `FLINK-1_19`.
        pub runtime_environment: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the IAM role used by the application to access Kinesis data streams, Kinesis Data Firehose delivery streams, Amazon S3 objects, and other external resources.
        pub service_execution_role: pulumi_gestalt_rust::Output<String>,
        /// Whether to start or stop the application.
        pub start_application: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The status of the application.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current application version. Kinesis Data Analytics updates the `version_id` each time the application is updated.
        pub version_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_configuration_binding = args
            .application_configuration
            .get_output(context);
        let application_mode_binding = args.application_mode.get_output(context);
        let cloudwatch_logging_options_binding = args
            .cloudwatch_logging_options
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let force_stop_binding = args.force_stop.get_output(context);
        let name_binding = args.name.get_output(context);
        let runtime_environment_binding = args.runtime_environment.get_output(context);
        let service_execution_role_binding = args
            .service_execution_role
            .get_output(context);
        let start_application_binding = args.start_application.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kinesisanalyticsv2/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationConfiguration".into(),
                    value: &application_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationMode".into(),
                    value: &application_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchLoggingOptions".into(),
                    value: &cloudwatch_logging_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceStop".into(),
                    value: &force_stop_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeEnvironment".into(),
                    value: &runtime_environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceExecutionRole".into(),
                    value: &service_execution_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startApplication".into(),
                    value: &start_application_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            application_configuration: o.get_field("applicationConfiguration"),
            application_mode: o.get_field("applicationMode"),
            arn: o.get_field("arn"),
            cloudwatch_logging_options: o.get_field("cloudwatchLoggingOptions"),
            create_timestamp: o.get_field("createTimestamp"),
            description: o.get_field("description"),
            force_stop: o.get_field("forceStop"),
            last_update_timestamp: o.get_field("lastUpdateTimestamp"),
            name: o.get_field("name"),
            runtime_environment: o.get_field("runtimeEnvironment"),
            service_execution_role: o.get_field("serviceExecutionRole"),
            start_application: o.get_field("startApplication"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version_id: o.get_field("versionId"),
        }
    }
}
