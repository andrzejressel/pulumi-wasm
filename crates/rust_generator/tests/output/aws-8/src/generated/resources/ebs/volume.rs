/// Manages a single EBS volume.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ebs:Volume
///     properties:
///       availabilityZone: us-west-2a
///       size: 40
///       tags:
///         Name: HelloWorld
/// ```
///
/// > **NOTE:** At least one of `size` or `snapshot_id` is required when specifying an EBS volume
///
/// ## Import
///
/// Using `pulumi import`, import EBS Volumes using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/volume:Volume id vol-049df61146c4d7901
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeArgs {
        /// The AZ where the EBS volume will exist.
        #[builder(into)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, the disk will be encrypted.
        #[builder(into, default)]
        pub encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, snapshot will be created before volume deletion. Any tags on the volume will be migrated to the snapshot. By default set to false
        #[builder(into, default)]
        pub final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The amount of IOPS to provision for the disk. Only valid for `type` of `io1`, `io2` or `gp3`.
        #[builder(into, default)]
        pub iops: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `encrypted` needs to be set to true. Note: The provider must be running with credentials which have the `GenerateDataKeyWithoutPlaintext` permission on the specified KMS key as required by the [EBS KMS CMK volume provisioning process](https://docs.aws.amazon.com/kms/latest/developerguide/services-ebs.html#ebs-cmk) to prevent a volume from being created and almost immediately deleted.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether to enable Amazon EBS Multi-Attach. Multi-Attach is supported on `io1` and `io2` volumes.
        #[builder(into, default)]
        pub multi_attach_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the Outpost.
        #[builder(into, default)]
        pub outpost_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The size of the drive in GiBs.
        #[builder(into, default)]
        pub size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A snapshot to base the EBS volume off of.
        #[builder(into, default)]
        pub snapshot_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The throughput that the volume supports, in MiB/s. Only valid for `type` of `gp3`.
        ///
        /// > **NOTE:** When changing the `size`, `iops` or `type` of an instance, there are [considerations](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/considerations.html) to be aware of.
        #[builder(into, default)]
        pub throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The type of EBS volume. Can be `standard`, `gp2`, `gp3`, `io1`, `io2`, `sc1` or `st1` (Default: `gp2`).
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeResult {
        /// The volume ARN (e.g., arn:aws:ec2:us-east-1:123456789012:volume/vol-59fcb34e).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AZ where the EBS volume will exist.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// If true, the disk will be encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        /// If true, snapshot will be created before volume deletion. Any tags on the volume will be migrated to the snapshot. By default set to false
        pub final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The amount of IOPS to provision for the disk. Only valid for `type` of `io1`, `io2` or `gp3`.
        pub iops: pulumi_gestalt_rust::Output<i32>,
        /// The ARN for the KMS encryption key. When specifying `kms_key_id`, `encrypted` needs to be set to true. Note: The provider must be running with credentials which have the `GenerateDataKeyWithoutPlaintext` permission on the specified KMS key as required by the [EBS KMS CMK volume provisioning process](https://docs.aws.amazon.com/kms/latest/developerguide/services-ebs.html#ebs-cmk) to prevent a volume from being created and almost immediately deleted.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to enable Amazon EBS Multi-Attach. Multi-Attach is supported on `io1` and `io2` volumes.
        pub multi_attach_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the Outpost.
        pub outpost_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The size of the drive in GiBs.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// A snapshot to base the EBS volume off of.
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The throughput that the volume supports, in MiB/s. Only valid for `type` of `gp3`.
        ///
        /// > **NOTE:** When changing the `size`, `iops` or `type` of an instance, there are [considerations](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/considerations.html) to be aware of.
        pub throughput: pulumi_gestalt_rust::Output<i32>,
        /// The type of EBS volume. Can be `standard`, `gp2`, `gp3`, `io1`, `io2`, `sc1` or `st1` (Default: `gp2`).
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VolumeArgs,
    ) -> VolumeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zone_binding = args.availability_zone.get_output(context);
        let encrypted_binding = args.encrypted.get_output(context);
        let final_snapshot_binding = args.final_snapshot.get_output(context);
        let iops_binding = args.iops.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let multi_attach_enabled_binding = args.multi_attach_enabled.get_output(context);
        let outpost_arn_binding = args.outpost_arn.get_output(context);
        let size_binding = args.size.get_output(context);
        let snapshot_id_binding = args.snapshot_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let throughput_binding = args.throughput.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/volume:Volume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encrypted".into(),
                    value: &encrypted_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "finalSnapshot".into(),
                    value: &final_snapshot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iops".into(),
                    value: &iops_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiAttachEnabled".into(),
                    value: &multi_attach_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outpostArn".into(),
                    value: &outpost_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "size".into(),
                    value: &size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotId".into(),
                    value: &snapshot_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughput".into(),
                    value: &throughput_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VolumeResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            encrypted: o.get_field("encrypted"),
            final_snapshot: o.get_field("finalSnapshot"),
            iops: o.get_field("iops"),
            kms_key_id: o.get_field("kmsKeyId"),
            multi_attach_enabled: o.get_field("multiAttachEnabled"),
            outpost_arn: o.get_field("outpostArn"),
            size: o.get_field("size"),
            snapshot_id: o.get_field("snapshotId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            throughput: o.get_field("throughput"),
            type_: o.get_field("type"),
        }
    }
}
