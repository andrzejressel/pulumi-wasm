/// The "AMI copy" resource allows duplication of an Amazon Machine Image (AMI),
/// including cross-region copies.
///
/// If the source AMI has associated EBS snapshots, those will also be duplicated
/// along with the AMI.
///
/// This is useful for taking a single AMI provisioned in one region and making
/// it available in another for a multi-region deployment.
///
/// Copying an AMI can take several minutes. The creation of this resource will
/// block until the new AMI is available for use on new instances.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:AmiCopy
///     properties:
///       name: example
///       sourceAmiId: ami-xxxxxxxx
///       sourceAmiRegion: us-west-1
///       tags:
///         Name: HelloWorld
/// ```
pub mod ami_copy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiCopyArgs {
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub deprecation_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Longer, human-readable description for the AMI.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the Outpost to which to copy the AMI.
        /// Only specify this parameter when copying an AMI from an AWS Region to an Outpost. The AMI must be in the Region of the destination Outpost.
        #[builder(into, default)]
        pub destination_outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::AmiCopyEbsBlockDevice>>,
        >,
        /// Whether the destination snapshots of the copied image should be encrypted. Defaults to `false`
        #[builder(into, default)]
        pub encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::AmiCopyEphemeralBlockDevice>>,
        >,
        /// Full ARN of the KMS Key to use when encrypting the snapshots of an image during a copy operation. If not specified, then the default AWS KMS Key will be used
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Id of the AMI to copy. This id must be valid in the region
        /// given by `source_ami_region`.
        #[builder(into)]
        pub source_ami_id: pulumi_wasm_rust::Output<String>,
        /// Region from which the AMI will be copied. This may be the
        /// same as the AWS provider region in order to create a copy within the same region.
        #[builder(into)]
        pub source_ami_region: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AmiCopyResult {
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
        /// ARN of the Outpost to which to copy the AMI.
        /// Only specify this parameter when copying an AMI from an AWS Region to an Outpost. The AMI must be in the Region of the destination Outpost.
        pub destination_outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::AmiCopyEbsBlockDevice>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        pub ena_support: pulumi_wasm_rust::Output<bool>,
        /// Whether the destination snapshots of the copied image should be encrypted. Defaults to `false`
        pub encrypted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::AmiCopyEphemeralBlockDevice>,
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
        /// Full ARN of the KMS Key to use when encrypting the snapshots of an image during a copy operation. If not specified, then the default AWS KMS Key will be used
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
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
        /// Id of the AMI to copy. This id must be valid in the region
        /// given by `source_ami_region`.
        pub source_ami_id: pulumi_wasm_rust::Output<String>,
        /// Region from which the AMI will be copied. This may be the
        /// same as the AWS provider region in order to create a copy within the same region.
        pub source_ami_region: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: AmiCopyArgs) -> AmiCopyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deprecation_time_binding = args.deprecation_time.get_inner();
        let description_binding = args.description.get_inner();
        let destination_outpost_arn_binding = args.destination_outpost_arn.get_inner();
        let ebs_block_devices_binding = args.ebs_block_devices.get_inner();
        let encrypted_binding = args.encrypted.get_inner();
        let ephemeral_block_devices_binding = args.ephemeral_block_devices.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let name_binding = args.name.get_inner();
        let source_ami_id_binding = args.source_ami_id.get_inner();
        let source_ami_region_binding = args.source_ami_region.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/amiCopy:AmiCopy".into(),
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
                    name: "destinationOutpostArn".into(),
                    value: &destination_outpost_arn_binding,
                },
                register_interface::ObjectField {
                    name: "ebsBlockDevices".into(),
                    value: &ebs_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "encrypted".into(),
                    value: &encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding,
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
                    name: "sourceAmiId".into(),
                    value: &source_ami_id_binding,
                },
                register_interface::ObjectField {
                    name: "sourceAmiRegion".into(),
                    value: &source_ami_region_binding,
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
                    name: "destinationOutpostArn".into(),
                },
                register_interface::ResultField {
                    name: "ebsBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "enaSupport".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
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
                    name: "kmsKeyId".into(),
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
                    name: "sourceAmiId".into(),
                },
                register_interface::ResultField {
                    name: "sourceAmiRegion".into(),
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
        AmiCopyResult {
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
            destination_outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationOutpostArn").unwrap(),
            ),
            ebs_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsBlockDevices").unwrap(),
            ),
            ena_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enaSupport").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
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
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
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
            source_ami_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceAmiId").unwrap(),
            ),
            source_ami_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceAmiRegion").unwrap(),
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
