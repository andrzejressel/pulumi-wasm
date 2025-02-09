#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeArgs {
        /// One or more name/value pairs to filter off of. There are
        /// several valid keys, for a full reference, check out
        /// [describe-volumes in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ebs::GetVolumeFilter>>,
        >,
        /// If more than one result is returned, use the most
        /// recent Volume.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVolumeResult {
        /// Volume ARN (e.g., arn:aws:ec2:us-east-1:123456789012:volume/vol-59fcb34e).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AZ where the EBS volume exists.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Whether the disk is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ebs::GetVolumeFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Amount of IOPS for the disk.
        pub iops: pulumi_gestalt_rust::Output<i32>,
        /// ARN for the KMS encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// (Optional) Specifies whether Amazon EBS Multi-Attach is enabled.
        pub multi_attach_enabled: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the Outpost.
        pub outpost_arn: pulumi_gestalt_rust::Output<String>,
        /// Size of the drive in GiBs.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// Snapshot_id the EBS volume is based off.
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Throughput that the volume supports, in MiB/s.
        pub throughput: pulumi_gestalt_rust::Output<i32>,
        /// Volume ID (e.g., vol-59fcb34e).
        pub volume_id: pulumi_gestalt_rust::Output<String>,
        /// Type of EBS volume.
        pub volume_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVolumeArgs,
    ) -> GetVolumeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ebs/getVolume:getVolume".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: most_recent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVolumeResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            encrypted: o.get_field("encrypted"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            iops: o.get_field("iops"),
            kms_key_id: o.get_field("kmsKeyId"),
            most_recent: o.get_field("mostRecent"),
            multi_attach_enabled: o.get_field("multiAttachEnabled"),
            outpost_arn: o.get_field("outpostArn"),
            size: o.get_field("size"),
            snapshot_id: o.get_field("snapshotId"),
            tags: o.get_field("tags"),
            throughput: o.get_field("throughput"),
            volume_id: o.get_field("volumeId"),
            volume_type: o.get_field("volumeType"),
        }
    }
}
