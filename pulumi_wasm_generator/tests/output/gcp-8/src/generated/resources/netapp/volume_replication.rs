/// ## Example Usage
///
/// ### Netapp Volume Replication Create
///
///
/// ```yaml
/// resources:
///   sourcePool:
///     type: gcp:netapp:StoragePool
///     name: source_pool
///     properties:
///       name: source-pool
///       location: us-central1
///       serviceLevel: PREMIUM
///       capacityGib: 2048
///       network: ${default.id}
///   destinationPool:
///     type: gcp:netapp:StoragePool
///     name: destination_pool
///     properties:
///       name: destination-pool
///       location: us-west2
///       serviceLevel: PREMIUM
///       capacityGib: 2048
///       network: ${default.id}
///   sourceVolume:
///     type: gcp:netapp:Volume
///     name: source_volume
///     properties:
///       location: ${sourcePool.location}
///       name: source-volume
///       capacityGib: 100
///       shareName: source-volume
///       storagePool: ${sourcePool.name}
///       protocols:
///         - NFSV3
///       deletionPolicy: FORCE
///   testReplication:
///     type: gcp:netapp:VolumeReplication
///     name: test_replication
///     properties:
///       location: ${sourceVolume.location}
///       volumeName: ${sourceVolume.name}
///       name: test-replication
///       replicationSchedule: EVERY_10_MINUTES
///       description: This is a replication resource
///       destinationVolumeParameters:
///         storagePool: ${destinationPool.id}
///         volumeId: destination-volume
///         shareName: source-volume
///         description: This is a replicated volume
///       deleteDestinationVolume: true
///       waitForMirror: true
///     options:
///       dependsOn:
///         - ${sourceVolume}
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
/// VolumeReplication can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/volumes/{{volume_name}}/replications/{{name}}`
///
/// * `{{project}}/{{location}}/{{volume_name}}/{{name}}`
///
/// * `{{location}}/{{volume_name}}/{{name}}`
///
/// When using the `pulumi import` command, VolumeReplication can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/volumeReplication:VolumeReplication default projects/{{project}}/locations/{{location}}/volumes/{{volume_name}}/replications/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/volumeReplication:VolumeReplication default {{project}}/{{location}}/{{volume_name}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/volumeReplication:VolumeReplication default {{location}}/{{volume_name}}/{{name}}
/// ```
///
pub mod volume_replication {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeReplicationArgs {
        #[builder(into, default)]
        pub delete_destination_volume: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// An description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Destination volume parameters.
        /// Structure is documented below.
        #[builder(into, default)]
        pub destination_volume_parameters: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::netapp::VolumeReplicationDestinationVolumeParameters,
            >,
        >,
        /// Only replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING
        /// currently receives an update and stopping the update might be undesirable. Set this parameter to true
        /// to stop anyway. All data transferred to the destination will be discarded and content of destination
        /// volume will remain at the state of the last successful update. Default is false.
        #[builder(into, default)]
        pub force_stopping: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of region for this resource. The resource needs to be created in the region of the destination volume.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the replication. Needs to be unique per location.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Set to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write
        /// and act independently from the source volume.
        /// Set to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes
        /// done to the destination volume with the content of the source volume.
        #[builder(into, default)]
        pub replication_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the replication interval.
        /// Possible values are: `EVERY_10_MINUTES`, `HOURLY`, `DAILY`.
        #[builder(into)]
        pub replication_schedule: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the existing source volume.
        #[builder(into)]
        pub volume_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub wait_for_mirror: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct VolumeReplicationResult {
        /// Create time of the active directory. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub delete_destination_volume: pulumi_wasm_rust::Output<Option<bool>>,
        /// An description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Full resource name of destination volume with format: `projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}`
        pub destination_volume: pulumi_wasm_rust::Output<String>,
        /// Destination volume parameters.
        /// Structure is documented below.
        pub destination_volume_parameters: pulumi_wasm_rust::Output<
            Option<
                super::super::types::netapp::VolumeReplicationDestinationVolumeParameters,
            >,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Only replications with mirror_state=MIRRORED can be stopped. A replication in mirror_state=TRANSFERRING
        /// currently receives an update and stopping the update might be undesirable. Set this parameter to true
        /// to stop anyway. All data transferred to the destination will be discarded and content of destination
        /// volume will remain at the state of the last successful update. Default is false.
        pub force_stopping: pulumi_wasm_rust::Output<Option<bool>>,
        /// Condition of the relationship. Can be one of the following:
        /// - true: The replication relationship is healthy. It has not missed the most recent scheduled transfer.
        /// - false: The replication relationship is not healthy. It has missed the most recent scheduled transfer.
        pub healthy: pulumi_wasm_rust::Output<bool>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of region for this resource. The resource needs to be created in the region of the destination volume.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Indicates the state of the mirror between source and destination volumes. Depending on the amount of data
        /// in your source volume, PREPARING phase can take hours or days. mirrorState = MIRRORED indicates your baseline
        /// transfer ended and destination volume became accessible read-only. TRANSFERRING means a MIRRORED volume
        /// currently receives an update. Updated every 5 minutes.
        pub mirror_state: pulumi_wasm_rust::Output<String>,
        /// The name of the replication. Needs to be unique per location.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set to false to stop/break the mirror. Stopping the mirror makes the destination volume read-write
        /// and act independently from the source volume.
        /// Set to true to enable/resume the mirror. WARNING: Resuming a mirror overwrites any changes
        /// done to the destination volume with the content of the source volume.
        pub replication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the replication interval.
        /// Possible values are: `EVERY_10_MINUTES`, `HOURLY`, `DAILY`.
        pub replication_schedule: pulumi_wasm_rust::Output<String>,
        /// Reverting a replication can swap source and destination volume roles. This field indicates if the `location` hosts
        /// the source or destination volume. For resume and revert and resume operations it is critical to understand
        /// which volume is the source volume, since it will overwrite changes done to the destination volume.
        pub role: pulumi_wasm_rust::Output<String>,
        /// Full resource name of source volume with format: `projects/{{project}}/locations/{{location}}/volumes/{{volumeId}}`
        pub source_volume: pulumi_wasm_rust::Output<String>,
        /// Indicates the state of replication resource. State of the mirror itself is indicated in mirrorState.
        pub state: pulumi_wasm_rust::Output<String>,
        /// State details of the replication resource.
        pub state_details: pulumi_wasm_rust::Output<String>,
        /// Replication transfer statistics. All statistics are updated every 5 minutes.
        /// Structure is documented below.
        pub transfer_stats: pulumi_wasm_rust::Output<
            Vec<super::super::types::netapp::VolumeReplicationTransferStat>,
        >,
        /// The name of the existing source volume.
        pub volume_name: pulumi_wasm_rust::Output<String>,
        pub wait_for_mirror: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VolumeReplicationArgs,
    ) -> VolumeReplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delete_destination_volume_binding = args
            .delete_destination_volume
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let destination_volume_parameters_binding = args
            .destination_volume_parameters
            .get_output(context)
            .get_inner();
        let force_stopping_binding = args.force_stopping.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let replication_enabled_binding = args
            .replication_enabled
            .get_output(context)
            .get_inner();
        let replication_schedule_binding = args
            .replication_schedule
            .get_output(context)
            .get_inner();
        let volume_name_binding = args.volume_name.get_output(context).get_inner();
        let wait_for_mirror_binding = args
            .wait_for_mirror
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:netapp/volumeReplication:VolumeReplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deleteDestinationVolume".into(),
                    value: &delete_destination_volume_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destinationVolumeParameters".into(),
                    value: &destination_volume_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "forceStopping".into(),
                    value: &force_stopping_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "replicationEnabled".into(),
                    value: &replication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "replicationSchedule".into(),
                    value: &replication_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "volumeName".into(),
                    value: &volume_name_binding,
                },
                register_interface::ObjectField {
                    name: "waitForMirror".into(),
                    value: &wait_for_mirror_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VolumeReplicationResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            delete_destination_volume: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deleteDestinationVolume"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destination_volume: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationVolume"),
            ),
            destination_volume_parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationVolumeParameters"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            force_stopping: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceStopping"),
            ),
            healthy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthy"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mirror_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mirrorState"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            replication_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationEnabled"),
            ),
            replication_schedule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationSchedule"),
            ),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
            source_volume: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceVolume"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            state_details: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stateDetails"),
            ),
            transfer_stats: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transferStats"),
            ),
            volume_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeName"),
            ),
            wait_for_mirror: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitForMirror"),
            ),
        }
    }
}
