/// Manages an Amazon FSx for OpenZFS volume.
/// See the [FSx OpenZFS User Guide](https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/what-is-fsx.html) for more information.
///
/// ## Example Usage
///
/// ### Root volume Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod open_zfs_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenZfsSnapshotArgs {
        /// The name of the Snapshot. You can use a maximum of 203 alphanumeric characters plus either _ or -  or : or . for the name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the volume to snapshot. This can be the root volume or a child volume.
        #[builder(into)]
        pub volume_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OpenZfsSnapshotResult {
        /// Amazon Resource Name of the snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Snapshot. You can use a maximum of 203 alphanumeric characters plus either _ or -  or : or . for the name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the file system. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level. If you have set `copy_tags_to_backups` to true, and you specify one or more tags, no existing file system tags are copied from the file system to the backup.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the volume to snapshot. This can be the root volume or a child volume.
        pub volume_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OpenZfsSnapshotArgs,
    ) -> OpenZfsSnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let volume_id_binding = args.volume_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:fsx/openZfsSnapshot:OpenZfsSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeId".into(),
                    value: volume_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OpenZfsSnapshotResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            volume_id: o.get_field("volumeId"),
        }
    }
}
