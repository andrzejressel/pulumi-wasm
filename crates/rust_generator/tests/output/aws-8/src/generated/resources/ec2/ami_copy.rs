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
#[allow(clippy::doc_lazy_continuation)]
pub mod ami_copy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiCopyArgs {
        /// Date and time to deprecate the AMI. If you specified a value for seconds, Amazon EC2 rounds the seconds to the nearest minute. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub deprecation_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Longer, human-readable description for the AMI.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the Outpost to which to copy the AMI.
        /// Only specify this parameter when copying an AMI from an AWS Region to an Outpost. The AMI must be in the Region of the destination Outpost.
        #[builder(into, default)]
        pub destination_outpost_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiCopyEbsBlockDevice>>,
        >,
        /// Whether the destination snapshots of the copied image should be encrypted. Defaults to `false`
        #[builder(into, default)]
        pub encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::AmiCopyEphemeralBlockDevice>>,
        >,
        /// Full ARN of the KMS Key to use when encrypting the snapshots of an image during a copy operation. If not specified, then the default AWS KMS Key will be used
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Id of the AMI to copy. This id must be valid in the region
        /// given by `source_ami_region`.
        #[builder(into)]
        pub source_ami_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Region from which the AMI will be copied. This may be the
        /// same as the AWS provider region in order to create a copy within the same region.
        #[builder(into)]
        pub source_ami_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AmiCopyResult {
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
        /// ARN of the Outpost to which to copy the AMI.
        /// Only specify this parameter when copying an AMI from an AWS Region to an Outpost. The AMI must be in the Region of the destination Outpost.
        pub destination_outpost_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Nested block describing an EBS block device that should be
        /// attached to created instances. The structure of this block is described below.
        pub ebs_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::AmiCopyEbsBlockDevice>,
        >,
        /// Whether enhanced networking with ENA is enabled. Defaults to `false`.
        pub ena_support: pulumi_gestalt_rust::Output<bool>,
        /// Whether the destination snapshots of the copied image should be encrypted. Defaults to `false`
        pub encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Nested block describing an ephemeral block device that
        /// should be attached to created instances. The structure of this block is described below.
        pub ephemeral_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::AmiCopyEphemeralBlockDevice>,
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
        /// Full ARN of the KMS Key to use when encrypting the snapshots of an image during a copy operation. If not specified, then the default AWS KMS Key will be used
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
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
        /// Id of the AMI to copy. This id must be valid in the region
        /// given by `source_ami_region`.
        pub source_ami_id: pulumi_gestalt_rust::Output<String>,
        /// Region from which the AMI will be copied. This may be the
        /// same as the AWS provider region in order to create a copy within the same region.
        pub source_ami_region: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AmiCopyArgs,
    ) -> AmiCopyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let deprecation_time_binding = args
            .deprecation_time
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let destination_outpost_arn_binding = args
            .destination_outpost_arn
            .get_output(context)
            .get_inner();
        let ebs_block_devices_binding = args
            .ebs_block_devices
            .get_output(context)
            .get_inner();
        let encrypted_binding = args.encrypted.get_output(context).get_inner();
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let source_ami_id_binding = args.source_ami_id.get_output(context).get_inner();
        let source_ami_region_binding = args
            .source_ami_region
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/amiCopy:AmiCopy".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AmiCopyResult {
            architecture: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("architecture"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            boot_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootMode"),
            ),
            deprecation_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deprecationTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destination_outpost_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationOutpostArn"),
            ),
            ebs_block_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ebsBlockDevices"),
            ),
            ena_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enaSupport"),
            ),
            encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            ephemeral_block_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ephemeralBlockDevices"),
            ),
            hypervisor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hypervisor"),
            ),
            image_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageLocation"),
            ),
            image_owner_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageOwnerAlias"),
            ),
            image_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageType"),
            ),
            imds_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imdsSupport"),
            ),
            kernel_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kernelId"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            manage_ebs_snapshots: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("manageEbsSnapshots"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            platform_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformDetails"),
            ),
            public: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("public"),
            ),
            ramdisk_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ramdiskId"),
            ),
            root_device_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootDeviceName"),
            ),
            root_snapshot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootSnapshotId"),
            ),
            source_ami_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceAmiId"),
            ),
            source_ami_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceAmiRegion"),
            ),
            sriov_net_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sriovNetSupport"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tpm_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tpmSupport"),
            ),
            usage_operation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("usageOperation"),
            ),
            virtualization_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualizationType"),
            ),
        }
    }
}
