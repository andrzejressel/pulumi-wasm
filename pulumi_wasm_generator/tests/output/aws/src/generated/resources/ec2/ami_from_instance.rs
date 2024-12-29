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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiFromInstanceArgs {
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub deprecation_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Longer, human-readable description for the AMI.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::AmiFromInstanceEbsBlockDevice>>,
        >,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::AmiFromInstanceEphemeralBlockDevice>>,
        >,
        /// Region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean that overrides the behavior of stopping
        /// the instance before snapshotting. This is risky since it may cause a snapshot of an
        /// inconsistent filesystem state, but can be used to avoid downtime if the user otherwise
        /// guarantees that no filesystem writes will be underway at the time of snapshot.
        #[builder(into, default)]
        pub snapshot_without_reboot: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the instance to use as the basis of the AMI.
        #[builder(into)]
        pub source_instance_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn create(name: &str, args: AmiFromInstanceArgs) -> AmiFromInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deprecation_time_binding = args.deprecation_time.get_inner();
        let description_binding = args.description.get_inner();
        let ebs_block_devices_binding = args.ebs_block_devices.get_inner();
        let ephemeral_block_devices_binding = args.ephemeral_block_devices.get_inner();
        let name_binding = args.name.get_inner();
        let snapshot_without_reboot_binding = args.snapshot_without_reboot.get_inner();
        let source_instance_id_binding = args.source_instance_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/amiFromInstance:AmiFromInstance".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "architecture".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bootMode".into(),
                },
                register_interface::ResultField {
                    name: "deprecationTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "ebsBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "enaSupport".into(),
                },
                register_interface::ResultField {
                    name: "ephemeralBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "hypervisor".into(),
                },
                register_interface::ResultField {
                    name: "imageLocation".into(),
                },
                register_interface::ResultField {
                    name: "imageOwnerAlias".into(),
                },
                register_interface::ResultField {
                    name: "imageType".into(),
                },
                register_interface::ResultField {
                    name: "imdsSupport".into(),
                },
                register_interface::ResultField {
                    name: "kernelId".into(),
                },
                register_interface::ResultField {
                    name: "manageEbsSnapshots".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "platformDetails".into(),
                },
                register_interface::ResultField {
                    name: "public".into(),
                },
                register_interface::ResultField {
                    name: "ramdiskId".into(),
                },
                register_interface::ResultField {
                    name: "rootDeviceName".into(),
                },
                register_interface::ResultField {
                    name: "rootSnapshotId".into(),
                },
                register_interface::ResultField {
                    name: "snapshotWithoutReboot".into(),
                },
                register_interface::ResultField {
                    name: "sourceInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "sriovNetSupport".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tpmSupport".into(),
                },
                register_interface::ResultField {
                    name: "usageOperation".into(),
                },
                register_interface::ResultField {
                    name: "virtualizationType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AmiFromInstanceResult {
            architecture: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("architecture").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            boot_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootMode").unwrap(),
            ),
            deprecation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deprecationTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            ebs_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsBlockDevices").unwrap(),
            ),
            ena_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enaSupport").unwrap(),
            ),
            ephemeral_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralBlockDevices").unwrap(),
            ),
            hypervisor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hypervisor").unwrap(),
            ),
            image_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageLocation").unwrap(),
            ),
            image_owner_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageOwnerAlias").unwrap(),
            ),
            image_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageType").unwrap(),
            ),
            imds_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imdsSupport").unwrap(),
            ),
            kernel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kernelId").unwrap(),
            ),
            manage_ebs_snapshots: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manageEbsSnapshots").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            platform_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformDetails").unwrap(),
            ),
            public: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("public").unwrap(),
            ),
            ramdisk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ramdiskId").unwrap(),
            ),
            root_device_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootDeviceName").unwrap(),
            ),
            root_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootSnapshotId").unwrap(),
            ),
            snapshot_without_reboot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotWithoutReboot").unwrap(),
            ),
            source_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceInstanceId").unwrap(),
            ),
            sriov_net_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sriovNetSupport").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tpm_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tpmSupport").unwrap(),
            ),
            usage_operation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usageOperation").unwrap(),
            ),
            virtualization_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualizationType").unwrap(),
            ),
        }
    }
}
