/// Resource for managing an Amazon File Cache cache.
/// See the [Create File Cache](https://docs.aws.amazon.com/fsx/latest/APIReference/API_CreateFileCache.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = file_cache::create(
///         "example",
///         FileCacheArgs::builder()
///             .data_repository_associations(
///                 vec![
///                     FileCacheDataRepositoryAssociation::builder()
///                     .dataRepositoryPath("nfs://filer.domain.com")
///                     .dataRepositorySubdirectories(vec!["test", "test2",])
///                     .fileCachePath("/ns1")
///                     .nfs(vec![FileCacheDataRepositoryAssociationNf::builder()
///                     .dnsIps(vec!["192.168.0.1", "192.168.0.2",]).version("NFS3")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .file_cache_type("LUSTRE")
///             .file_cache_type_version("2.12")
///             .lustre_configurations(
///                 vec![
///                     FileCacheLustreConfiguration::builder().deploymentType("CACHE_1")
///                     .metadataConfigurations(vec![FileCacheLustreConfigurationMetadataConfiguration::builder()
///                     .storageCapacity(2400).build_struct(),])
///                     .perUnitStorageThroughput(1000).weeklyMaintenanceStartTime("2:05:00")
///                     .build_struct(),
///                 ],
///             )
///             .storage_capacity(1200)
///             .subnet_ids(vec!["${test1.id}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon File Cache cache using the resource `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/fileCache:FileCache example fc-8012925589
/// ```
pub mod file_cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FileCacheArgs {
        /// A boolean flag indicating whether tags for the cache should be copied to data repository associations. This value defaults to false.
        #[builder(into, default)]
        pub copy_tags_to_data_repository_associations: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// See the `data_repository_association` configuration block. Max of 8.
        /// A list of up to 8 configurations for data repository associations (DRAs) to be created during the cache creation. The DRAs link the cache to either an Amazon S3 data repository or a Network File System (NFS) data repository that supports the NFSv3 protocol. The DRA configurations must meet the following requirements: 1) All configurations on the list must be of the same data repository type, either all S3 or all NFS. A cache can't link to different data repository types at the same time. 2) An NFS DRA must link to an NFS file system that supports the NFSv3 protocol. DRA automatic import and automatic export is not supported.
        #[builder(into, default)]
        pub data_repository_associations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::fsx::FileCacheDataRepositoryAssociation>>,
        >,
        /// The type of cache that you're creating. The only supported value is `LUSTRE`.
        #[builder(into)]
        pub file_cache_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version for the type of cache that you're creating. The only supported value is `2.12`.
        #[builder(into)]
        pub file_cache_type_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the AWS Key Management Service (AWS KMS) key to use for encrypting data on an Amazon File Cache. If a KmsKeyId isn't specified, the Amazon FSx-managed AWS KMS key for your account is used.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// See the `lustre_configuration` block. Required when `file_cache_type` is `LUSTRE`.
        #[builder(into, default)]
        pub lustre_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::fsx::FileCacheLustreConfiguration>>,
        >,
        /// A list of IDs specifying the security groups to apply to all network interfaces created for Amazon File Cache access.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The storage capacity of the cache in gibibytes (GiB). Valid values are `1200` GiB, `2400` GiB, and increments of `2400` GiB.
        #[builder(into)]
        pub storage_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A list of subnet IDs that the cache will be accessible from. You can specify only one subnet ID.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A map of tags to assign to the file cache. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FileCacheResult {
        /// The Amazon Resource Name (ARN) for the resource.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A boolean flag indicating whether tags for the cache should be copied to data repository associations. This value defaults to false.
        pub copy_tags_to_data_repository_associations: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A list of IDs of data repository associations that are associated with this cache.
        pub data_repository_association_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// See the `data_repository_association` configuration block. Max of 8.
        /// A list of up to 8 configurations for data repository associations (DRAs) to be created during the cache creation. The DRAs link the cache to either an Amazon S3 data repository or a Network File System (NFS) data repository that supports the NFSv3 protocol. The DRA configurations must meet the following requirements: 1) All configurations on the list must be of the same data repository type, either all S3 or all NFS. A cache can't link to different data repository types at the same time. 2) An NFS DRA must link to an NFS file system that supports the NFSv3 protocol. DRA automatic import and automatic export is not supported.
        pub data_repository_associations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::fsx::FileCacheDataRepositoryAssociation>>,
        >,
        /// The Domain Name System (DNS) name for the cache.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// The system-generated, unique ID of the cache.
        pub file_cache_id: pulumi_gestalt_rust::Output<String>,
        /// The type of cache that you're creating. The only supported value is `LUSTRE`.
        pub file_cache_type: pulumi_gestalt_rust::Output<String>,
        /// The version for the type of cache that you're creating. The only supported value is `2.12`.
        pub file_cache_type_version: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the AWS Key Management Service (AWS KMS) key to use for encrypting data on an Amazon File Cache. If a KmsKeyId isn't specified, the Amazon FSx-managed AWS KMS key for your account is used.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// See the `lustre_configuration` block. Required when `file_cache_type` is `LUSTRE`.
        pub lustre_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::fsx::FileCacheLustreConfiguration>>,
        >,
        /// A list of network interface IDs.
        pub network_interface_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A list of IDs specifying the security groups to apply to all network interfaces created for Amazon File Cache access.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The storage capacity of the cache in gibibytes (GiB). Valid values are `1200` GiB, `2400` GiB, and increments of `2400` GiB.
        pub storage_capacity: pulumi_gestalt_rust::Output<i32>,
        /// A list of subnet IDs that the cache will be accessible from. You can specify only one subnet ID.
        ///
        /// The following arguments are optional:
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the file cache. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of your virtual private cloud (VPC).
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FileCacheArgs,
    ) -> FileCacheResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let copy_tags_to_data_repository_associations_binding = args
            .copy_tags_to_data_repository_associations
            .get_output(context)
            .get_inner();
        let data_repository_associations_binding = args
            .data_repository_associations
            .get_output(context)
            .get_inner();
        let file_cache_type_binding = args
            .file_cache_type
            .get_output(context)
            .get_inner();
        let file_cache_type_version_binding = args
            .file_cache_type_version
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let lustre_configurations_binding = args
            .lustre_configurations
            .get_output(context)
            .get_inner();
        let security_group_ids_binding = args
            .security_group_ids
            .get_output(context)
            .get_inner();
        let storage_capacity_binding = args
            .storage_capacity
            .get_output(context)
            .get_inner();
        let subnet_ids_binding = args.subnet_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/fileCache:FileCache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "copyTagsToDataRepositoryAssociations".into(),
                    value: &copy_tags_to_data_repository_associations_binding,
                },
                register_interface::ObjectField {
                    name: "dataRepositoryAssociations".into(),
                    value: &data_repository_associations_binding,
                },
                register_interface::ObjectField {
                    name: "fileCacheType".into(),
                    value: &file_cache_type_binding,
                },
                register_interface::ObjectField {
                    name: "fileCacheTypeVersion".into(),
                    value: &file_cache_type_version_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "lustreConfigurations".into(),
                    value: &lustre_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "storageCapacity".into(),
                    value: &storage_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FileCacheResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            copy_tags_to_data_repository_associations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("copyTagsToDataRepositoryAssociations"),
            ),
            data_repository_association_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataRepositoryAssociationIds"),
            ),
            data_repository_associations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataRepositoryAssociations"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            file_cache_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileCacheId"),
            ),
            file_cache_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileCacheType"),
            ),
            file_cache_type_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileCacheTypeVersion"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            lustre_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lustreConfigurations"),
            ),
            network_interface_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceIds"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupIds"),
            ),
            storage_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageCapacity"),
            ),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
