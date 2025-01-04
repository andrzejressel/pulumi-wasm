/// The AMI resource allows the creation and management of a completely-custom
/// *Amazon Machine Image* (AMI).
///
/// If you just want to duplicate an existing AMI, possibly copying it to another
/// region, it's better to use `aws.ec2.AmiCopy` instead.
///
/// If you just want to share an existing AMI with another AWS account,
/// it's better to use `aws.ec2.AmiLaunchPermission` instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami::create(
///         "example",
///         AmiArgs::builder()
///             .ebs_block_devices(
///                 vec![
///                     AmiEbsBlockDevice::builder().deviceName("/dev/xvda")
///                     .snapshotId("snap-xxxxxxxx").volumeSize(8).build_struct(),
///                 ],
///             )
///             .imds_support("v2.0")
///             .name("example")
///             .root_device_name("/dev/xvda")
///             .virtualization_type("hvm")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ami` using the ID of the AMI. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/ami:Ami example ami-12345678
/// ```
pub mod ami {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiArgs {
        /// Machine architecture for created instances. Defaults to "x86_64".
        #[builder(into, default)]
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// Boot mode of the AMI. For more information, see [Boot modes](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html) in the Amazon Elastic Compute Cloud User Guide.
        #[builder(into, default)]
        pub boot_mode: pulumi_wasm_rust::Output<Option<String>>,
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
            Option<Vec<super::super::types::ec2::AmiEbsBlockDevice>>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub ena_support: pulumi_wasm_rust::Output<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::AmiEphemeralBlockDevice>>,
        >,
        #[builder(into, default)]
        pub image_location: pulumi_wasm_rust::Output<Option<String>>,
        /// If EC2 instances started from this image should require the use of the Instance Metadata Service V2 (IMDSv2), set this argument to `v2.0`. For more information, see [Configure instance metadata options for new instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration).
        #[builder(into, default)]
        pub imds_support: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub kernel_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub ramdisk_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the root device (for example, `/dev/sda1`, or `/dev/xvda`).
        #[builder(into, default)]
        pub root_device_name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub sriov_net_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If the image is configured for NitroTPM support, the value is `v2.0`. For more information, see [NitroTPM](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html) in the Amazon Elastic Compute Cloud User Guide.
        #[builder(into, default)]
        pub tpm_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Keyword to choose what virtualization mode created instances
        /// will use. Can be either "paravirtual" (the default) or "hvm". The choice of virtualization type
        /// changes the set of further arguments that are required, as described below.
        #[builder(into, default)]
        pub virtualization_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AmiResult {
        /// Machine architecture for created instances. Defaults to "x86_64".
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the AMI.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Boot mode of the AMI. For more information, see [Boot modes](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html) in the Amazon Elastic Compute Cloud User Guide.
        pub boot_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub deprecation_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Longer, human-readable description for the AMI.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::AmiEbsBlockDevice>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        pub ena_support: pulumi_wasm_rust::Output<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::AmiEphemeralBlockDevice>,
        >,
        /// Hypervisor type of the image.
        pub hypervisor: pulumi_wasm_rust::Output<String>,
        pub image_location: pulumi_wasm_rust::Output<String>,
        /// AWS account alias (for example, amazon, self) or the AWS account ID of the AMI owner.
        pub image_owner_alias: pulumi_wasm_rust::Output<String>,
        /// Type of image.
        pub image_type: pulumi_wasm_rust::Output<String>,
        /// If EC2 instances started from this image should require the use of the Instance Metadata Service V2 (IMDSv2), set this argument to `v2.0`. For more information, see [Configure instance metadata options for new instances](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration).
        pub imds_support: pulumi_wasm_rust::Output<Option<String>>,
        pub kernel_id: pulumi_wasm_rust::Output<Option<String>>,
        pub manage_ebs_snapshots: pulumi_wasm_rust::Output<bool>,
        /// Region-unique name for the AMI.
        pub name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the image owner.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// This value is set to windows for Windows AMIs; otherwise, it is blank.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Platform details associated with the billing code of the AMI.
        pub platform_details: pulumi_wasm_rust::Output<String>,
        /// Whether the image has public launch permissions.
        pub public: pulumi_wasm_rust::Output<bool>,
        pub ramdisk_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the root device (for example, `/dev/sda1`, or `/dev/xvda`).
        pub root_device_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Snapshot ID for the root volume (for EBS-backed AMIs)
        pub root_snapshot_id: pulumi_wasm_rust::Output<String>,
        pub sriov_net_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If the image is configured for NitroTPM support, the value is `v2.0`. For more information, see [NitroTPM](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html) in the Amazon Elastic Compute Cloud User Guide.
        pub tpm_support: pulumi_wasm_rust::Output<Option<String>>,
        /// Operation of the Amazon EC2 instance and the billing code that is associated with the AMI.
        pub usage_operation: pulumi_wasm_rust::Output<String>,
        /// Keyword to choose what virtualization mode created instances
        /// will use. Can be either "paravirtual" (the default) or "hvm". The choice of virtualization type
        /// changes the set of further arguments that are required, as described below.
        pub virtualization_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AmiArgs) -> AmiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let architecture_binding = args.architecture.get_inner();
        let boot_mode_binding = args.boot_mode.get_inner();
        let deprecation_time_binding = args.deprecation_time.get_inner();
        let description_binding = args.description.get_inner();
        let ebs_block_devices_binding = args.ebs_block_devices.get_inner();
        let ena_support_binding = args.ena_support.get_inner();
        let ephemeral_block_devices_binding = args.ephemeral_block_devices.get_inner();
        let image_location_binding = args.image_location.get_inner();
        let imds_support_binding = args.imds_support.get_inner();
        let kernel_id_binding = args.kernel_id.get_inner();
        let name_binding = args.name.get_inner();
        let ramdisk_id_binding = args.ramdisk_id.get_inner();
        let root_device_name_binding = args.root_device_name.get_inner();
        let sriov_net_support_binding = args.sriov_net_support.get_inner();
        let tags_binding = args.tags.get_inner();
        let tpm_support_binding = args.tpm_support.get_inner();
        let virtualization_type_binding = args.virtualization_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/ami:Ami".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "architecture".into(),
                    value: &architecture_binding,
                },
                register_interface::ObjectField {
                    name: "bootMode".into(),
                    value: &boot_mode_binding,
                },
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
                    name: "enaSupport".into(),
                    value: &ena_support_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "imageLocation".into(),
                    value: &image_location_binding,
                },
                register_interface::ObjectField {
                    name: "imdsSupport".into(),
                    value: &imds_support_binding,
                },
                register_interface::ObjectField {
                    name: "kernelId".into(),
                    value: &kernel_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ramdiskId".into(),
                    value: &ramdisk_id_binding,
                },
                register_interface::ObjectField {
                    name: "rootDeviceName".into(),
                    value: &root_device_name_binding,
                },
                register_interface::ObjectField {
                    name: "sriovNetSupport".into(),
                    value: &sriov_net_support_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tpmSupport".into(),
                    value: &tpm_support_binding,
                },
                register_interface::ObjectField {
                    name: "virtualizationType".into(),
                    value: &virtualization_type_binding,
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
        AmiResult {
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
