pub mod get_ami {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAmiArgs {
        /// Limit search to users with *explicit* launch permission on
        /// the image. Valid items are the numeric account ID or `self`.
        #[builder(into, default)]
        pub executable_users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more name/value pairs to filter off of. There are
        /// several valid keys, for a full reference, check out
        /// [describe-images in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetAmiFilter>>,
        >,
        /// If true, all deprecated AMIs are included in the response. If false, no deprecated AMIs are included in the response. If no value is specified, the default value is false.
        #[builder(into, default)]
        pub include_deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        /// If more than one result is returned, use the most
        /// recent AMI.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
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
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AMI owners to limit search. Valid values: an AWS account ID, `self` (the current account), or an AWS owner alias (e.g., `amazon`, `aws-marketplace`, `microsoft`).
        #[builder(into, default)]
        pub owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Any tags assigned to the image.
        /// * `tags.#.key` - Key name of the tag.
        /// * `tags.#.value` - Value of the tag.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAmiResult {
        /// OS architecture of the AMI (ie: `i386` or `x86_64`).
        pub architecture: pulumi_wasm_rust::Output<String>,
        /// ARN of the AMI.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Set of objects with block device mappings of the AMI.
        pub block_device_mappings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetAmiBlockDeviceMapping>,
        >,
        /// Boot mode of the image.
        pub boot_mode: pulumi_wasm_rust::Output<String>,
        /// Date and time the image was created.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// Date and time when the image will be deprecated.
        pub deprecation_time: pulumi_wasm_rust::Output<String>,
        /// Description of the AMI that was provided during image
        /// creation.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether enhanced networking with ENA is enabled.
        pub ena_support: pulumi_wasm_rust::Output<bool>,
        pub executable_users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetAmiFilter>>,
        >,
        /// Hypervisor type of the image.
        pub hypervisor: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ID of the AMI. Should be the same as the resource `id`.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// Location of the AMI.
        pub image_location: pulumi_wasm_rust::Output<String>,
        /// AWS account alias (for example, `amazon`, `self`) or
        /// the AWS account ID of the AMI owner.
        pub image_owner_alias: pulumi_wasm_rust::Output<String>,
        /// Type of image.
        pub image_type: pulumi_wasm_rust::Output<String>,
        /// Instance Metadata Service (IMDS) support mode for the image. Set to `v2.0` if instances ran from this image enforce IMDSv2.
        pub imds_support: pulumi_wasm_rust::Output<String>,
        pub include_deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        /// Kernel associated with the image, if any. Only applicable
        /// for machine images.
        pub kernel_id: pulumi_wasm_rust::Output<String>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the AMI that was provided during image creation.
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS account ID of the image owner.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Value is Windows for `Windows` AMIs; otherwise blank.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Platform details associated with the billing code of the AMI.
        pub platform_details: pulumi_wasm_rust::Output<String>,
        /// Any product codes associated with the AMI.
        /// * `product_codes.#.product_code_id` - The product code.
        /// * `product_codes.#.product_code_type` - The type of product code.
        pub product_codes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetAmiProductCode>,
        >,
        /// `true` if the image has public launch permissions.
        pub public: pulumi_wasm_rust::Output<bool>,
        /// RAM disk associated with the image, if any. Only applicable
        /// for machine images.
        pub ramdisk_id: pulumi_wasm_rust::Output<String>,
        /// Device name of the root device.
        pub root_device_name: pulumi_wasm_rust::Output<String>,
        /// Type of root device (ie: `ebs` or `instance-store`).
        pub root_device_type: pulumi_wasm_rust::Output<String>,
        /// Snapshot id associated with the root device, if any
        /// (only applies to `ebs` root devices).
        pub root_snapshot_id: pulumi_wasm_rust::Output<String>,
        /// Whether enhanced networking is enabled.
        pub sriov_net_support: pulumi_wasm_rust::Output<String>,
        /// Current state of the AMI. If the state is `available`, the image
        /// is successfully registered and can be used to launch an instance.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Describes a state change. Fields are `UNSET` if not available.
        pub state_reason: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Any tags assigned to the image.
        /// * `tags.#.key` - Key name of the tag.
        /// * `tags.#.value` - Value of the tag.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// If the image is configured for NitroTPM support, the value is `v2.0`.
        pub tpm_support: pulumi_wasm_rust::Output<String>,
        /// Operation of the Amazon EC2 instance and the billing code that is associated with the AMI.
        pub usage_operation: pulumi_wasm_rust::Output<String>,
        /// Type of virtualization of the AMI (ie: `hvm` or
        /// `paravirtual`).
        pub virtualization_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAmiArgs) -> GetAmiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let executable_users_binding = args.executable_users.get_inner();
        let filters_binding = args.filters.get_inner();
        let include_deprecated_binding = args.include_deprecated.get_inner();
        let most_recent_binding = args.most_recent.get_inner();
        let name_regex_binding = args.name_regex.get_inner();
        let owners_binding = args.owners.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getAmi:getAmi".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "executableUsers".into(),
                    value: &executable_users_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "includeDeprecated".into(),
                    value: &include_deprecated_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
                register_interface::ObjectField {
                    name: "owners".into(),
                    value: &owners_binding,
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
                    name: "blockDeviceMappings".into(),
                },
                register_interface::ResultField {
                    name: "bootMode".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "deprecationTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enaSupport".into(),
                },
                register_interface::ResultField {
                    name: "executableUsers".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "hypervisor".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
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
                    name: "includeDeprecated".into(),
                },
                register_interface::ResultField {
                    name: "kernelId".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nameRegex".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "owners".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "platformDetails".into(),
                },
                register_interface::ResultField {
                    name: "productCodes".into(),
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
                    name: "rootDeviceType".into(),
                },
                register_interface::ResultField {
                    name: "rootSnapshotId".into(),
                },
                register_interface::ResultField {
                    name: "sriovNetSupport".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateReason".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAmiResult {
            architecture: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("architecture").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            block_device_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockDeviceMappings").unwrap(),
            ),
            boot_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootMode").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            deprecation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deprecationTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            ena_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enaSupport").unwrap(),
            ),
            executable_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executableUsers").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            hypervisor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hypervisor").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
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
            include_deprecated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeDeprecated").unwrap(),
            ),
            kernel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kernelId").unwrap(),
            ),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_regex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameRegex").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owners").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            platform_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformDetails").unwrap(),
            ),
            product_codes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productCodes").unwrap(),
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
            root_device_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootDeviceType").unwrap(),
            ),
            root_snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootSnapshotId").unwrap(),
            ),
            sriov_net_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sriovNetSupport").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateReason").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
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
