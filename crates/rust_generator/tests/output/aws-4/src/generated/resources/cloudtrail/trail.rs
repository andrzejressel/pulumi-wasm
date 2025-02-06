/// Provides a CloudTrail resource.
///
/// > **Tip:** For a multi-region trail, this resource must be in the home region of the trail.
///
/// > **Tip:** For an organization trail, this resource must be in the master account of the organization.
///
/// ## Example Usage
///
/// ### Basic
///
/// Enable CloudTrail to capture all compatible management events in region.
/// For capturing events from services like IAM, `include_global_service_events` must be enabled.
///
/// ```yaml
/// resources:
///   exampleTrail:
///     type: aws:cloudtrail:Trail
///     name: example
///     properties:
///       name: example
///       s3BucketName: ${exampleBucketV2.id}
///       s3KeyPrefix: prefix
///       includeGlobalServiceEvents: false
///     options:
///       dependsOn:
///         - ${exampleBucketPolicy}
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: my-test-trail
///       forceDestroy: true
///   exampleBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: example
///     properties:
///       bucket: ${exampleBucketV2.id}
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: AWSCloudTrailAclCheck
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - cloudtrail.amazonaws.com
///             actions:
///               - s3:GetBucketAcl
///             resources:
///               - ${exampleBucketV2.arn}
///             conditions:
///               - test: StringEquals
///                 variable: aws:SourceArn
///                 values:
///                   - arn:${currentGetPartition.partition}:cloudtrail:${currentGetRegion.name}:${current.accountId}:trail/example
///           - sid: AWSCloudTrailWrite
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - cloudtrail.amazonaws.com
///             actions:
///               - s3:PutObject
///             resources:
///               - ${exampleBucketV2.arn}/prefix/AWSLogs/${current.accountId}/*
///             conditions:
///               - test: StringEquals
///                 variable: s3:x-amz-acl
///                 values:
///                   - bucket-owner-full-control
///               - test: StringEquals
///                 variable: aws:SourceArn
///                 values:
///                   - arn:${currentGetPartition.partition}:cloudtrail:${currentGetRegion.name}:${current.accountId}:trail/example
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   currentGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ### Data Event Logging
///
/// CloudTrail can log [Data Events](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html) for certain services such as S3 objects and Lambda function invocations. Additional information about data event configuration can be found in the following links:
///
/// * [CloudTrail API DataResource documentation](https://docs.aws.amazon.com/awscloudtrail/latest/APIReference/API_DataResource.html) (for basic event selector).
/// * [CloudTrail API AdvancedFieldSelector documentation](https://docs.aws.amazon.com/awscloudtrail/latest/APIReference/API_AdvancedFieldSelector.html) (for advanced event selector).
///
/// ### Logging All Lambda Function Invocations By Using Basic Event Selectors
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trail::create(
///         "example",
///         TrailArgs::builder()
///             .event_selectors(
///                 vec![
///                     TrailEventSelector::builder()
///                     .dataResources(vec![TrailEventSelectorDataResource::builder(). type
///                     ("AWS::Lambda::Function").values(vec!["arn:aws:lambda",])
///                     .build_struct(),]).includeManagementEvents(true).readWriteType("All")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Logging All S3 Object Events By Using Basic Event Selectors
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trail::create(
///         "example",
///         TrailArgs::builder()
///             .event_selectors(
///                 vec![
///                     TrailEventSelector::builder()
///                     .dataResources(vec![TrailEventSelectorDataResource::builder(). type
///                     ("AWS::S3::Object").values(vec!["arn:aws:s3",]).build_struct(),])
///                     .includeManagementEvents(true).readWriteType("All").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Logging Individual S3 Bucket Events By Using Basic Event Selectors
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudtrail:Trail
///     properties:
///       eventSelectors:
///         - readWriteType: All
///           includeManagementEvents: true
///           dataResources:
///             - type: AWS::S3::Object
///               values:
///                 - ${["important-bucket"].arn}/
/// variables:
///   important-bucket:
///     fn::invoke:
///       function: aws:s3:getBucket
///       arguments:
///         bucket: important-bucket
/// ```
///
/// ### Logging All S3 Object Events Except For Two S3 Buckets By Using Advanced Event Selectors
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudtrail:Trail
///     properties:
///       advancedEventSelectors:
///         - name: Log all S3 objects events except for two S3 buckets
///           fieldSelectors:
///             - field: eventCategory
///               equals:
///                 - Data
///             - field: resources.ARN
///               notStartsWiths:
///                 - ${["not-important-bucket-1"].arn}/
///                 - ${["not-important-bucket-2"].arn}/
///             - field: resources.type
///               equals:
///                 - AWS::S3::Object
///         - name: Log readOnly and writeOnly management events
///           fieldSelectors:
///             - field: eventCategory
///               equals:
///                 - Management
/// variables:
///   not-important-bucket-1:
///     fn::invoke:
///       function: aws:s3:getBucket
///       arguments:
///         bucket: not-important-bucket-1
///   not-important-bucket-2:
///     fn::invoke:
///       function: aws:s3:getBucket
///       arguments:
///         bucket: not-important-bucket-2
/// ```
///
/// ### Logging Individual S3 Buckets And Specific Event Names By Using Advanced Event Selectors
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudtrail:Trail
///     properties:
///       advancedEventSelectors:
///         - name: Log PutObject and DeleteObject events for two S3 buckets
///           fieldSelectors:
///             - field: eventCategory
///               equals:
///                 - Data
///             - field: eventName
///               equals:
///                 - PutObject
///                 - DeleteObject
///             - field: resources.ARN
///               startsWiths:
///                 - ${["important-bucket-1"].arn}/
///                 - ${["important-bucket-2"].arn}/
///             - field: readOnly
///               equals:
///                 - 'false'
///             - field: resources.type
///               equals:
///                 - AWS::S3::Object
///         - name: Log Delete* events for one S3 bucket
///           fieldSelectors:
///             - field: eventCategory
///               equals:
///                 - Data
///             - field: eventName
///               startsWiths:
///                 - Delete
///             - field: resources.ARN
///               equals:
///                 - ${["important-bucket-3"].arn}/important-prefix
///             - field: readOnly
///               equals:
///                 - 'false'
///             - field: resources.type
///               equals:
///                 - AWS::S3::Object
/// variables:
///   important-bucket-1:
///     fn::invoke:
///       function: aws:s3:getBucket
///       arguments:
///         bucket: important-bucket-1
///   important-bucket-2:
///     fn::invoke:
///       function: aws:s3:getBucket
///       arguments:
///         bucket: important-bucket-2
///   important-bucket-3:
///     fn::invoke:
///       function: aws:s3:getBucket
///       arguments:
///         bucket: important-bucket-3
/// ```
///
/// ### Sending Events to CloudWatch Logs
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = log_group::create(
///         "example",
///         LogGroupArgs::builder().name("Example").build_struct(),
///     );
///     let exampleTrail = trail::create(
///         "exampleTrail",
///         TrailArgs::builder()
///             .cloud_watch_logs_group_arn("${example.arn}:*")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudtrails using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudtrail/trail:Trail sample arn:aws:cloudtrail:us-east-1:123456789012:trail/my-sample-trail
/// ```
pub mod trail {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrailArgs {
        /// Specifies an advanced event selector for enabling data event logging. Fields documented below. Conflicts with `event_selector`.
        #[builder(into, default)]
        pub advanced_event_selectors: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudtrail::TrailAdvancedEventSelector>>,
        >,
        /// Log group name using an ARN that represents the log group to which CloudTrail logs will be delivered. Note that CloudTrail requires the Log Stream wildcard.
        #[builder(into, default)]
        pub cloud_watch_logs_group_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Role for the CloudWatch Logs endpoint to assume to write to a user’s log group.
        #[builder(into, default)]
        pub cloud_watch_logs_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether log file integrity validation is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub enable_log_file_validation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enables logging for the trail. Defaults to `true`. Setting this to `false` will pause logging.
        #[builder(into, default)]
        pub enable_logging: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies an event selector for enabling data event logging. Fields documented below. Please note the [CloudTrail limits](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/WhatIsCloudTrail-Limits.html) when configuring these. Conflicts with `advanced_event_selector`.
        #[builder(into, default)]
        pub event_selectors: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudtrail::TrailEventSelector>>,
        >,
        /// Whether the trail is publishing events from global services such as IAM to the log files. Defaults to `true`.
        #[builder(into, default)]
        pub include_global_service_events: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for identifying unusual operational activity. See details below.
        #[builder(into, default)]
        pub insight_selectors: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudtrail::TrailInsightSelector>>,
        >,
        /// Whether the trail is created in the current region or in all regions. Defaults to `false`.
        #[builder(into, default)]
        pub is_multi_region_trail: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether the trail is an AWS Organizations trail. Organization trails log events for the master account and all member accounts. Can only be created in the organization master account. Defaults to `false`.
        #[builder(into, default)]
        pub is_organization_trail: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// KMS key ARN to use to encrypt the logs delivered by CloudTrail.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the trail.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the S3 bucket designated for publishing log files.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub s3_bucket_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// S3 key prefix that follows the name of the bucket you have designated for log file delivery.
        #[builder(into, default)]
        pub s3_key_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the Amazon SNS topic defined for notification of log file delivery.
        #[builder(into, default)]
        pub sns_topic_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the trail. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrailResult {
        /// Specifies an advanced event selector for enabling data event logging. Fields documented below. Conflicts with `event_selector`.
        pub advanced_event_selectors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudtrail::TrailAdvancedEventSelector>>,
        >,
        /// ARN of the trail.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Log group name using an ARN that represents the log group to which CloudTrail logs will be delivered. Note that CloudTrail requires the Log Stream wildcard.
        pub cloud_watch_logs_group_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Role for the CloudWatch Logs endpoint to assume to write to a user’s log group.
        pub cloud_watch_logs_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether log file integrity validation is enabled. Defaults to `false`.
        pub enable_log_file_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables logging for the trail. Defaults to `true`. Setting this to `false` will pause logging.
        pub enable_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies an event selector for enabling data event logging. Fields documented below. Please note the [CloudTrail limits](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/WhatIsCloudTrail-Limits.html) when configuring these. Conflicts with `advanced_event_selector`.
        pub event_selectors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudtrail::TrailEventSelector>>,
        >,
        /// Region in which the trail was created.
        pub home_region: pulumi_wasm_rust::Output<String>,
        /// Whether the trail is publishing events from global services such as IAM to the log files. Defaults to `true`.
        pub include_global_service_events: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for identifying unusual operational activity. See details below.
        pub insight_selectors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudtrail::TrailInsightSelector>>,
        >,
        /// Whether the trail is created in the current region or in all regions. Defaults to `false`.
        pub is_multi_region_trail: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the trail is an AWS Organizations trail. Organization trails log events for the master account and all member accounts. Can only be created in the organization master account. Defaults to `false`.
        pub is_organization_trail: pulumi_wasm_rust::Output<Option<bool>>,
        /// KMS key ARN to use to encrypt the logs delivered by CloudTrail.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the trail.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of the S3 bucket designated for publishing log files.
        ///
        /// The following arguments are optional:
        pub s3_bucket_name: pulumi_wasm_rust::Output<String>,
        /// S3 key prefix that follows the name of the bucket you have designated for log file delivery.
        pub s3_key_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Amazon SNS topic defined for notification of log file delivery.
        pub sns_topic_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the trail. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TrailArgs,
    ) -> TrailResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_event_selectors_binding = args
            .advanced_event_selectors
            .get_output(context)
            .get_inner();
        let cloud_watch_logs_group_arn_binding = args
            .cloud_watch_logs_group_arn
            .get_output(context)
            .get_inner();
        let cloud_watch_logs_role_arn_binding = args
            .cloud_watch_logs_role_arn
            .get_output(context)
            .get_inner();
        let enable_log_file_validation_binding = args
            .enable_log_file_validation
            .get_output(context)
            .get_inner();
        let enable_logging_binding = args.enable_logging.get_output(context).get_inner();
        let event_selectors_binding = args
            .event_selectors
            .get_output(context)
            .get_inner();
        let include_global_service_events_binding = args
            .include_global_service_events
            .get_output(context)
            .get_inner();
        let insight_selectors_binding = args
            .insight_selectors
            .get_output(context)
            .get_inner();
        let is_multi_region_trail_binding = args
            .is_multi_region_trail
            .get_output(context)
            .get_inner();
        let is_organization_trail_binding = args
            .is_organization_trail
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let s3_bucket_name_binding = args.s3_bucket_name.get_output(context).get_inner();
        let s3_key_prefix_binding = args.s3_key_prefix.get_output(context).get_inner();
        let sns_topic_name_binding = args.sns_topic_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudtrail/trail:Trail".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedEventSelectors".into(),
                    value: &advanced_event_selectors_binding,
                },
                register_interface::ObjectField {
                    name: "cloudWatchLogsGroupArn".into(),
                    value: &cloud_watch_logs_group_arn_binding,
                },
                register_interface::ObjectField {
                    name: "cloudWatchLogsRoleArn".into(),
                    value: &cloud_watch_logs_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "enableLogFileValidation".into(),
                    value: &enable_log_file_validation_binding,
                },
                register_interface::ObjectField {
                    name: "enableLogging".into(),
                    value: &enable_logging_binding,
                },
                register_interface::ObjectField {
                    name: "eventSelectors".into(),
                    value: &event_selectors_binding,
                },
                register_interface::ObjectField {
                    name: "includeGlobalServiceEvents".into(),
                    value: &include_global_service_events_binding,
                },
                register_interface::ObjectField {
                    name: "insightSelectors".into(),
                    value: &insight_selectors_binding,
                },
                register_interface::ObjectField {
                    name: "isMultiRegionTrail".into(),
                    value: &is_multi_region_trail_binding,
                },
                register_interface::ObjectField {
                    name: "isOrganizationTrail".into(),
                    value: &is_organization_trail_binding,
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
                    name: "s3BucketName".into(),
                    value: &s3_bucket_name_binding,
                },
                register_interface::ObjectField {
                    name: "s3KeyPrefix".into(),
                    value: &s3_key_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "snsTopicName".into(),
                    value: &sns_topic_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TrailResult {
            advanced_event_selectors: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("advancedEventSelectors"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cloud_watch_logs_group_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudWatchLogsGroupArn"),
            ),
            cloud_watch_logs_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudWatchLogsRoleArn"),
            ),
            enable_log_file_validation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableLogFileValidation"),
            ),
            enable_logging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableLogging"),
            ),
            event_selectors: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventSelectors"),
            ),
            home_region: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("homeRegion"),
            ),
            include_global_service_events: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includeGlobalServiceEvents"),
            ),
            insight_selectors: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("insightSelectors"),
            ),
            is_multi_region_trail: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isMultiRegionTrail"),
            ),
            is_organization_trail: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isOrganizationTrail"),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            s3_bucket_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3BucketName"),
            ),
            s3_key_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("s3KeyPrefix"),
            ),
            sns_topic_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snsTopicName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
