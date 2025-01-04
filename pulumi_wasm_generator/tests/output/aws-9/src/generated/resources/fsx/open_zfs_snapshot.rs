/// Manages an Amazon FSx for OpenZFS volume.
/// See the [FSx OpenZFS User Guide](https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/what-is-fsx.html) for more information.
///
/// ## Example Usage
///
/// ### Root volume Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = open_zfs_snapshot::create(
///         "example",
///         OpenZfsSnapshotArgs::builder()
///             .name("example")
///             .volume_id("${exampleOpenZfsFileSystem.rootVolumeId}")
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
/// ### Child volume Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = open_zfs_snapshot::create(
///         "example",
///         OpenZfsSnapshotArgs::builder()
///             .name("example")
///             .volume_id("${exampleOpenZfsVolume.id}")
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
///     let exampleOpenZfsVolume = open_zfs_volume::create(
///         "exampleOpenZfsVolume",
///         OpenZfsVolumeArgs::builder()
///             .name("example")
///             .parent_volume_id("${exampleOpenZfsFileSystem.rootVolumeId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx OpenZFS snapshot using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/openZfsSnapshot:OpenZfsSnapshot example fs-543ab12b1ca672f33
/// ```
pub mod open_zfs_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenZfsSnapshotArgs {
        /// The name of the Snapshot. You can use a maximum of 203 alphanumeric characters plus either _ or -  or : or . for the name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the volume to snapshot. This can be the root volume or a child volume.
        #[builder(into)]
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OpenZfsSnapshotResult {
        /// Amazon Resource Name of the snapshot.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// The name of the Snapshot. You can use a maximum of 203 alphanumeric characters plus either _ or -  or : or . for the name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the volume to snapshot. This can be the root volume or a child volume.
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OpenZfsSnapshotArgs) -> OpenZfsSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let volume_id_binding = args.volume_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/openZfsSnapshot:OpenZfsSnapshot".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
        OpenZfsSnapshotResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeId").unwrap(),
            ),
        }
    }
}
