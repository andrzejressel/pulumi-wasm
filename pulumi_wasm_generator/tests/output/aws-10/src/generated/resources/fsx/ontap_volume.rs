/// Manages a FSx ONTAP Volume.
/// See the [FSx ONTAP User Guide](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-volumes.html) for more information.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = ontap_volume::create(
///         "test",
///         OntapVolumeArgs::builder()
///             .junction_path("/test")
///             .name("test")
///             .size_in_megabytes(1024)
///             .storage_efficiency_enabled(true)
///             .storage_virtual_machine_id("${testAwsFsxOntapStorageVirtualMachine.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using Tiering Policy
///
/// Additional information on tiering policy with ONTAP Volumes can be found in the [FSx ONTAP Guide](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-volumes.html).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = ontap_volume::create(
///         "test",
///         OntapVolumeArgs::builder()
///             .junction_path("/test")
///             .name("test")
///             .size_in_megabytes(1024)
///             .storage_efficiency_enabled(true)
///             .storage_virtual_machine_id("${testAwsFsxOntapStorageVirtualMachine.id}")
///             .tiering_policy(
///                 OntapVolumeTieringPolicy::builder()
///                     .coolingPeriod(31)
///                     .name("AUTO")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import FSx ONTAP volume using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:fsx/ontapVolume:OntapVolume example fsvol-12345678abcdef123
/// ```
pub mod ontap_volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OntapVolumeArgs {
        /// The Aggregate configuration only applies to `FLEXGROUP` volumes. See [`aggregate_configuration` Block] for details.
        #[builder(into, default)]
        pub aggregate_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::fsx::OntapVolumeAggregateConfiguration>,
        >,
        /// Setting this to `true` allows a SnapLock administrator to delete an FSx for ONTAP SnapLock Enterprise volume with unexpired write once, read many (WORM) files. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        #[builder(into, default)]
        pub bypass_snaplock_enterprise_retention: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A boolean flag indicating whether tags for the volume should be copied to backups. This value defaults to `false`.
        #[builder(into, default)]
        pub copy_tags_to_backups: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to apply to the volume's final backup.
        #[builder(into, default)]
        pub final_backup_tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the location in the storage virtual machine's namespace where the volume is mounted. The junction_path must have a leading forward slash, such as `/vol3`
        #[builder(into, default)]
        pub junction_path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Volume. You can use a maximum of 203 alphanumeric characters, plus the underscore (_) special character.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the type of volume, valid values are `RW`, `DP`. Default value is `RW`. These can be set by the ONTAP CLI or API. This setting is used as part of migration and replication [Migrating to Amazon FSx for NetApp ONTAP](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/migrating-fsx-ontap.html)
        #[builder(into, default)]
        pub ontap_volume_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the volume security style, Valid values are `UNIX`, `NTFS`, and `MIXED`.
        #[builder(into, default)]
        pub security_style: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the size of the volume, in megabytes (MB), that you are creating. Can be used for any size but required for volumes over 2 PB. Either size_in_bytes or size_in_megabytes must be specified. Minimum size for `FLEXGROUP` volumes are 100GiB per constituent.
        #[builder(into, default)]
        pub size_in_bytes: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the size of the volume, in megabytes (MB), that you are creating. Supported when creating volumes under 2 PB. Either size_in_bytes or size_in_megabytes must be specified. Minimum size for `FLEXGROUP` volumes are 100GiB per constituent.
        #[builder(into, default)]
        pub size_in_megabytes: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// When enabled, will skip the default final backup taken when the volume is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        #[builder(into, default)]
        pub skip_final_backup: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The SnapLock configuration for an FSx for ONTAP volume. See `snaplock_configuration` Block for details.
        #[builder(into, default)]
        pub snaplock_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::fsx::OntapVolumeSnaplockConfiguration>,
        >,
        /// Specifies the snapshot policy for the volume. See [snapshot policies](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/snapshots-ontap.html#snapshot-policies) in the Amazon FSx ONTAP User Guide
        #[builder(into, default)]
        pub snapshot_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Set to true to enable deduplication, compression, and compaction storage efficiency features on the volume.
        #[builder(into, default)]
        pub storage_efficiency_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the storage virtual machine in which to create the volume.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub storage_virtual_machine_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the volume. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The data tiering policy for an FSx for ONTAP volume. See `tiering_policy` Block for details.
        #[builder(into, default)]
        pub tiering_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::fsx::OntapVolumeTieringPolicy>,
        >,
        /// Specifies the styles of volume, valid values are `FLEXVOL`, `FLEXGROUP`. Default value is `FLEXVOL`. FLEXGROUPS have a larger minimum and maximum size. See Volume Styles for more details. [Volume Styles](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/volume-styles.html)
        #[builder(into, default)]
        pub volume_style: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The type of volume, currently the only valid value is `ONTAP`.
        #[builder(into, default)]
        pub volume_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OntapVolumeResult {
        /// The Aggregate configuration only applies to `FLEXGROUP` volumes. See [`aggregate_configuration` Block] for details.
        pub aggregate_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OntapVolumeAggregateConfiguration>,
        >,
        /// Amazon Resource Name of the volune.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Setting this to `true` allows a SnapLock administrator to delete an FSx for ONTAP SnapLock Enterprise volume with unexpired write once, read many (WORM) files. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        pub bypass_snaplock_enterprise_retention: pulumi_wasm_rust::Output<Option<bool>>,
        /// A boolean flag indicating whether tags for the volume should be copied to backups. This value defaults to `false`.
        pub copy_tags_to_backups: pulumi_wasm_rust::Output<Option<bool>>,
        /// Describes the file system for the volume, e.g. `fs-12345679`
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to apply to the volume's final backup.
        pub final_backup_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the FlexCache endpoint type of the volume, Valid values are `NONE`, `ORIGIN`, `CACHE`. Default value is `NONE`. These can be set by the ONTAP CLI or API and are use with FlexCache feature.
        pub flexcache_endpoint_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the location in the storage virtual machine's namespace where the volume is mounted. The junction_path must have a leading forward slash, such as `/vol3`
        pub junction_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Volume. You can use a maximum of 203 alphanumeric characters, plus the underscore (_) special character.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the type of volume, valid values are `RW`, `DP`. Default value is `RW`. These can be set by the ONTAP CLI or API. This setting is used as part of migration and replication [Migrating to Amazon FSx for NetApp ONTAP](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/migrating-fsx-ontap.html)
        pub ontap_volume_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the volume security style, Valid values are `UNIX`, `NTFS`, and `MIXED`.
        pub security_style: pulumi_wasm_rust::Output<String>,
        /// Specifies the size of the volume, in megabytes (MB), that you are creating. Can be used for any size but required for volumes over 2 PB. Either size_in_bytes or size_in_megabytes must be specified. Minimum size for `FLEXGROUP` volumes are 100GiB per constituent.
        pub size_in_bytes: pulumi_wasm_rust::Output<String>,
        /// Specifies the size of the volume, in megabytes (MB), that you are creating. Supported when creating volumes under 2 PB. Either size_in_bytes or size_in_megabytes must be specified. Minimum size for `FLEXGROUP` volumes are 100GiB per constituent.
        pub size_in_megabytes: pulumi_wasm_rust::Output<i32>,
        /// When enabled, will skip the default final backup taken when the volume is deleted. This configuration must be applied separately before attempting to delete the resource to have the desired behavior. Defaults to `false`.
        pub skip_final_backup: pulumi_wasm_rust::Output<Option<bool>>,
        /// The SnapLock configuration for an FSx for ONTAP volume. See `snaplock_configuration` Block for details.
        pub snaplock_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OntapVolumeSnaplockConfiguration>,
        >,
        /// Specifies the snapshot policy for the volume. See [snapshot policies](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/snapshots-ontap.html#snapshot-policies) in the Amazon FSx ONTAP User Guide
        pub snapshot_policy: pulumi_wasm_rust::Output<String>,
        /// Set to true to enable deduplication, compression, and compaction storage efficiency features on the volume.
        pub storage_efficiency_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the storage virtual machine in which to create the volume.
        ///
        /// The following arguments are optional:
        pub storage_virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the volume. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The data tiering policy for an FSx for ONTAP volume. See `tiering_policy` Block for details.
        pub tiering_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::fsx::OntapVolumeTieringPolicy>,
        >,
        /// The Volume's UUID (universally unique identifier).
        pub uuid: pulumi_wasm_rust::Output<String>,
        /// Specifies the styles of volume, valid values are `FLEXVOL`, `FLEXGROUP`. Default value is `FLEXVOL`. FLEXGROUPS have a larger minimum and maximum size. See Volume Styles for more details. [Volume Styles](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/volume-styles.html)
        pub volume_style: pulumi_wasm_rust::Output<String>,
        /// The type of volume, currently the only valid value is `ONTAP`.
        pub volume_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OntapVolumeArgs,
    ) -> OntapVolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aggregate_configuration_binding = args
            .aggregate_configuration
            .get_output(context)
            .get_inner();
        let bypass_snaplock_enterprise_retention_binding = args
            .bypass_snaplock_enterprise_retention
            .get_output(context)
            .get_inner();
        let copy_tags_to_backups_binding = args
            .copy_tags_to_backups
            .get_output(context)
            .get_inner();
        let final_backup_tags_binding = args
            .final_backup_tags
            .get_output(context)
            .get_inner();
        let junction_path_binding = args.junction_path.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let ontap_volume_type_binding = args
            .ontap_volume_type
            .get_output(context)
            .get_inner();
        let security_style_binding = args.security_style.get_output(context).get_inner();
        let size_in_bytes_binding = args.size_in_bytes.get_output(context).get_inner();
        let size_in_megabytes_binding = args
            .size_in_megabytes
            .get_output(context)
            .get_inner();
        let skip_final_backup_binding = args
            .skip_final_backup
            .get_output(context)
            .get_inner();
        let snaplock_configuration_binding = args
            .snaplock_configuration
            .get_output(context)
            .get_inner();
        let snapshot_policy_binding = args
            .snapshot_policy
            .get_output(context)
            .get_inner();
        let storage_efficiency_enabled_binding = args
            .storage_efficiency_enabled
            .get_output(context)
            .get_inner();
        let storage_virtual_machine_id_binding = args
            .storage_virtual_machine_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tiering_policy_binding = args.tiering_policy.get_output(context).get_inner();
        let volume_style_binding = args.volume_style.get_output(context).get_inner();
        let volume_type_binding = args.volume_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fsx/ontapVolume:OntapVolume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aggregateConfiguration".into(),
                    value: &aggregate_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "bypassSnaplockEnterpriseRetention".into(),
                    value: &bypass_snaplock_enterprise_retention_binding,
                },
                register_interface::ObjectField {
                    name: "copyTagsToBackups".into(),
                    value: &copy_tags_to_backups_binding,
                },
                register_interface::ObjectField {
                    name: "finalBackupTags".into(),
                    value: &final_backup_tags_binding,
                },
                register_interface::ObjectField {
                    name: "junctionPath".into(),
                    value: &junction_path_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ontapVolumeType".into(),
                    value: &ontap_volume_type_binding,
                },
                register_interface::ObjectField {
                    name: "securityStyle".into(),
                    value: &security_style_binding,
                },
                register_interface::ObjectField {
                    name: "sizeInBytes".into(),
                    value: &size_in_bytes_binding,
                },
                register_interface::ObjectField {
                    name: "sizeInMegabytes".into(),
                    value: &size_in_megabytes_binding,
                },
                register_interface::ObjectField {
                    name: "skipFinalBackup".into(),
                    value: &skip_final_backup_binding,
                },
                register_interface::ObjectField {
                    name: "snaplockConfiguration".into(),
                    value: &snaplock_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotPolicy".into(),
                    value: &snapshot_policy_binding,
                },
                register_interface::ObjectField {
                    name: "storageEfficiencyEnabled".into(),
                    value: &storage_efficiency_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "storageVirtualMachineId".into(),
                    value: &storage_virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tieringPolicy".into(),
                    value: &tiering_policy_binding,
                },
                register_interface::ObjectField {
                    name: "volumeStyle".into(),
                    value: &volume_style_binding,
                },
                register_interface::ObjectField {
                    name: "volumeType".into(),
                    value: &volume_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aggregateConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bypassSnaplockEnterpriseRetention".into(),
                },
                register_interface::ResultField {
                    name: "copyTagsToBackups".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "finalBackupTags".into(),
                },
                register_interface::ResultField {
                    name: "flexcacheEndpointType".into(),
                },
                register_interface::ResultField {
                    name: "junctionPath".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ontapVolumeType".into(),
                },
                register_interface::ResultField {
                    name: "securityStyle".into(),
                },
                register_interface::ResultField {
                    name: "sizeInBytes".into(),
                },
                register_interface::ResultField {
                    name: "sizeInMegabytes".into(),
                },
                register_interface::ResultField {
                    name: "skipFinalBackup".into(),
                },
                register_interface::ResultField {
                    name: "snaplockConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "snapshotPolicy".into(),
                },
                register_interface::ResultField {
                    name: "storageEfficiencyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "storageVirtualMachineId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tieringPolicy".into(),
                },
                register_interface::ResultField {
                    name: "uuid".into(),
                },
                register_interface::ResultField {
                    name: "volumeStyle".into(),
                },
                register_interface::ResultField {
                    name: "volumeType".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OntapVolumeResult {
            aggregate_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aggregateConfiguration").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bypass_snaplock_enterprise_retention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bypassSnaplockEnterpriseRetention").unwrap(),
            ),
            copy_tags_to_backups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("copyTagsToBackups").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            final_backup_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalBackupTags").unwrap(),
            ),
            flexcache_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flexcacheEndpointType").unwrap(),
            ),
            junction_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("junctionPath").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            ontap_volume_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ontapVolumeType").unwrap(),
            ),
            security_style: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityStyle").unwrap(),
            ),
            size_in_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sizeInBytes").unwrap(),
            ),
            size_in_megabytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sizeInMegabytes").unwrap(),
            ),
            skip_final_backup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipFinalBackup").unwrap(),
            ),
            snaplock_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snaplockConfiguration").unwrap(),
            ),
            snapshot_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotPolicy").unwrap(),
            ),
            storage_efficiency_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEfficiencyEnabled").unwrap(),
            ),
            storage_virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageVirtualMachineId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tiering_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tieringPolicy").unwrap(),
            ),
            uuid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uuid").unwrap(),
            ),
            volume_style: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeStyle").unwrap(),
            ),
            volume_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeType").unwrap(),
            ),
        }
    }
}
