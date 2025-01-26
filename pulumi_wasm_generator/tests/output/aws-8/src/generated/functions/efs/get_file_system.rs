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
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneId".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneName".into(),
                },
                register_interface::ResultField {
                    name: "creationToken".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "lifecyclePolicies".into(),
                },
                register_interface::ResultField {
                    name: "lifecyclePolicy".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "performanceMode".into(),
                },
                register_interface::ResultField {
                    name: "protections".into(),
                },
                register_interface::ResultField {
                    name: "provisionedThroughputInMibps".into(),
                },
                register_interface::ResultField {
                    name: "sizeInBytes".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "throughputMode".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFileSystemResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneId").unwrap(),
            ),
            availability_zone_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneName").unwrap(),
            ),
            creation_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationToken").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            lifecycle_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecyclePolicies").unwrap(),
            ),
            lifecycle_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecyclePolicy").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            performance_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("performanceMode").unwrap(),
            ),
            protections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protections").unwrap(),
            ),
            provisioned_throughput_in_mibps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisionedThroughputInMibps").unwrap(),
            ),
            size_in_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sizeInBytes").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            throughput_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughputMode").unwrap(),
            ),
        }
    }
}
