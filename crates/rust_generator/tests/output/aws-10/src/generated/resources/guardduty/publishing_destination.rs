/// Provides a resource to manage a GuardDuty PublishingDestination. Requires an existing GuardDuty Detector.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testGd:
///     type: aws:guardduty:Detector
///     name: test_gd
///     properties:
///       enable: true
///   gdBucket:
///     type: aws:s3:BucketV2
///     name: gd_bucket
///     properties:
///       bucket: example
///       forceDestroy: true
///   gdBucketAcl:
///     type: aws:s3:BucketAclV2
///     name: gd_bucket_acl
///     properties:
///       bucket: ${gdBucket.id}
///       acl: private
///   gdBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: gd_bucket_policy
///     properties:
///       bucket: ${gdBucket.id}
///       policy: ${bucketPol.json}
///   gdKey:
///     type: aws:kms:Key
///     name: gd_key
///     properties:
///       description: Temporary key for AccTest of TF
///       deletionWindowInDays: 7
///       policy: ${kmsPol.json}
///   test:
///     type: aws:guardduty:PublishingDestination
///     properties:
///       detectorId: ${testGd.id}
///       destinationArn: ${gdBucket.arn}
///       kmsKeyArn: ${gdKey.arn}
///     options:
///       dependsOn:
///         - ${gdBucketPolicy}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   bucketPol:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: Allow PutObject
///             actions:
///               - s3:PutObject
///             resources:
///               - ${gdBucket.arn}/*
///             principals:
///               - type: Service
///                 identifiers:
///                   - guardduty.amazonaws.com
///           - sid: Allow GetBucketLocation
///             actions:
///               - s3:GetBucketLocation
///             resources:
///               - ${gdBucket.arn}
///             principals:
///               - type: Service
///                 identifiers:
///                   - guardduty.amazonaws.com
///   kmsPol:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: Allow GuardDuty to encrypt findings
///             actions:
///               - kms:GenerateDataKey
///             resources:
///               - arn:aws:kms:${currentGetRegion.name}:${current.accountId}:key/*
///             principals:
///               - type: Service
///                 identifiers:
///                   - guardduty.amazonaws.com
///           - sid: Allow all users to modify/delete key (test only)
///             actions:
///               - kms:*
///             resources:
///               - arn:aws:kms:${currentGetRegion.name}:${current.accountId}:key/*
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::${current.accountId}:root
/// ```
///
/// > **Note:** Please do not use this simple example for Bucket-Policy and KMS Key Policy in a production environment. It is much too open for such a use-case. Refer to the AWS documentation here: https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_exportfindings.html
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty PublishingDestination using the master GuardDuty detector ID and PublishingDestinationID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/publishingDestination:PublishingDestination test a4b86f26fa42e7e7cf0d1c333ea77777:a4b86f27a0e464e4a7e0516d242f1234
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod publishing_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublishingDestinationArgs {
        /// The bucket arn and prefix under which the findings get exported. Bucket-ARN is required, the prefix is optional and will be `AWSLogs/[Account-ID]/GuardDuty/[Region]/` if not provided
        #[builder(into)]
        pub destination_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Currently there is only "S3" available as destination type which is also the default value
        ///
        /// > **Note:** In case of missing permissions (S3 Bucket Policy _or_ KMS Key permissions) the resource will fail to create. If the permissions are changed after resource creation, this can be asked from the AWS API via the "DescribePublishingDestination" call (https://docs.aws.amazon.com/cli/latest/reference/guardduty/describe-publishing-destination.html).
        #[builder(into, default)]
        pub destination_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The detector ID of the GuardDuty.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the KMS key used to encrypt GuardDuty findings. GuardDuty enforces this to be encrypted.
        #[builder(into)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PublishingDestinationResult {
        /// The bucket arn and prefix under which the findings get exported. Bucket-ARN is required, the prefix is optional and will be `AWSLogs/[Account-ID]/GuardDuty/[Region]/` if not provided
        pub destination_arn: pulumi_gestalt_rust::Output<String>,
        /// Currently there is only "S3" available as destination type which is also the default value
        ///
        /// > **Note:** In case of missing permissions (S3 Bucket Policy _or_ KMS Key permissions) the resource will fail to create. If the permissions are changed after resource creation, this can be asked from the AWS API via the "DescribePublishingDestination" call (https://docs.aws.amazon.com/cli/latest/reference/guardduty/describe-publishing-destination.html).
        pub destination_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The detector ID of the GuardDuty.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the KMS key used to encrypt GuardDuty findings. GuardDuty enforces this to be encrypted.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PublishingDestinationArgs,
    ) -> PublishingDestinationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_arn_binding = args.destination_arn.get_output(context);
        let destination_type_binding = args.destination_type.get_output(context);
        let detector_id_binding = args.detector_id.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:guardduty/publishingDestination:PublishingDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationArn".into(),
                    value: &destination_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationType".into(),
                    value: &destination_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PublishingDestinationResult {
            destination_arn: o.get_field("destinationArn"),
            destination_type: o.get_field("destinationType"),
            detector_id: o.get_field("detectorId"),
            kms_key_arn: o.get_field("kmsKeyArn"),
        }
    }
}
