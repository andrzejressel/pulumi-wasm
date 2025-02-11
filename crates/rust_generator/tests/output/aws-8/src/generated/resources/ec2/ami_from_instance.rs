/// The "AMI from instance" resource allows the creation of an Amazon Machine
/// Image (AMI) modeled after an existing EBS-backed EC2 instance.
///
/// The created AMI will refer to implicitly-created snapshots of the instance's
/// EBS volumes and mimick its assigned block device configuration at the time
/// the resource is created.
///
/// This resource is best applied to an instance that is stopped when this instance
/// is created, so that the contents of the created image are predictable. When
/// applied to an instance that is running, *the instance will be stopped before taking
/// the snapshots and then started back up again*, resulting in a period of
/// downtime.
///
/// Note that the source instance is inspected only at the initial creation of this
/// resource. Ongoing updates to the referenced instance will not be propagated into
/// the generated AMI. Users may taint or otherwise recreate the resource in order
/// to produce a fresh snapshot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami_from_instance::create(
///         "example",
///         AmiFromInstanceArgs::builder()
///             .name("example")
///             .source_instance_id("i-xxxxxxxx")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ami_from_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiFromInstanceArgs {
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub deprecation_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Longer, human-readable description for the AMI.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiFromInstanceEbsBlockDevice>>,
        >,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiFromInstanceEphemeralBlockDevice>>,
        >,
        /// Region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean that overrides the behavior of stopping
        /// the instance before snapshotting. This is risky since it may cause a snapshot of an
        /// inconsistent filesystem state, but can be used to avoid downtime if the user otherwise
        /// guarantees that no filesystem writes will be underway at the time of snapshot.
        #[builder(into, default)]
        pub snapshot_without_reboot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ID of the instance to use as the basis of the AMI.
        #[builder(into)]
        pub source_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AmiFromInstanceResult {
        /// Machine architecture for created instances. Defaults to "x86_64".
        pub architecture: pulumi_gestalt_rust::Output<String>,
        /// ARN of the AMI.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Boot mode of the AMI. For more information, see [Boot modes](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html) in the Amazon Elastic Compute Cloud User Guide.
        pub boot_mode: pulumi_gestalt_rust::Output<String>,
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub deprecation_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Longer, human-readable description for the AMI.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        pub ebs_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::AmiFromInstanceEbsBlockDevice>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        pub ena_support: pulumi_gestalt_rust::Output<bool>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        pub ephemeral_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::AmiFromInstanceEphemeralBlockDevice>,
        >,
        pub hypervisor: pulumi_gestalt_rust::Output<String>,
        /// Path to an S3 object containing an image manifest, e.g., created
        /// by the `ec2-upload-bundle` command in the EC2 command line tools.
        pub image_location: pulumi_gestalt_rust::Output<String>,
        pub image_owner_alias: pulumi_gestalt_rust::Output<String>,
        pub image_type: pulumi_gestalt_rust::Output<String>,
        /// If EC2 instances started from this image should require the use of the Instance Metadata Service V2 (IMDSv2), set this argument to `v2.0`. For more information, see [Configure instance metadata options for new instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration).
        pub imds_support: pulumi_gestalt_rust::Output<String>,
        /// ID of the kernel image (AKI) that will be used as the paravirtual
        /// kernel in created instances.
        pub kernel_id: pulumi_gestalt_rust::Output<String>,
        pub manage_ebs_snapshots: pulumi_gestalt_rust::Output<bool>,
        /// Region-unique name for the AMI.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub platform: pulumi_gestalt_rust::Output<String>,
        pub platform_details: pulumi_gestalt_rust::Output<String>,
        pub public: pulumi_gestalt_rust::Output<bool>,
        /// ID of an initrd image (ARI) that will be used when booting the
        /// created instances.
        pub ramdisk_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the root device (for example, `/dev/sda1`, or `/dev/xvda`).
        pub root_device_name: pulumi_gestalt_rust::Output<String>,
        pub root_snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// Boolean that overrides the behavior of stopping
        /// the instance before snapshotting. This is risky since it may cause a snapshot of an
        /// inconsistent filesystem state, but can be used to avoid downtime if the user otherwise
        /// guarantees that no filesystem writes will be underway at the time of snapshot.
        pub snapshot_without_reboot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ID of the instance to use as the basis of the AMI.
        pub source_instance_id: pulumi_gestalt_rust::Output<String>,
        /// When set to "simple" (the default), enables enhanced networking
        /// for created instances. No other value is supported at this time.
        pub sriov_net_support: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If the image is configured for NitroTPM support, the value is `v2.0`. For more information, see [NitroTPM](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html) in the Amazon Elastic Compute Cloud User Guide.
        pub tpm_support: pulumi_gestalt_rust::Output<String>,
        pub usage_operation: pulumi_gestalt_rust::Output<String>,
        /// Keyword to choose what virtualization mode created instances
        /// will use. Can be either "paravirtual" (the default) or "hvm". The choice of virtualization type
        /// changes the set of further arguments that are required, as described below.
        pub virtualization_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AmiFromInstanceArgs,
    ) -> AmiFromInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deprecation_time_binding = args.deprecation_time.get_output(context);
        let description_binding = args.description.get_output(context);
        let ebs_block_devices_binding = args.ebs_block_devices.get_output(context);
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let snapshot_without_reboot_binding = args
            .snapshot_without_reboot
            .get_output(context);
        let source_instance_id_binding = args.source_instance_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/amiFromInstance:AmiFromInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deprecationTime".into(),
                    value: &deprecation_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsBlockDevices".into(),
                    value: &ebs_block_devices_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotWithoutReboot".into(),
                    value: &snapshot_without_reboot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceInstanceId".into(),
                    value: &source_instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AmiFromInstanceResult {
            architecture: o.get_field("architecture"),
            arn: o.get_field("arn"),
            boot_mode: o.get_field("bootMode"),
            deprecation_time: o.get_field("deprecationTime"),
            description: o.get_field("description"),
            ebs_block_devices: o.get_field("ebsBlockDevices"),
            ena_support: o.get_field("enaSupport"),
            ephemeral_block_devices: o.get_field("ephemeralBlockDevices"),
            hypervisor: o.get_field("hypervisor"),
            image_location: o.get_field("imageLocation"),
            image_owner_alias: o.get_field("imageOwnerAlias"),
            image_type: o.get_field("imageType"),
            imds_support: o.get_field("imdsSupport"),
            kernel_id: o.get_field("kernelId"),
            manage_ebs_snapshots: o.get_field("manageEbsSnapshots"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            platform: o.get_field("platform"),
            platform_details: o.get_field("platformDetails"),
            public: o.get_field("public"),
            ramdisk_id: o.get_field("ramdiskId"),
            root_device_name: o.get_field("rootDeviceName"),
            root_snapshot_id: o.get_field("rootSnapshotId"),
            snapshot_without_reboot: o.get_field("snapshotWithoutReboot"),
            source_instance_id: o.get_field("sourceInstanceId"),
            sriov_net_support: o.get_field("sriovNetSupport"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tpm_support: o.get_field("tpmSupport"),
            usage_operation: o.get_field("usageOperation"),
            virtualization_type: o.get_field("virtualizationType"),
        }
    }
}
