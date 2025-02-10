/// NetApp Volumes helps you manage your data usage with snapshots that can quickly restore lost data.
/// Snapshots are point-in-time versions of your volume's content. They are resources of volumes and are
/// instant captures of your data that consume space only for modified data. Because data changes over
/// time, snapshots usually consume more space as they get older.
/// NetApp Volumes volumes use just-in-time copy-on-write so that unmodified files in snapshots don't
/// consume any of the volume's capacity.
///
///
/// To get more information about VolumeSnapshot, see:
///
/// * [API documentation](https://cloud.google.com/netapp/volumes/docs/reference/rest/v1/projects.locations.volumes.snapshots)
/// * How-to Guides
///     * [Documentation](https://cloud.google.com/netapp/volumes/docs/configure-and-use/volume-snapshots/overview)
///
/// ## Example Usage
///
/// ### Volume Snapshot Create
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
///       capacityGib: 2048
///       network: ${default.id}
///   defaultVolume:
///     type: gcp:netapp:Volume
///     name: default
///     properties:
///       location: ${defaultStoragePool.location}
///       name: test-volume
///       capacityGib: 100
///       shareName: test-volume
///       storagePool: ${defaultStoragePool.name}
///       protocols:
///         - NFSV3
///   testSnapshot:
///     type: gcp:netapp:VolumeSnapshot
///     name: test_snapshot
///     properties:
///       location: ${defaultVolume.location}
///       volumeName: ${defaultVolume.name}
///       name: testvolumesnap
///     options:
///       dependsOn:
///         - ${defaultVolume}
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
/// VolumeSnapshot can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/volumes/{{volume_name}}/snapshots/{{name}}`
///
/// * `{{project}}/{{location}}/{{volume_name}}/{{name}}`
///
/// * `{{location}}/{{volume_name}}/{{name}}`
///
/// When using the `pulumi import` command, VolumeSnapshot can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/volumeSnapshot:VolumeSnapshot default projects/{{project}}/locations/{{location}}/volumes/{{volume_name}}/snapshots/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/volumeSnapshot:VolumeSnapshot default {{project}}/{{location}}/{{volume_name}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/volumeSnapshot:VolumeSnapshot default {{location}}/{{volume_name}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod volume_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeSnapshotArgs {
        /// Description for the snapshot.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the snapshot location. Snapshots are child resources of volumes and live in the same location.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the snapshot.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the volume to create the snapshot in.
        #[builder(into)]
        pub volume_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VolumeSnapshotResult {
        /// Description for the snapshot.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the snapshot location. Snapshots are child resources of volumes and live in the same location.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the snapshot.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Storage used to store blocks unique to this snapshot.
        pub used_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The name of the volume to create the snapshot in.
        pub volume_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VolumeSnapshotArgs,
    ) -> VolumeSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let volume_name_binding = args.volume_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:netapp/volumeSnapshot:VolumeSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeName".into(),
                    value: volume_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VolumeSnapshotResult {
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            used_bytes: o.get_field("usedBytes"),
            volume_name: o.get_field("volumeName"),
        }
    }
}
