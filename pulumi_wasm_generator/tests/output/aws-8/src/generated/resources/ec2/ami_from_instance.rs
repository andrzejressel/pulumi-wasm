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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod ami_from_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiFromInstanceArgs {
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub deprecation_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Longer, human-readable description for the AMI.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiFromInstanceEbsBlockDevice>>,
        >,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiFromInstanceEphemeralBlockDevice>>,
        >,
        /// Region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Boolean that overrides the behavior of stopping
        /// the instance before snapshotting. This is risky since it may cause a snapshot of an
        /// inconsistent filesystem state, but can be used to avoid downtime if the user otherwise
        /// guarantees that no filesystem writes will be underway at the time of snapshot.
        #[builder(into, default)]
        pub snapshot_without_reboot: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// ID of the instance to use as the basis of the AMI.
        #[builder(into)]
        pub source_instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AmiFromInstanceResult {
        /// Machine architecture for created instances. Defaults to "x86_64".
        pub architecture: pulumi_wasm_rust::Output<String>,
        /// ARN of the AMI.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Boot mode of the AMI. For more information, see [Boot modes](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html) in the Amazon Elastic Compute Cloud User Guide.
        pub boot_mode: pulumi_wasm_rust::Output<String>,
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub deprecation_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Longer, human-readable description for the AMI.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::AmiFromInstanceEbsBlockDevice>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        pub ena_support: pulumi_wasm_rust::Output<bool>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::AmiFromInstanceEphemeralBlockDevice>,
        >,
        pub hypervisor: pulumi_wasm_rust::Output<String>,
        /// Path to an S3 object containing an image manifest, e.g., created
        /// by the `ec2-upload-bundle` command in the EC2 command line tools.
        pub image_location: pulumi_wasm_rust::Output<String>,
        pub image_owner_alias: pulumi_wasm_rust::Output<String>,
        pub image_type: pulumi_wasm_rust::Output<String>,
        /// If EC2 instances started from this image should require the use of the Instance Metadata Service V2 (IMDSv2), set this argument to `v2.0`. For more information, see [Configure instance metadata options for new instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration).
        pub imds_support: pulumi_wasm_rust::Output<String>,
        /// ID of the kernel image (AKI) that will be used as the paravirtual
        /// kernel in created instances.
        pub kernel_id: pulumi_wasm_rust::Output<String>,
        pub manage_ebs_snapshots: pulumi_wasm_rust::Output<bool>,
        /// Region-unique name for the AMI.
        pub name: pulumi_wasm_rust::Output<String>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub platform: pulumi_wasm_rust::Output<String>,
        pub platform_details: pulumi_wasm_rust::Output<String>,
        pub public: pulumi_wasm_rust::Output<bool>,
        /// ID of an initrd image (ARI) that will be used when booting the
        /// created instances.
        pub ramdisk_id: pulumi_wasm_rust::Output<String>,
        /// Name of the root device (for example, `/dev/sda1`, or `/dev/xvda`).
        pub root_device_name: pulumi_wasm_rust::Output<String>,
        pub root_snapshot_id: pulumi_wasm_rust::Output<String>,
        /// Boolean that overrides the behavior of stopping
        /// the instance before snapshotting. This is risky since it may cause a snapshot of an
        /// inconsistent filesystem state, but can be used to avoid downtime if the user otherwise
        /// guarantees that no filesystem writes will be underway at the time of snapshot.
        pub snapshot_without_reboot: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the instance to use as the basis of the AMI.
        pub source_instance_id: pulumi_wasm_rust::Output<String>,
        /// When set to "simple" (the default), enables enhanced networking
        /// for created instances. No other value is supported at this time.
        pub sriov_net_support: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If the image is configured for NitroTPM support, the value is `v2.0`. For more information, see [NitroTPM](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html) in the Amazon Elastic Compute Cloud User Guide.
        pub tpm_support: pulumi_wasm_rust::Output<String>,
        pub usage_operation: pulumi_wasm_rust::Output<String>,
        /// Keyword to choose what virtualization mode created instances
        /// will use. Can be either "paravirtual" (the default) or "hvm". The choice of virtualization type
        /// changes the set of further arguments that are required, as described below.
        pub virtualization_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AmiFromInstanceArgs,
    ) -> AmiFromInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deprecation_time_binding = args
            .deprecation_time
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let ebs_block_devices_binding = args
            .ebs_block_devices
            .get_output(context)
            .get_inner();
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let snapshot_without_reboot_binding = args
            .snapshot_without_reboot
            .get_output(context)
            .get_inner();
        let source_instance_id_binding = args
            .source_instance_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/amiFromInstance:AmiFromInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deprecationTime".into(),
                    value: &deprecation_time_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ebsBlockDevices".into(),
                    value: &ebs_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotWithoutReboot".into(),
                    value: &snapshot_without_reboot_binding,
                },
                register_interface::ObjectField {
                    name: "sourceInstanceId".into(),
                    value: &source_instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AmiFromInstanceResult {
            architecture: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("architecture"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            boot_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bootMode"),
            ),
            deprecation_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deprecationTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            ebs_block_devices: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ebsBlockDevices"),
            ),
            ena_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enaSupport"),
            ),
            ephemeral_block_devices: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ephemeralBlockDevices"),
            ),
            hypervisor: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hypervisor"),
            ),
            image_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageLocation"),
            ),
            image_owner_alias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageOwnerAlias"),
            ),
            image_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageType"),
            ),
            imds_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imdsSupport"),
            ),
            kernel_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kernelId"),
            ),
            manage_ebs_snapshots: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("manageEbsSnapshots"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            platform_details: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformDetails"),
            ),
            public: pulumi_wasm_rust::__private::into_domain(o.extract_field("public")),
            ramdisk_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ramdiskId"),
            ),
            root_device_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rootDeviceName"),
            ),
            root_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rootSnapshotId"),
            ),
            snapshot_without_reboot: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotWithoutReboot"),
            ),
            source_instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceInstanceId"),
            ),
            sriov_net_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sriovNetSupport"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tpm_support: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tpmSupport"),
            ),
            usage_operation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("usageOperation"),
            ),
            virtualization_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualizationType"),
            ),
        }
    }
}
