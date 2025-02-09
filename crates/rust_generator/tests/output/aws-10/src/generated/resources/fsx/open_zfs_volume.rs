/// Manages an Amazon FSx for OpenZFS volume.
/// See the [FSx OpenZFS User Guide](https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/what-is-fsx.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = open_zfs_volume::create(
///         "test",
///         OpenZfsVolumeArgs::builder()
///             .name("testvolume")
///             .parent_volume_id("${testAwsFsxOpenzfsFileSystem.rootVolumeId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx Volumes using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/openZfsVolume:OpenZfsVolume example fsvol-543ab12b1ca672f33
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod open_zfs_volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenZfsVolumeArgs {
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        #[builder(into, default)]
        pub copy_tags_to_snapshots: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Method used to compress the data on the volume. Valid values are `NONE` or `ZSTD`. Child volumes that don't specify compression option will inherit from parent volume. This option on file system applies to the root volume.
        #[builder(into, default)]
        pub data_compression_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to delete all child volumes and snapshots. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`. This configuration must be applied separately before attempting to delete the resource to have the desired behavior..
        #[builder(into, default)]
        pub delete_volume_options: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Volume. You can use a maximum of 203 alphanumeric characters, plus the underscore (_) special character.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// NFS export configuration for the root volume. Exactly 1 item. See `nfs_exports` Block Below for details.
        #[builder(into, default)]
        pub nfs_exports: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::OpenZfsVolumeNfsExports>,
        >,
        /// Specifies the configuration to use when creating the OpenZFS volume. See `origin_snapshot` Block below for details.
        #[builder(into, default)]
        pub origin_snapshot: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fsx::OpenZfsVolumeOriginSnapshot>,
        >,
        /// The volume id of volume that will be the parent volume for the volume being created, this could be the root volume created from the `aws.fsx.OpenZfsFileSystem` resource with the `root_volume_id` or the `id` property of another `aws.fsx.OpenZfsVolume`.
        #[builder(into)]
        pub parent_volume_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// specifies whether the volume is read-only. Default is false.
        #[builder(into, default)]
        pub read_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The record size of an OpenZFS volume, in kibibytes (KiB). Valid values are `4`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, or `1024` KiB. The default is `128` KiB.
        #[builder(into, default)]
        pub record_size_kib: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum amount of storage in gibibytes (GiB) that the volume can use from its parent.
        #[builder(into, default)]
        pub storage_capacity_quota_gib: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The amount of storage in gibibytes (GiB) to reserve from the parent volume.
        #[builder(into, default)]
        pub storage_capacity_reservation_gib: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specify how much storage users or groups can use on the volume. Maximum of 100 items. See `user_and_group_quotas` Block Below.
        #[builder(into, default)]
        pub user_and_group_quotas: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::fsx::OpenZfsVolumeUserAndGroupQuota>>,
        >,
        #[builder(into, default)]
        pub volume_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OpenZfsVolumeResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        pub copy_tags_to_snapshots: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Method used to compress the data on the volume. Valid values are `NONE` or `ZSTD`. Child volumes that don't specify compression option will inherit from parent volume. This option on file system applies to the root volume.
        pub data_compression_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to delete all child volumes and snapshots. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`. This configuration must be applied separately before attempting to delete the resource to have the desired behavior..
        pub delete_volume_options: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Volume. You can use a maximum of 203 alphanumeric characters, plus the underscore (_) special character.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// NFS export configuration for the root volume. Exactly 1 item. See `nfs_exports` Block Below for details.
        pub nfs_exports: pulumi_gestalt_rust::Output<
            Option<super::super::types::fsx::OpenZfsVolumeNfsExports>,
        >,
        /// Specifies the configuration to use when creating the OpenZFS volume. See `origin_snapshot` Block below for details.
        pub origin_snapshot: pulumi_gestalt_rust::Output<
            Option<super::super::types::fsx::OpenZfsVolumeOriginSnapshot>,
        >,
        /// The volume id of volume that will be the parent volume for the volume being created, this could be the root volume created from the `aws.fsx.OpenZfsFileSystem` resource with the `root_volume_id` or the `id` property of another `aws.fsx.OpenZfsVolume`.
        pub parent_volume_id: pulumi_gestalt_rust::Output<String>,
        /// specifies whether the volume is read-only. Default is false.
        pub read_only: pulumi_gestalt_rust::Output<bool>,
        /// The record size of an OpenZFS volume, in kibibytes (KiB). Valid values are `4`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, or `1024` KiB. The default is `128` KiB.
        pub record_size_kib: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum amount of storage in gibibytes (GiB) that the volume can use from its parent.
        pub storage_capacity_quota_gib: pulumi_gestalt_rust::Output<i32>,
        /// The amount of storage in gibibytes (GiB) to reserve from the parent volume.
        pub storage_capacity_reservation_gib: pulumi_gestalt_rust::Output<i32>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specify how much storage users or groups can use on the volume. Maximum of 100 items. See `user_and_group_quotas` Block Below.
        pub user_and_group_quotas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::fsx::OpenZfsVolumeUserAndGroupQuota>,
        >,
        pub volume_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OpenZfsVolumeArgs,
    ) -> OpenZfsVolumeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let copy_tags_to_snapshots_binding = args
            .copy_tags_to_snapshots
            .get_output(context);
        let data_compression_type_binding = args
            .data_compression_type
            .get_output(context);
        let delete_volume_options_binding = args
            .delete_volume_options
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let nfs_exports_binding = args.nfs_exports.get_output(context);
        let origin_snapshot_binding = args.origin_snapshot.get_output(context);
        let parent_volume_id_binding = args.parent_volume_id.get_output(context);
        let read_only_binding = args.read_only.get_output(context);
        let record_size_kib_binding = args.record_size_kib.get_output(context);
        let storage_capacity_quota_gib_binding = args
            .storage_capacity_quota_gib
            .get_output(context);
        let storage_capacity_reservation_gib_binding = args
            .storage_capacity_reservation_gib
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_and_group_quotas_binding = args
            .user_and_group_quotas
            .get_output(context);
        let volume_type_binding = args.volume_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:fsx/openZfsVolume:OpenZfsVolume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyTagsToSnapshots".into(),
                    value: copy_tags_to_snapshots_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataCompressionType".into(),
                    value: data_compression_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteVolumeOptions".into(),
                    value: delete_volume_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nfsExports".into(),
                    value: nfs_exports_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originSnapshot".into(),
                    value: origin_snapshot_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentVolumeId".into(),
                    value: parent_volume_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readOnly".into(),
                    value: read_only_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recordSizeKib".into(),
                    value: record_size_kib_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageCapacityQuotaGib".into(),
                    value: storage_capacity_quota_gib_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageCapacityReservationGib".into(),
                    value: storage_capacity_reservation_gib_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userAndGroupQuotas".into(),
                    value: user_and_group_quotas_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeType".into(),
                    value: volume_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OpenZfsVolumeResult {
            arn: o.get_field("arn"),
            copy_tags_to_snapshots: o.get_field("copyTagsToSnapshots"),
            data_compression_type: o.get_field("dataCompressionType"),
            delete_volume_options: o.get_field("deleteVolumeOptions"),
            name: o.get_field("name"),
            nfs_exports: o.get_field("nfsExports"),
            origin_snapshot: o.get_field("originSnapshot"),
            parent_volume_id: o.get_field("parentVolumeId"),
            read_only: o.get_field("readOnly"),
            record_size_kib: o.get_field("recordSizeKib"),
            storage_capacity_quota_gib: o.get_field("storageCapacityQuotaGib"),
            storage_capacity_reservation_gib: o
                .get_field("storageCapacityReservationGib"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_and_group_quotas: o.get_field("userAndGroupQuotas"),
            volume_type: o.get_field("volumeType"),
        }
    }
}
