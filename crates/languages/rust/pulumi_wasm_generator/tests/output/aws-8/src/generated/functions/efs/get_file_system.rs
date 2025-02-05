pub mod get_file_system {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFileSystemArgs {
        /// Restricts the list to the file system with this creation token.
        #[builder(into, default)]
        pub creation_token: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID that identifies the file system (e.g., fs-ccfc0d65).
        #[builder(into, default)]
        pub file_system_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Restricts the list to the file system with these tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFileSystemResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The identifier of the Availability Zone in which the file system's One Zone storage classes exist.
        pub availability_zone_id: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone name in which the file system's One Zone storage classes exist.
        pub availability_zone_name: pulumi_wasm_rust::Output<String>,
        pub creation_token: pulumi_wasm_rust::Output<String>,
        /// DNS name for the filesystem per [documented convention](http://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-cmd-dns-name.html).
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Whether EFS is encrypted.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN for the KMS encryption key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        pub lifecycle_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::efs::GetFileSystemLifecyclePolicy>,
        >,
        /// File system [lifecycle policy](https://docs.aws.amazon.com/efs/latest/ug/API_LifecyclePolicy.html) object.
        pub lifecycle_policy: pulumi_wasm_rust::Output<
            super::super::super::types::efs::GetFileSystemLifecyclePolicy,
        >,
        /// The value of the file system's `Name` tag.
        pub name: pulumi_wasm_rust::Output<String>,
        /// File system performance mode.
        pub performance_mode: pulumi_wasm_rust::Output<String>,
        pub protections: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::efs::GetFileSystemProtection>,
        >,
        /// The throughput, measured in MiB/s, that you want to provision for the file system.
        pub provisioned_throughput_in_mibps: pulumi_wasm_rust::Output<f64>,
        /// Current byte count used by the file system.
        pub size_in_bytes: pulumi_wasm_rust::Output<i32>,
        /// A map of tags to assign to the file system.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Throughput mode for the file system.
        pub throughput_mode: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFileSystemArgs,
    ) -> GetFileSystemResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let creation_token_binding = args.creation_token.get_output(context).get_inner();
        let file_system_id_binding = args.file_system_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:efs/getFileSystem:getFileSystem".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "creationToken".into(),
                    value: &creation_token_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFileSystemResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZoneId"),
            ),
            availability_zone_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZoneName"),
            ),
            creation_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationToken"),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encrypted"),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileSystemId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            lifecycle_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecyclePolicies"),
            ),
            lifecycle_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecyclePolicy"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            performance_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("performanceMode"),
            ),
            protections: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protections"),
            ),
            provisioned_throughput_in_mibps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("provisionedThroughputInMibps"),
            ),
            size_in_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sizeInBytes"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            throughput_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("throughputMode"),
            ),
        }
    }
}
