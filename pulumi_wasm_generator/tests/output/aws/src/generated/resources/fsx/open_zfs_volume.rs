/// Manages an Amazon FSx for OpenZFS volume.
/// See the [FSx OpenZFS User Guide](https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/what-is-fsx.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod open_zfs_volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenZfsVolumeArgs {
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        #[builder(into, default)]
        pub copy_tags_to_snapshots: pulumi_wasm_rust::Output<Option<bool>>,
        /// Method used to compress the data on the volume. Valid values are `NONE` or `ZSTD`. Child volumes that don't specify compression option will inherit from parent volume. This option on file system applies to the root volume.
        #[builder(into, default)]
        pub data_compression_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to delete all child volumes and snapshots. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`. This configuration must be applied separately before attempting to delete the resource to have the desired behavior..
        #[builder(into, default)]
        pub delete_volume_options: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Volume. You can use a maximum of 203 alphanumeric characters, plus the underscore (_) special character.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// NFS export configuration for the root volume. Exactly 1 item. See `nfs_exports` Block Below for details.
        #[builder(into, default)]
        pub nfs_exports: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OpenZfsVolumeNfsExports>,
        >,
        /// Specifies the configuration to use when creating the OpenZFS volume. See `origin_snapshot` Block below for details.
        #[builder(into, default)]
        pub origin_snapshot: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OpenZfsVolumeOriginSnapshot>,
        >,
        /// The volume id of volume that will be the parent volume for the volume being created, this could be the root volume created from the `aws.fsx.OpenZfsFileSystem` resource with the `root_volume_id` or the `id` property of another `aws.fsx.OpenZfsVolume`.
        #[builder(into)]
        pub parent_volume_id: pulumi_wasm_rust::Output<String>,
        /// specifies whether the volume is read-only. Default is false.
        #[builder(into, default)]
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// The record size of an OpenZFS volume, in kibibytes (KiB). Valid values are `4`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, or `1024` KiB. The default is `128` KiB.
        #[builder(into, default)]
        pub record_size_kib: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum amount of storage in gibibytes (GiB) that the volume can use from its parent.
        #[builder(into, default)]
        pub storage_capacity_quota_gib: pulumi_wasm_rust::Output<Option<i32>>,
        /// The amount of storage in gibibytes (GiB) to reserve from the parent volume.
        #[builder(into, default)]
        pub storage_capacity_reservation_gib: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specify how much storage users or groups can use on the volume. Maximum of 100 items. See `user_and_group_quotas` Block Below.
        #[builder(into, default)]
        pub user_and_group_quotas: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::fsx::OpenZfsVolumeUserAndGroupQuota>>,
        >,
        #[builder(into, default)]
        pub volume_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OpenZfsVolumeResult {
        /// Amazon Resource Name of the file system.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A boolean flag indicating whether tags for the file system should be copied to snapshots. The default value is false.
        pub copy_tags_to_snapshots: pulumi_wasm_rust::Output<Option<bool>>,
        /// Method used to compress the data on the volume. Valid values are `NONE` or `ZSTD`. Child volumes that don't specify compression option will inherit from parent volume. This option on file system applies to the root volume.
        pub data_compression_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to delete all child volumes and snapshots. Valid values: `DELETE_CHILD_VOLUMES_AND_SNAPSHOTS`. This configuration must be applied separately before attempting to delete the resource to have the desired behavior..
        pub delete_volume_options: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Volume. You can use a maximum of 203 alphanumeric characters, plus the underscore (_) special character.
        pub name: pulumi_wasm_rust::Output<String>,
        /// NFS export configuration for the root volume. Exactly 1 item. See `nfs_exports` Block Below for details.
        pub nfs_exports: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OpenZfsVolumeNfsExports>,
        >,
        /// Specifies the configuration to use when creating the OpenZFS volume. See `origin_snapshot` Block below for details.
        pub origin_snapshot: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OpenZfsVolumeOriginSnapshot>,
        >,
        /// The volume id of volume that will be the parent volume for the volume being created, this could be the root volume created from the `aws.fsx.OpenZfsFileSystem` resource with the `root_volume_id` or the `id` property of another `aws.fsx.OpenZfsVolume`.
        pub parent_volume_id: pulumi_wasm_rust::Output<String>,
        /// specifies whether the volume is read-only. Default is false.
        pub read_only: pulumi_wasm_rust::Output<bool>,
        /// The record size of an OpenZFS volume, in kibibytes (KiB). Valid values are `4`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, or `1024` KiB. The default is `128` KiB.
        pub record_size_kib: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum amount of storage in gibibytes (GiB) that the volume can use from its parent.
        pub storage_capacity_quota_gib: pulumi_wasm_rust::Output<i32>,
        /// The amount of storage in gibibytes (GiB) to reserve from the parent volume.
        pub storage_capacity_reservation_gib: pulumi_wasm_rust::Output<i32>,
        /// A map of tags to assign to the file system. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specify how much storage users or groups can use on the volume. Maximum of 100 items. See `user_and_group_quotas` Block Below.
        pub user_and_group_quotas: pulumi_wasm_rust::Output<
            Vec<super::super::types::fsx::OpenZfsVolumeUserAndGroupQuota>,
        >,
        pub volume_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OpenZfsVolumeArgs) -> OpenZfsVolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let copy_tags_to_snapshots_binding = args.copy_tags_to_snapshots.get_inner();
        let data_compression_type_binding = args.data_compression_type.get_inner();
        let delete_volume_options_binding = args.delete_volume_options.get_inner();
        let name_binding = args.name.get_inner();
        let nfs_exports_binding = args.nfs_exports.get_inner();
        let origin_snapshot_binding = args.origin_snapshot.get_inner();
        let parent_volume_id_binding = args.parent_volume_id.get_inner();
        let read_only_binding = args.read_only.get_inner();
        let record_size_kib_binding = args.record_size_kib.get_inner();
        let storage_capacity_quota_gib_binding = args
            .storage_capacity_quota_gib
            .get_inner();
        let storage_capacity_reservation_gib_binding = args
            .storage_capacity_reservation_gib
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let user_and_group_quotas_binding = args.user_and_group_quotas.get_inner();
        let volume_type_binding = args.volume_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/openZfsVolume:OpenZfsVolume".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "copyTagsToSnapshots".into(),
                    value: &copy_tags_to_snapshots_binding,
                },
                register_interface::ObjectField {
                    name: "dataCompressionType".into(),
                    value: &data_compression_type_binding,
                },
                register_interface::ObjectField {
                    name: "deleteVolumeOptions".into(),
                    value: &delete_volume_options_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nfsExports".into(),
                    value: &nfs_exports_binding,
                },
                register_interface::ObjectField {
                    name: "originSnapshot".into(),
                    value: &origin_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "parentVolumeId".into(),
                    value: &parent_volume_id_binding,
                },
                register_interface::ObjectField {
                    name: "readOnly".into(),
                    value: &read_only_binding,
                },
                register_interface::ObjectField {
                    name: "recordSizeKib".into(),
                    value: &record_size_kib_binding,
                },
                register_interface::ObjectField {
                    name: "storageCapacityQuotaGib".into(),
                    value: &storage_capacity_quota_gib_binding,
                },
                register_interface::ObjectField {
                    name: "storageCapacityReservationGib".into(),
                    value: &storage_capacity_reservation_gib_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userAndGroupQuotas".into(),
                    value: &user_and_group_quotas_binding,
                },
                register_interface::ObjectField {
                    name: "volumeType".into(),
                    value: &volume_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "copyTagsToSnapshots".into(),
                },
                register_interface::ResultField {
                    name: "dataCompressionType".into(),
                },
                register_interface::ResultField {
                    name: "deleteVolumeOptions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nfsExports".into(),
                },
                register_interface::ResultField {
                    name: "originSnapshot".into(),
                },
                register_interface::ResultField {
                    name: "parentVolumeId".into(),
                },
                register_interface::ResultField {
                    name: "readOnly".into(),
                },
                register_interface::ResultField {
                    name: "recordSizeKib".into(),
                },
                register_interface::ResultField {
                    name: "storageCapacityQuotaGib".into(),
                },
                register_interface::ResultField {
                    name: "storageCapacityReservationGib".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userAndGroupQuotas".into(),
                },
                register_interface::ResultField {
                    name: "volumeType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OpenZfsVolumeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            copy_tags_to_snapshots: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyTagsToSnapshots").unwrap(),
            ),
            data_compression_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCompressionType").unwrap(),
            ),
            delete_volume_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteVolumeOptions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nfs_exports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nfsExports").unwrap(),
            ),
            origin_snapshot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originSnapshot").unwrap(),
            ),
            parent_volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentVolumeId").unwrap(),
            ),
            read_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readOnly").unwrap(),
            ),
            record_size_kib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recordSizeKib").unwrap(),
            ),
            storage_capacity_quota_gib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageCapacityQuotaGib").unwrap(),
            ),
            storage_capacity_reservation_gib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageCapacityReservationGib").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_and_group_quotas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userAndGroupQuotas").unwrap(),
            ),
            volume_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeType").unwrap(),
            ),
        }
    }
}
