/// A volume is a file system container in a storage pool that stores application, database, and user data.
///
/// You can create a volume's capacity using the available capacity in the storage pool and you can define and resize the capacity without disruption to any processes.
///
/// Storage pool settings apply to the volumes contained within them automatically.
///
///
/// To get more information about Volume, see:
///
/// * [API documentation](https://cloud.google.com/netapp/volumes/docs/reference/rest/v1/projects.locations.volumes)
/// * How-to Guides
///     * [Documentation](https://cloud.google.com/netapp/volumes/docs/configure-and-use/volumes/overview)
///     * [Quickstart](https://cloud.google.com/netapp/volumes/docs/get-started/quickstarts/create-volume)
///
/// ## Example Usage
///
/// ### Netapp Volume Basic
///
///
/// ```yaml
/// resources:
///   defaultStoragePool:
///     type: gcp:netapp:StoragePool
///     name: default
///     properties:
///       name: test-pool
///       location: us-west2
///       serviceLevel: PREMIUM
///       capacityGib: '2048'
///       network: ${default.id}
///   testVolume:
///     type: gcp:netapp:Volume
///     name: test_volume
///     properties:
///       location: us-west2
///       name: test-volume
///       capacityGib: '100'
///       shareName: test-volume
///       storagePool: ${defaultStoragePool.name}
///       protocols:
///         - NFSV3
///       deletionPolicy: DEFAULT
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: test-network
/// ```
///
/// ## Import
///
/// Volume can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/volumes/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Volume can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/volume:Volume default projects/{{project}}/locations/{{location}}/volumes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/volume:Volume default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/volume:Volume default {{location}}/{{name}}
/// ```
///
pub mod volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeArgs {
        /// Backup configuration for the volume.
        /// Structure is documented below.
        #[builder(into, default)]
        pub backup_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeBackupConfig>,
        >,
        /// Capacity of the volume (in GiB).
        #[builder(into)]
        pub capacity_gib: pulumi_wasm_rust::InputOrOutput<String>,
        /// Policy to determine if the volume should be deleted forcefully.
        /// Volumes may have nested snapshot resources. Deleting such a volume will fail.
        /// Setting this parameter to FORCE will delete volumes including nested snapshots.
        /// Possible values: DEFAULT, FORCE.
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Export policy of the volume for NFSV3 and/or NFSV4.1 access.
        /// Structure is documented below.
        #[builder(into, default)]
        pub export_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeExportPolicy>,
        >,
        /// Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p).
        #[builder(into, default)]
        pub kerberos_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Flag indicating if the volume will be a large capacity volume or a regular volume.
        #[builder(into, default)]
        pub large_capacity: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.
        /// Only the volume with largeCapacity will be allowed to have multiple endpoints.
        #[builder(into, default)]
        pub multiple_endpoints: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the volume. Needs to be unique per location.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The protocol of the volume. Allowed combinations are `['NFSV3']`, `['NFSV4']`, `['SMB']`, `['NFSV3', 'NFSV4']`, `['SMB', 'NFSV3']` and `['SMB', 'NFSV4']`.
        /// Each value may be one of: `NFSV3`, `NFSV4`, `SMB`.
        #[builder(into)]
        pub protocols: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Used to create this volume from a snapshot (= cloning) or an backup.
        /// Structure is documented below.
        #[builder(into, default)]
        pub restore_parameters: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeRestoreParameters>,
        >,
        /// List of actions that are restricted on this volume.
        /// Each value may be one of: `DELETE`.
        #[builder(into, default)]
        pub restricted_actions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Security Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.
        /// Use NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol.
        /// Possible values are: `NTFS`, `UNIX`.
        #[builder(into, default)]
        pub security_style: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Share name (SMB) or export path (NFS) of the volume. Needs to be unique per location.
        #[builder(into)]
        pub share_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Settings for volumes with SMB access.
        /// Each value may be one of: `ENCRYPT_DATA`, `BROWSABLE`, `CHANGE_NOTIFY`, `NON_BROWSABLE`, `OPLOCKS`, `SHOW_SNAPSHOT`, `SHOW_PREVIOUS_VERSIONS`, `ACCESS_BASED_ENUMERATION`, `CONTINUOUSLY_AVAILABLE`.
        #[builder(into, default)]
        pub smb_settings: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// If enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable "Previous Versions" support for SMB.
        #[builder(into, default)]
        pub snapshot_directory: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Snapshot policy defines the schedule for automatic snapshot creation.
        /// To disable automatic snapshot creation you have to remove the whole snapshot_policy block.
        /// Structure is documented below.
        #[builder(into, default)]
        pub snapshot_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeSnapshotPolicy>,
        >,
        /// Name of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume.
        #[builder(into)]
        pub storage_pool: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tiering policy for the volume.
        /// Structure is documented below.
        #[builder(into, default)]
        pub tiering_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::VolumeTieringPolicy>,
        >,
        /// Unix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only.
        #[builder(into, default)]
        pub unix_permissions: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeResult {
        /// Reports the resource name of the Active Directory policy being used. Inherited from storage pool.
        pub active_directory: pulumi_wasm_rust::Output<String>,
        /// Backup configuration for the volume.
        /// Structure is documented below.
        pub backup_config: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::VolumeBackupConfig>,
        >,
        /// Capacity of the volume (in GiB).
        pub capacity_gib: pulumi_wasm_rust::Output<String>,
        /// Output only. Size of the volume cold tier data in GiB.
        pub cold_tier_size_gib: pulumi_wasm_rust::Output<String>,
        /// Create time of the volume. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Policy to determine if the volume should be deleted forcefully.
        /// Volumes may have nested snapshot resources. Deleting such a volume will fail.
        /// Setting this parameter to FORCE will delete volumes including nested snapshots.
        /// Possible values: DEFAULT, FORCE.
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Reports the data-at-rest encryption type of the volume. Inherited from storage pool.
        pub encryption_type: pulumi_wasm_rust::Output<String>,
        /// Export policy of the volume for NFSV3 and/or NFSV4.1 access.
        /// Structure is documented below.
        pub export_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::VolumeExportPolicy>,
        >,
        /// Indicates whether the volume is part of a volume replication relationship.
        pub has_replication: pulumi_wasm_rust::Output<bool>,
        /// Flag indicating if the volume is a kerberos volume or not, export policy rules control kerberos security modes (krb5, krb5i, krb5p).
        pub kerberos_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Reports the CMEK policy resurce name being used for volume encryption. Inherited from storage pool.
        pub kms_config: pulumi_wasm_rust::Output<String>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Flag indicating if the volume will be a large capacity volume or a regular volume.
        pub large_capacity: pulumi_wasm_rust::Output<Option<bool>>,
        /// Flag indicating if the volume is NFS LDAP enabled or not. Inherited from storage pool.
        pub ldap_enabled: pulumi_wasm_rust::Output<bool>,
        /// Name of the pool location. Usually a region name, expect for some STANDARD service level pools which require a zone name.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Reports mount instructions for this volume.
        /// Structure is documented below.
        pub mount_options: pulumi_wasm_rust::Output<
            Vec<super::super::types::netapp::VolumeMountOption>,
        >,
        /// Optional. Flag indicating if the volume will have an IP address per node for volumes supporting multiple IP endpoints.
        /// Only the volume with largeCapacity will be allowed to have multiple endpoints.
        pub multiple_endpoints: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the volume. Needs to be unique per location.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// VPC network name with format: `projects/{{project}}/global/networks/{{network}}`. Inherited from storage pool.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The protocol of the volume. Allowed combinations are `['NFSV3']`, `['NFSV4']`, `['SMB']`, `['NFSV3', 'NFSV4']`, `['SMB', 'NFSV3']` and `['SMB', 'NFSV4']`.
        /// Each value may be one of: `NFSV3`, `NFSV4`, `SMB`.
        pub protocols: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the Private Service Access allocated range. Inherited from storage pool.
        pub psa_range: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the replica zone for regional volume.
        pub replica_zone: pulumi_wasm_rust::Output<String>,
        /// Used to create this volume from a snapshot (= cloning) or an backup.
        /// Structure is documented below.
        pub restore_parameters: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::VolumeRestoreParameters>,
        >,
        /// List of actions that are restricted on this volume.
        /// Each value may be one of: `DELETE`.
        pub restricted_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Security Style of the Volume. Use UNIX to use UNIX or NFSV4 ACLs for file permissions.
        /// Use NTFS to use NTFS ACLs for file permissions. Can only be set for volumes which use SMB together with NFS as protocol.
        /// Possible values are: `NTFS`, `UNIX`.
        pub security_style: pulumi_wasm_rust::Output<String>,
        /// Service level of the volume. Inherited from storage pool. Supported values are : PREMIUM, EXTREME, STANDARD, FLEX.
        pub service_level: pulumi_wasm_rust::Output<String>,
        /// Share name (SMB) or export path (NFS) of the volume. Needs to be unique per location.
        pub share_name: pulumi_wasm_rust::Output<String>,
        /// Settings for volumes with SMB access.
        /// Each value may be one of: `ENCRYPT_DATA`, `BROWSABLE`, `CHANGE_NOTIFY`, `NON_BROWSABLE`, `OPLOCKS`, `SHOW_SNAPSHOT`, `SHOW_PREVIOUS_VERSIONS`, `ACCESS_BASED_ENUMERATION`, `CONTINUOUSLY_AVAILABLE`.
        pub smb_settings: pulumi_wasm_rust::Output<Vec<String>>,
        /// If enabled, a NFS volume will contain a read-only .snapshot directory which provides access to each of the volume's snapshots. Will enable "Previous Versions" support for SMB.
        pub snapshot_directory: pulumi_wasm_rust::Output<Option<bool>>,
        /// Snapshot policy defines the schedule for automatic snapshot creation.
        /// To disable automatic snapshot creation you have to remove the whole snapshot_policy block.
        /// Structure is documented below.
        pub snapshot_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::VolumeSnapshotPolicy>,
        >,
        /// State of the volume.
        pub state: pulumi_wasm_rust::Output<String>,
        /// State details of the volume.
        pub state_details: pulumi_wasm_rust::Output<String>,
        /// Name of the storage pool to create the volume in. Pool needs enough spare capacity to accommodate the volume.
        pub storage_pool: pulumi_wasm_rust::Output<String>,
        /// Tiering policy for the volume.
        /// Structure is documented below.
        pub tiering_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::VolumeTieringPolicy>,
        >,
        /// Unix permission the mount point will be created with. Default is 0770. Applicable for UNIX security style volumes only.
        pub unix_permissions: pulumi_wasm_rust::Output<String>,
        /// Used capacity of the volume (in GiB). This is computed periodically and it does not represent the realtime usage.
        pub used_gib: pulumi_wasm_rust::Output<String>,
        /// Specifies the active zone for regional volume.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VolumeArgs,
    ) -> VolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_config_binding = args.backup_config.get_output(context).get_inner();
        let capacity_gib_binding = args.capacity_gib.get_output(context).get_inner();
        let deletion_policy_binding = args
            .deletion_policy
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let export_policy_binding = args.export_policy.get_output(context).get_inner();
        let kerberos_enabled_binding = args
            .kerberos_enabled
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let large_capacity_binding = args.large_capacity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let multiple_endpoints_binding = args
            .multiple_endpoints
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let protocols_binding = args.protocols.get_output(context).get_inner();
        let restore_parameters_binding = args
            .restore_parameters
            .get_output(context)
            .get_inner();
        let restricted_actions_binding = args
            .restricted_actions
            .get_output(context)
            .get_inner();
        let security_style_binding = args.security_style.get_output(context).get_inner();
        let share_name_binding = args.share_name.get_output(context).get_inner();
        let smb_settings_binding = args.smb_settings.get_output(context).get_inner();
        let snapshot_directory_binding = args
            .snapshot_directory
            .get_output(context)
            .get_inner();
        let snapshot_policy_binding = args
            .snapshot_policy
            .get_output(context)
            .get_inner();
        let storage_pool_binding = args.storage_pool.get_output(context).get_inner();
        let tiering_policy_binding = args.tiering_policy.get_output(context).get_inner();
        let unix_permissions_binding = args
            .unix_permissions
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:netapp/volume:Volume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupConfig".into(),
                    value: &backup_config_binding,
                },
                register_interface::ObjectField {
                    name: "capacityGib".into(),
                    value: &capacity_gib_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "exportPolicy".into(),
                    value: &export_policy_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosEnabled".into(),
                    value: &kerberos_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "largeCapacity".into(),
                    value: &large_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "multipleEndpoints".into(),
                    value: &multiple_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "protocols".into(),
                    value: &protocols_binding,
                },
                register_interface::ObjectField {
                    name: "restoreParameters".into(),
                    value: &restore_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "restrictedActions".into(),
                    value: &restricted_actions_binding,
                },
                register_interface::ObjectField {
                    name: "securityStyle".into(),
                    value: &security_style_binding,
                },
                register_interface::ObjectField {
                    name: "shareName".into(),
                    value: &share_name_binding,
                },
                register_interface::ObjectField {
                    name: "smbSettings".into(),
                    value: &smb_settings_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotDirectory".into(),
                    value: &snapshot_directory_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotPolicy".into(),
                    value: &snapshot_policy_binding,
                },
                register_interface::ObjectField {
                    name: "storagePool".into(),
                    value: &storage_pool_binding,
                },
                register_interface::ObjectField {
                    name: "tieringPolicy".into(),
                    value: &tiering_policy_binding,
                },
                register_interface::ObjectField {
                    name: "unixPermissions".into(),
                    value: &unix_permissions_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VolumeResult {
            active_directory: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeDirectory"),
            ),
            backup_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupConfig"),
            ),
            capacity_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityGib"),
            ),
            cold_tier_size_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("coldTierSizeGib"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            encryption_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionType"),
            ),
            export_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exportPolicy"),
            ),
            has_replication: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hasReplication"),
            ),
            kerberos_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kerberosEnabled"),
            ),
            kms_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsConfig"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            large_capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("largeCapacity"),
            ),
            ldap_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ldapEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mount_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mountOptions"),
            ),
            multiple_endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multipleEndpoints"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            protocols: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protocols"),
            ),
            psa_range: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("psaRange"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            replica_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicaZone"),
            ),
            restore_parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restoreParameters"),
            ),
            restricted_actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restrictedActions"),
            ),
            security_style: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityStyle"),
            ),
            service_level: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceLevel"),
            ),
            share_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shareName"),
            ),
            smb_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("smbSettings"),
            ),
            snapshot_directory: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotDirectory"),
            ),
            snapshot_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotPolicy"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            state_details: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stateDetails"),
            ),
            storage_pool: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storagePool"),
            ),
            tiering_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tieringPolicy"),
            ),
            unix_permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("unixPermissions"),
            ),
            used_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("usedGib"),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
