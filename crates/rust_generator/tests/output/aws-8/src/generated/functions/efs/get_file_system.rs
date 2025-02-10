#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_file_system {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFileSystemArgs {
        /// Restricts the list to the file system with this creation token.
        #[builder(into, default)]
        pub creation_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID that identifies the file system (e.g., fs-ccfc0d65).
        #[builder(into, default)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Restricts the list to the file system with these tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFileSystemResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the Availability Zone in which the file system's One Zone storage classes exist.
        pub availability_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The Availability Zone name in which the file system's One Zone storage classes exist.
        pub availability_zone_name: pulumi_gestalt_rust::Output<String>,
        pub creation_token: pulumi_gestalt_rust::Output<String>,
        /// DNS name for the filesystem per [documented convention](http://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-cmd-dns-name.html).
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Whether EFS is encrypted.
        pub encrypted: pulumi_gestalt_rust::Output<bool>,
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the KMS encryption key.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        pub lifecycle_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::efs::GetFileSystemLifecyclePolicy>,
        >,
        /// File system [lifecycle policy](https://docs.aws.amazon.com/efs/latest/ug/API_LifecyclePolicy.html) object.
        pub lifecycle_policy: pulumi_gestalt_rust::Output<
            super::super::super::types::efs::GetFileSystemLifecyclePolicy,
        >,
        /// The value of the file system's `Name` tag.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// File system performance mode.
        pub performance_mode: pulumi_gestalt_rust::Output<String>,
        pub protections: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::efs::GetFileSystemProtection>,
        >,
        /// The throughput, measured in MiB/s, that you want to provision for the file system.
        pub provisioned_throughput_in_mibps: pulumi_gestalt_rust::Output<f64>,
        /// Current byte count used by the file system.
        pub size_in_bytes: pulumi_gestalt_rust::Output<i32>,
        /// A map of tags to assign to the file system.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Throughput mode for the file system.
        pub throughput_mode: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFileSystemArgs,
    ) -> GetFileSystemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let creation_token_binding = args.creation_token.get_output(context);
        let file_system_id_binding = args.file_system_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:efs/getFileSystem:getFileSystem".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "creationToken".into(),
                    value: creation_token_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemId".into(),
                    value: file_system_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFileSystemResult {
            arn: o.get_field("arn"),
            availability_zone_id: o.get_field("availabilityZoneId"),
            availability_zone_name: o.get_field("availabilityZoneName"),
            creation_token: o.get_field("creationToken"),
            dns_name: o.get_field("dnsName"),
            encrypted: o.get_field("encrypted"),
            file_system_id: o.get_field("fileSystemId"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            lifecycle_policies: o.get_field("lifecyclePolicies"),
            lifecycle_policy: o.get_field("lifecyclePolicy"),
            name: o.get_field("name"),
            performance_mode: o.get_field("performanceMode"),
            protections: o.get_field("protections"),
            provisioned_throughput_in_mibps: o.get_field("provisionedThroughputInMibps"),
            size_in_bytes: o.get_field("sizeInBytes"),
            tags: o.get_field("tags"),
            throughput_mode: o.get_field("throughputMode"),
        }
    }
}
