/// Manages a FSx for Lustre Data Repository Association. See [Linking your file system to an S3 bucket](https://docs.aws.amazon.com/fsx/latest/LustreGuide/create-dra-linked-data-repo.html) for more information.
///
/// > **NOTE:** Data Repository Associations are only compatible with AWS FSx for Lustre File Systems and `PERSISTENT_2` deployment type.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("my-bucket").build_struct(),
///     );
///     let exampleBucketAclV2 = bucket_acl_v_2::create(
///         "exampleBucketAclV2",
///         BucketAclV2Args::builder().acl("private").bucket("${example.id}").build_struct(),
///     );
///     let exampleDataRepositoryAssociation = data_repository_association::create(
///         "exampleDataRepositoryAssociation",
///         DataRepositoryAssociationArgs::builder()
///             .data_repository_path("s3://${example.id}")
///             .file_system_id("${exampleLustreFileSystem.id}")
///             .file_system_path("/my-bucket")
///             .s_3(
///                 DataRepositoryAssociationS3::builder()
///                     .autoExportPolicy(
///                         DataRepositoryAssociationS3AutoExportPolicy::builder()
///                             .events(vec!["NEW", "CHANGED", "DELETED",])
///                             .build_struct(),
///                     )
///                     .autoImportPolicy(
///                         DataRepositoryAssociationS3AutoImportPolicy::builder()
///                             .events(vec!["NEW", "CHANGED", "DELETED",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleLustreFileSystem = lustre_file_system::create(
///         "exampleLustreFileSystem",
///         LustreFileSystemArgs::builder()
///             .deployment_type("PERSISTENT_2")
///             .per_unit_storage_throughput(125)
///             .storage_capacity(1200)
///             .subnet_ids("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx Data Repository Associations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/dataRepositoryAssociation:DataRepositoryAssociation example dra-0b1cfaeca11088b10
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_repository_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataRepositoryAssociationArgs {
        /// Set to true to run an import data repository task to import metadata from the data repository to the file system after the data repository association is created. Defaults to `false`.
        #[builder(into, default)]
        pub batch_import_meta_data_on_create: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The path to the Amazon S3 data repository that will be linked to the file system. The path must be an S3 bucket s3://myBucket/myPrefix/. This path specifies where in the S3 data repository files will be imported from or exported to. The same S3 bucket cannot be linked more than once to the same file system.
        #[builder(into)]
        pub data_repository_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set to true to delete files from the file system upon deleting this data repository association. Defaults to `false`.
        #[builder(into, default)]
        pub delete_data_in_filesystem: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Amazon FSx file system to on which to create a data repository association.
        #[builder(into)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A path on the file system that points to a high-level directory (such as `/ns1/`) or subdirectory (such as `/ns1/subdir/`) that will be mapped 1-1 with `data_repository_path`. The leading forward slash in the name is required. Two data repository associations cannot have overlapping file system paths. For example, if a data repository is associated with file system path `/ns1/`, then you cannot link another data repository with file system path `/ns1/ns2`. This path specifies where in your file system files will be exported from or imported to. This file system directory can be linked to only one Amazon S3 bucket, and no other S3 bucket can be linked to the directory.
        #[builder(into)]
        pub file_system_path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. The maximum number of disks that a single file can be striped across is limited by the total number of disks that make up the file system.
        #[builder(into, default)]
        pub imported_file_chunk_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// See the `s3` configuration block. Max of 1.
        /// The configuration for an Amazon S3 data repository linked to an Amazon FSx Lustre file system with a data repository association. The configuration defines which file events (new, changed, or deleted files or directories) are automatically imported from the linked data repository to the file system or automatically exported from the file system to the data repository.
        #[builder(into, default)]
        pub s3: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::DataRepositoryAssociationS3>,
        >,
        /// A map of tags to assign to the data repository association. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataRepositoryAssociationResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub association_id: pulumi_gestalt_rust::Output<String>,
        /// Set to true to run an import data repository task to import metadata from the data repository to the file system after the data repository association is created. Defaults to `false`.
        pub batch_import_meta_data_on_create: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The path to the Amazon S3 data repository that will be linked to the file system. The path must be an S3 bucket s3://myBucket/myPrefix/. This path specifies where in the S3 data repository files will be imported from or exported to. The same S3 bucket cannot be linked more than once to the same file system.
        pub data_repository_path: pulumi_gestalt_rust::Output<String>,
        /// Set to true to delete files from the file system upon deleting this data repository association. Defaults to `false`.
        pub delete_data_in_filesystem: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Amazon FSx file system to on which to create a data repository association.
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
        /// A path on the file system that points to a high-level directory (such as `/ns1/`) or subdirectory (such as `/ns1/subdir/`) that will be mapped 1-1 with `data_repository_path`. The leading forward slash in the name is required. Two data repository associations cannot have overlapping file system paths. For example, if a data repository is associated with file system path `/ns1/`, then you cannot link another data repository with file system path `/ns1/ns2`. This path specifies where in your file system files will be exported from or imported to. This file system directory can be linked to only one Amazon S3 bucket, and no other S3 bucket can be linked to the directory.
        pub file_system_path: pulumi_gestalt_rust::Output<String>,
        /// For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. The maximum number of disks that a single file can be striped across is limited by the total number of disks that make up the file system.
        pub imported_file_chunk_size: pulumi_gestalt_rust::Output<i32>,
        /// See the `s3` configuration block. Max of 1.
        /// The configuration for an Amazon S3 data repository linked to an Amazon FSx Lustre file system with a data repository association. The configuration defines which file events (new, changed, or deleted files or directories) are automatically imported from the linked data repository to the file system or automatically exported from the file system to the data repository.
        pub s3: pulumi_gestalt_rust::Output<
            super::super::types::fsx::DataRepositoryAssociationS3,
        >,
        /// A map of tags to assign to the data repository association. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DataRepositoryAssociationArgs,
    ) -> DataRepositoryAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let batch_import_meta_data_on_create_binding = args
            .batch_import_meta_data_on_create
            .get_output(context)
            .get_inner();
        let data_repository_path_binding = args
            .data_repository_path
            .get_output(context)
            .get_inner();
        let delete_data_in_filesystem_binding = args
            .delete_data_in_filesystem
            .get_output(context)
            .get_inner();
        let file_system_id_binding = args.file_system_id.get_output(context).get_inner();
        let file_system_path_binding = args
            .file_system_path
            .get_output(context)
            .get_inner();
        let imported_file_chunk_size_binding = args
            .imported_file_chunk_size
            .get_output(context)
            .get_inner();
        let s3_binding = args.s3.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/dataRepositoryAssociation:DataRepositoryAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "batchImportMetaDataOnCreate".into(),
                    value: &batch_import_meta_data_on_create_binding,
                },
                register_interface::ObjectField {
                    name: "dataRepositoryPath".into(),
                    value: &data_repository_path_binding,
                },
                register_interface::ObjectField {
                    name: "deleteDataInFilesystem".into(),
                    value: &delete_data_in_filesystem_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemPath".into(),
                    value: &file_system_path_binding,
                },
                register_interface::ObjectField {
                    name: "importedFileChunkSize".into(),
                    value: &imported_file_chunk_size_binding,
                },
                register_interface::ObjectField {
                    name: "s3".into(),
                    value: &s3_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataRepositoryAssociationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            association_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            batch_import_meta_data_on_create: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("batchImportMetaDataOnCreate"),
            ),
            data_repository_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataRepositoryPath"),
            ),
            delete_data_in_filesystem: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteDataInFilesystem"),
            ),
            file_system_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileSystemId"),
            ),
            file_system_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileSystemPath"),
            ),
            imported_file_chunk_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("importedFileChunkSize"),
            ),
            s3: pulumi_gestalt_rust::__private::into_domain(o.extract_field("s3")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
