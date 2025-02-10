/// Provides a Synthetics Canary resource.
///
/// > **NOTE:** When you create a canary, AWS creates supporting implicit resources. See the Amazon CloudWatch Synthetics documentation on [DeleteCanary](https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_DeleteCanary.html) for a full list. Neither AWS nor this provider deletes these implicit resources automatically when the canary is deleted. Before deleting a canary, ensure you have all the information about the canary that you need to delete the implicit resources using the AWS Console, or AWS CLI.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let some = canary::create(
///         "some",
///         CanaryArgs::builder()
///             .artifact_s_3_location("s3://some-bucket/")
///             .execution_role_arn("some-role")
///             .handler("exports.handler")
///             .name("some-canary")
///             .runtime_version("syn-1.0")
///             .schedule(
///                 CanarySchedule::builder().expression("rate(0 minute)").build_struct(),
///             )
///             .zip_file("test-fixtures/lambdatest.zip")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Synthetics Canaries using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:synthetics/canary:Canary some some-canary
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod canary {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CanaryArgs {
        /// configuration for canary artifacts, including the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3. See Artifact Config.
        #[builder(into, default)]
        pub artifact_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synthetics::CanaryArtifactConfig>,
        >,
        /// Location in Amazon S3 where Synthetics stores artifacts from the test runs of this canary.
        #[builder(into)]
        pub artifact_s3_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether to also delete the Lambda functions and layers used by this canary. The default is `false`.
        #[builder(into, default)]
        pub delete_lambda: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ARN of the IAM role to be used to run the canary. see [AWS Docs](https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_CreateCanary.html#API_CreateCanary_RequestSyntax) for permissions needs for IAM Role.
        #[builder(into)]
        pub execution_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of days to retain data about failed runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
        #[builder(into, default)]
        pub failure_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Entry point to use for the source code when running the canary. This value must end with the string `.handler` .
        #[builder(into)]
        pub handler: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name for this canary. Has a maximum length of 21 characters. Valid characters are lowercase alphanumeric, hyphen, or underscore.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for individual canary runs. Detailed below.
        #[builder(into, default)]
        pub run_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synthetics::CanaryRunConfig>,
        >,
        /// Runtime version to use for the canary. Versions change often so consult the [Amazon CloudWatch documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html) for the latest valid versions. Values include `syn-python-selenium-1.0`, `syn-nodejs-puppeteer-3.0`, `syn-nodejs-2.2`, `syn-nodejs-2.1`, `syn-nodejs-2.0`, and `syn-1.0`.
        #[builder(into)]
        pub runtime_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Full bucket name which is used if your canary script is located in S3. The bucket must already exist. **Conflicts with `zip_file`.**
        #[builder(into, default)]
        pub s3_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// S3 key of your script. **Conflicts with `zip_file`.**
        #[builder(into, default)]
        pub s3_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// S3 version ID of your script. **Conflicts with `zip_file`.**
        #[builder(into, default)]
        pub s3_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block providing how often the canary is to run and when these test runs are to stop. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::synthetics::CanarySchedule,
        >,
        /// Whether to run or stop the canary.
        #[builder(into, default)]
        pub start_canary: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Number of days to retain data about successful runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
        #[builder(into, default)]
        pub success_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synthetics::CanaryVpcConfig>,
        >,
        /// ZIP file that contains the script, if you input your canary script directly into the canary instead of referring to an S3 location. It can be up to 225KB. **Conflicts with `s3_bucket`, `s3_key`, and `s3_version`.**
        #[builder(into, default)]
        pub zip_file: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CanaryResult {
        /// Amazon Resource Name (ARN) of the Canary.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// configuration for canary artifacts, including the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3. See Artifact Config.
        pub artifact_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::synthetics::CanaryArtifactConfig>,
        >,
        /// Location in Amazon S3 where Synthetics stores artifacts from the test runs of this canary.
        pub artifact_s3_location: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to also delete the Lambda functions and layers used by this canary. The default is `false`.
        pub delete_lambda: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ARN of the Lambda function that is used as your canary's engine.
        pub engine_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role to be used to run the canary. see [AWS Docs](https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_CreateCanary.html#API_CreateCanary_RequestSyntax) for permissions needs for IAM Role.
        pub execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Number of days to retain data about failed runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
        pub failure_retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Entry point to use for the source code when running the canary. This value must end with the string `.handler` .
        pub handler: pulumi_gestalt_rust::Output<String>,
        /// Name for this canary. Has a maximum length of 21 characters. Valid characters are lowercase alphanumeric, hyphen, or underscore.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for individual canary runs. Detailed below.
        pub run_config: pulumi_gestalt_rust::Output<
            super::super::types::synthetics::CanaryRunConfig,
        >,
        /// Runtime version to use for the canary. Versions change often so consult the [Amazon CloudWatch documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html) for the latest valid versions. Values include `syn-python-selenium-1.0`, `syn-nodejs-puppeteer-3.0`, `syn-nodejs-2.2`, `syn-nodejs-2.1`, `syn-nodejs-2.0`, and `syn-1.0`.
        pub runtime_version: pulumi_gestalt_rust::Output<String>,
        /// Full bucket name which is used if your canary script is located in S3. The bucket must already exist. **Conflicts with `zip_file`.**
        pub s3_bucket: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 key of your script. **Conflicts with `zip_file`.**
        pub s3_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// S3 version ID of your script. **Conflicts with `zip_file`.**
        pub s3_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block providing how often the canary is to run and when these test runs are to stop. Detailed below.
        ///
        /// The following arguments are optional:
        pub schedule: pulumi_gestalt_rust::Output<
            super::super::types::synthetics::CanarySchedule,
        >,
        /// ARN of the Lambda layer where Synthetics stores the canary script code.
        pub source_location_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to run or stop the canary.
        pub start_canary: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Canary status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Number of days to retain data about successful runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
        pub success_retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Structure that contains information about when the canary was created, modified, and most recently run. see Timeline.
        pub timelines: pulumi_gestalt_rust::Output<
            Vec<super::super::types::synthetics::CanaryTimeline>,
        >,
        /// Configuration block. Detailed below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::synthetics::CanaryVpcConfig>,
        >,
        /// ZIP file that contains the script, if you input your canary script directly into the canary instead of referring to an S3 location. It can be up to 225KB. **Conflicts with `s3_bucket`, `s3_key`, and `s3_version`.**
        pub zip_file: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CanaryArgs,
    ) -> CanaryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let artifact_config_binding = args.artifact_config.get_output(context);
        let artifact_s3_location_binding = args.artifact_s3_location.get_output(context);
        let delete_lambda_binding = args.delete_lambda.get_output(context);
        let execution_role_arn_binding = args.execution_role_arn.get_output(context);
        let failure_retention_period_binding = args
            .failure_retention_period
            .get_output(context);
        let handler_binding = args.handler.get_output(context);
        let name_binding = args.name.get_output(context);
        let run_config_binding = args.run_config.get_output(context);
        let runtime_version_binding = args.runtime_version.get_output(context);
        let s3_bucket_binding = args.s3_bucket.get_output(context);
        let s3_key_binding = args.s3_key.get_output(context);
        let s3_version_binding = args.s3_version.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let start_canary_binding = args.start_canary.get_output(context);
        let success_retention_period_binding = args
            .success_retention_period
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let zip_file_binding = args.zip_file.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:synthetics/canary:Canary".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "artifactConfig".into(),
                    value: artifact_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "artifactS3Location".into(),
                    value: artifact_s3_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteLambda".into(),
                    value: delete_lambda_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRoleArn".into(),
                    value: execution_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failureRetentionPeriod".into(),
                    value: failure_retention_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "handler".into(),
                    value: handler_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runConfig".into(),
                    value: run_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeVersion".into(),
                    value: runtime_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Bucket".into(),
                    value: s3_bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Key".into(),
                    value: s3_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Version".into(),
                    value: s3_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startCanary".into(),
                    value: start_canary_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "successRetentionPeriod".into(),
                    value: success_retention_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: vpc_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zipFile".into(),
                    value: zip_file_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CanaryResult {
            arn: o.get_field("arn"),
            artifact_config: o.get_field("artifactConfig"),
            artifact_s3_location: o.get_field("artifactS3Location"),
            delete_lambda: o.get_field("deleteLambda"),
            engine_arn: o.get_field("engineArn"),
            execution_role_arn: o.get_field("executionRoleArn"),
            failure_retention_period: o.get_field("failureRetentionPeriod"),
            handler: o.get_field("handler"),
            name: o.get_field("name"),
            run_config: o.get_field("runConfig"),
            runtime_version: o.get_field("runtimeVersion"),
            s3_bucket: o.get_field("s3Bucket"),
            s3_key: o.get_field("s3Key"),
            s3_version: o.get_field("s3Version"),
            schedule: o.get_field("schedule"),
            source_location_arn: o.get_field("sourceLocationArn"),
            start_canary: o.get_field("startCanary"),
            status: o.get_field("status"),
            success_retention_period: o.get_field("successRetentionPeriod"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timelines: o.get_field("timelines"),
            vpc_config: o.get_field("vpcConfig"),
            zip_file: o.get_field("zipFile"),
        }
    }
}
