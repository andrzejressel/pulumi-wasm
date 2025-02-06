/// Provides a FSx Backup resource.
///
/// ## Lustre Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = backup::create(
///         "example",
///         BackupArgs::builder()
///             .file_system_id("${exampleLustreFileSystem.id}")
///             .build_struct(),
///     );
///     let exampleLustreFileSystem = lustre_file_system::create(
///         "exampleLustreFileSystem",
///         LustreFileSystemArgs::builder()
///             .deployment_type("PERSISTENT_1")
///             .per_unit_storage_throughput(50)
///             .storage_capacity(1200)
///             .subnet_ids("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Windows Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = backup::create(
///         "example",
///         BackupArgs::builder()
///             .file_system_id("${exampleWindowsFileSystem.id}")
///             .build_struct(),
///     );
///     let exampleWindowsFileSystem = windows_file_system::create(
///         "exampleWindowsFileSystem",
///         WindowsFileSystemArgs::builder()
///             .active_directory_id("${eample.id}")
///             .skip_final_backup(true)
///             .storage_capacity(32)
///             .subnet_ids(vec!["${example1.id}",])
///             .throughput_capacity(8)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## ONTAP Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = backup::create(
///         "example",
///         BackupArgs::builder().volume_id("${exampleOntapVolume.id}").build_struct(),
///     );
///     let exampleOntapVolume = ontap_volume::create(
///         "exampleOntapVolume",
///         OntapVolumeArgs::builder()
///             .junction_path("/example")
///             .name("example")
///             .size_in_megabytes(1024)
///             .storage_efficiency_enabled(true)
///             .storage_virtual_machine_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## OpenZFS Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = backup::create(
///         "example",
///         BackupArgs::builder()
///             .file_system_id("${exampleOpenZfsFileSystem.id}")
///             .build_struct(),
///     );
///     let exampleOpenZfsFileSystem = open_zfs_file_system::create(
///         "exampleOpenZfsFileSystem",
///         OpenZfsFileSystemArgs::builder()
///             .deployment_type("SINGLE_AZ_1")
///             .storage_capacity(64)
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .throughput_capacity(64)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx Backups using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/backup:Backup example fs-543ab12b1ca672f33
/// ```
pub mod backup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupArgs {
        /// The ID of the file system to back up. Required if backing up Lustre or Windows file systems.
        #[builder(into, default)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the volume to back up. Required if backing up a ONTAP Volume.
        #[builder(into, default)]
        pub volume_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackupResult {
        /// Amazon Resource Name of the backup.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the file system to back up. Required if backing up Lustre or Windows file systems.
        pub file_system_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the AWS Key Management Service (AWS KMS) key used to encrypt the backup of the Amazon FSx file system's data at rest.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the file system backup.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The ID of the volume to back up. Required if backing up a ONTAP Volume.
        pub volume_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackupArgs,
    ) -> BackupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let file_system_id_binding = args.file_system_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let volume_id_binding = args.volume_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/backup:Backup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "volumeId".into(),
                    value: &volume_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            file_system_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileSystemId"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            volume_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeId"),
            ),
        }
    }
}
