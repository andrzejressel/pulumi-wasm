/// Provides an AWS Config Delivery Channel.
///
/// > **Note:** Delivery Channel requires a Configuration Recorder to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:cfg:DeliveryChannel
///     properties:
///       name: example
///       s3BucketName: ${b.bucket}
///     options:
///       dependsOn:
///         - ${fooRecorder}
///   b:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example-awsconfig
///       forceDestroy: true
///   fooRecorder:
///     type: aws:cfg:Recorder
///     name: foo
///     properties:
///       name: example
///       roleArn: ${r.arn}
///   r:
///     type: aws:iam:Role
///     properties:
///       name: awsconfig-example
///       assumeRolePolicy: ${assumeRole.json}
///   pRolePolicy:
///     type: aws:iam:RolePolicy
///     name: p
///     properties:
///       name: awsconfig-example
///       role: ${r.id}
///       policy: ${p.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - config.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   p:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - s3:*
///             resources:
///               - ${b.arn}
///               - ${b.arn}/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Delivery Channel using the name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/deliveryChannel:DeliveryChannel foo example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod delivery_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeliveryChannelArgs {
        /// The name of the delivery channel. Defaults to `default`. Changing it recreates the resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the S3 bucket used to store the configuration history.
        #[builder(into)]
        pub s3_bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The prefix for the specified S3 bucket.
        #[builder(into, default)]
        pub s3_key_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the AWS KMS key used to encrypt objects delivered by AWS Config. Must belong to the same Region as the destination S3 bucket.
        #[builder(into, default)]
        pub s3_kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options for how AWS Config delivers configuration snapshots. See below
        #[builder(into, default)]
        pub snapshot_delivery_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cfg::DeliveryChannelSnapshotDeliveryProperties>,
        >,
        /// The ARN of the SNS topic that AWS Config delivers notifications to.
        #[builder(into, default)]
        pub sns_topic_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DeliveryChannelResult {
        /// The name of the delivery channel. Defaults to `default`. Changing it recreates the resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the S3 bucket used to store the configuration history.
        pub s3_bucket_name: pulumi_gestalt_rust::Output<String>,
        /// The prefix for the specified S3 bucket.
        pub s3_key_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the AWS KMS key used to encrypt objects delivered by AWS Config. Must belong to the same Region as the destination S3 bucket.
        pub s3_kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Options for how AWS Config delivers configuration snapshots. See below
        pub snapshot_delivery_properties: pulumi_gestalt_rust::Output<
            Option<super::super::types::cfg::DeliveryChannelSnapshotDeliveryProperties>,
        >,
        /// The ARN of the SNS topic that AWS Config delivers notifications to.
        pub sns_topic_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeliveryChannelArgs,
    ) -> DeliveryChannelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let s3_bucket_name_binding = args.s3_bucket_name.get_output(context);
        let s3_key_prefix_binding = args.s3_key_prefix.get_output(context);
        let s3_kms_key_arn_binding = args.s3_kms_key_arn.get_output(context);
        let snapshot_delivery_properties_binding = args
            .snapshot_delivery_properties
            .get_output(context);
        let sns_topic_arn_binding = args.sns_topic_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/deliveryChannel:DeliveryChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3BucketName".into(),
                    value: s3_bucket_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3KeyPrefix".into(),
                    value: s3_key_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3KmsKeyArn".into(),
                    value: s3_kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotDeliveryProperties".into(),
                    value: snapshot_delivery_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snsTopicArn".into(),
                    value: sns_topic_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DeliveryChannelResult {
            name: o.get_field("name"),
            s3_bucket_name: o.get_field("s3BucketName"),
            s3_key_prefix: o.get_field("s3KeyPrefix"),
            s3_kms_key_arn: o.get_field("s3KmsKeyArn"),
            snapshot_delivery_properties: o.get_field("snapshotDeliveryProperties"),
            sns_topic_arn: o.get_field("snsTopicArn"),
        }
    }
}
