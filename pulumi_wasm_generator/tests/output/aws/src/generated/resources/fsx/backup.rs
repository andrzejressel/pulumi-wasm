/// Provides a FSx Backup resource.
///
/// ## Lustre Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupArgs {
        /// The ID of the file system to back up. Required if backing up Lustre or Windows file systems.
        #[builder(into, default)]
        pub file_system_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the volume to back up. Required if backing up a ONTAP Volume.
        #[builder(into, default)]
        pub volume_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackupResult {
        /// Amazon Resource Name of the backup.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the file system to back up. Required if backing up Lustre or Windows file systems.
        pub file_system_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AWS Key Management Service (AWS KMS) key used to encrypt the backup of the Amazon FSx file system's data at rest.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// AWS account identifier that created the file system.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the file system backup.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The ID of the volume to back up. Required if backing up a ONTAP Volume.
        pub volume_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupArgs) -> BackupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let file_system_id_binding = args.file_system_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let volume_id_binding = args.volume_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/backup:Backup".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "volumeId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeId").unwrap(),
            ),
        }
    }
}
