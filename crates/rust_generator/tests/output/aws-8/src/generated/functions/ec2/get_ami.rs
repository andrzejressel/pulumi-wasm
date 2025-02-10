#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ami {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAmiArgs {
        /// Limit search to users with *explicit* launch permission on
        /// the image. Valid items are the numeric account ID or `self`.
        #[builder(into, default)]
        pub executable_users: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more name/value pairs to filter off of. There are
        /// several valid keys, for a full reference, check out
        /// [describe-images in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetAmiFilter>>,
        >,
        /// If true, all deprecated AMIs are included in the response. If false, no deprecated AMIs are included in the response. If no value is specified, the default value is false.
        #[builder(into, default)]
        pub include_deprecated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If more than one result is returned, use the most
        /// recent AMI.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Regex string to apply to the AMI list returned
        /// by AWS. This allows more advanced filtering not supported from the AWS API. This
        /// filtering is done locally on what AWS returns, and could have a performance
        /// impact if the result is large. Combine this with other
        /// options to narrow down the list AWS returns.
        ///
        /// > **NOTE:** If more or less than a single match is returned by the search,
        /// this call will fail. Ensure that your search is specific enough to return
        /// a single AMI ID only, or use `most_recent` to choose the most recent one. If
        /// you want to match multiple AMIs, use the `aws.ec2.getAmiIds` data source instead.
        #[builder(into, default)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of AMI owners to limit search. Valid values: an AWS account ID, `self` (the current account), or an AWS owner alias (e.g., `amazon`, `aws-marketplace`, `microsoft`).
        #[builder(into, default)]
        pub owners: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Any tags assigned to the image.
        /// * `tags.#.key` - Key name of the tag.
        /// * `tags.#.value` - Value of the tag.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAmiResult {
        /// OS architecture of the AMI (ie: `i386` or `x86_64`).
        pub architecture: pulumi_gestalt_rust::Output<String>,
        /// ARN of the AMI.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Set of objects with block device mappings of the AMI.
        pub block_device_mappings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetAmiBlockDeviceMapping>,
        >,
        /// Boot mode of the image.
        pub boot_mode: pulumi_gestalt_rust::Output<String>,
        /// Date and time the image was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the image will be deprecated.
        pub deprecation_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the AMI that was provided during image
        /// creation.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether enhanced networking with ENA is enabled.
        pub ena_support: pulumi_gestalt_rust::Output<bool>,
        pub executable_users: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetAmiFilter>>,
        >,
        /// Hypervisor type of the image.
        pub hypervisor: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the AMI. Should be the same as the resource `id`.
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// Location of the AMI.
        pub image_location: pulumi_gestalt_rust::Output<String>,
        /// AWS account alias (for example, `amazon`, `self`) or
        /// the AWS account ID of the AMI owner.
        pub image_owner_alias: pulumi_gestalt_rust::Output<String>,
        /// Type of image.
        pub image_type: pulumi_gestalt_rust::Output<String>,
        /// Instance Metadata Service (IMDS) support mode for the image. Set to `v2.0` if instances ran from this image enforce IMDSv2.
        pub imds_support: pulumi_gestalt_rust::Output<String>,
        pub include_deprecated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Kernel associated with the image, if any. Only applicable
        /// for machine images.
        pub kernel_id: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the AMI that was provided during image creation.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_regex: pulumi_gestalt_rust::Output<Option<String>>,
        /// AWS account ID of the image owner.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub owners: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Value is Windows for `Windows` AMIs; otherwise blank.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Platform details associated with the billing code of the AMI.
        pub platform_details: pulumi_gestalt_rust::Output<String>,
        /// Any product codes associated with the AMI.
        /// * `product_codes.#.product_code_id` - The product code.
        /// * `product_codes.#.product_code_type` - The type of product code.
        pub product_codes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetAmiProductCode>,
        >,
        /// `true` if the image has public launch permissions.
        pub public: pulumi_gestalt_rust::Output<bool>,
        /// RAM disk associated with the image, if any. Only applicable
        /// for machine images.
        pub ramdisk_id: pulumi_gestalt_rust::Output<String>,
        /// Device name of the root device.
        pub root_device_name: pulumi_gestalt_rust::Output<String>,
        /// Type of root device (ie: `ebs` or `instance-store`).
        pub root_device_type: pulumi_gestalt_rust::Output<String>,
        /// Snapshot id associated with the root device, if any
        /// (only applies to `ebs` root devices).
        pub root_snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// Whether enhanced networking is enabled.
        pub sriov_net_support: pulumi_gestalt_rust::Output<String>,
        /// Current state of the AMI. If the state is `available`, the image
        /// is successfully registered and can be used to launch an instance.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Describes a state change. Fields are `UNSET` if not available.
        pub state_reason: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Any tags assigned to the image.
        /// * `tags.#.key` - Key name of the tag.
        /// * `tags.#.value` - Value of the tag.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// If the image is configured for NitroTPM support, the value is `v2.0`.
        pub tpm_support: pulumi_gestalt_rust::Output<String>,
        /// Operation of the Amazon EC2 instance and the billing code that is associated with the AMI.
        pub usage_operation: pulumi_gestalt_rust::Output<String>,
        /// Type of virtualization of the AMI (ie: `hvm` or
        /// `paravirtual`).
        pub virtualization_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAmiArgs,
    ) -> GetAmiResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let executable_users_binding = args.executable_users.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let include_deprecated_binding = args.include_deprecated.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let name_regex_binding = args.name_regex.get_output(context);
        let owners_binding = args.owners.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getAmi:getAmi".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executableUsers".into(),
                    value: executable_users_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeDeprecated".into(),
                    value: include_deprecated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: most_recent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nameRegex".into(),
                    value: name_regex_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owners".into(),
                    value: owners_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAmiResult {
            architecture: o.get_field("architecture"),
            arn: o.get_field("arn"),
            block_device_mappings: o.get_field("blockDeviceMappings"),
            boot_mode: o.get_field("bootMode"),
            creation_date: o.get_field("creationDate"),
            deprecation_time: o.get_field("deprecationTime"),
            description: o.get_field("description"),
            ena_support: o.get_field("enaSupport"),
            executable_users: o.get_field("executableUsers"),
            filters: o.get_field("filters"),
            hypervisor: o.get_field("hypervisor"),
            id: o.get_field("id"),
            image_id: o.get_field("imageId"),
            image_location: o.get_field("imageLocation"),
            image_owner_alias: o.get_field("imageOwnerAlias"),
            image_type: o.get_field("imageType"),
            imds_support: o.get_field("imdsSupport"),
            include_deprecated: o.get_field("includeDeprecated"),
            kernel_id: o.get_field("kernelId"),
            most_recent: o.get_field("mostRecent"),
            name: o.get_field("name"),
            name_regex: o.get_field("nameRegex"),
            owner_id: o.get_field("ownerId"),
            owners: o.get_field("owners"),
            platform: o.get_field("platform"),
            platform_details: o.get_field("platformDetails"),
            product_codes: o.get_field("productCodes"),
            public: o.get_field("public"),
            ramdisk_id: o.get_field("ramdiskId"),
            root_device_name: o.get_field("rootDeviceName"),
            root_device_type: o.get_field("rootDeviceType"),
            root_snapshot_id: o.get_field("rootSnapshotId"),
            sriov_net_support: o.get_field("sriovNetSupport"),
            state: o.get_field("state"),
            state_reason: o.get_field("stateReason"),
            tags: o.get_field("tags"),
            tpm_support: o.get_field("tpmSupport"),
            usage_operation: o.get_field("usageOperation"),
            virtualization_type: o.get_field("virtualizationType"),
        }
    }
}
