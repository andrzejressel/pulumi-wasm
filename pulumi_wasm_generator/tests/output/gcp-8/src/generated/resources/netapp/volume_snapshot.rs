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
pub mod volume_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeSnapshotArgs {
        /// Description for the snapshot.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the snapshot location. Snapshots are child resources of volumes and live in the same location.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the snapshot.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the volume to create the snapshot in.
        #[builder(into)]
        pub volume_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VolumeSnapshotResult {
        /// Description for the snapshot.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the snapshot location. Snapshots are child resources of volumes and live in the same location.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the snapshot.
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
        /// Storage used to store blocks unique to this snapshot.
        pub used_bytes: pulumi_wasm_rust::Output<i32>,
        /// The name of the volume to create the snapshot in.
        pub volume_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VolumeSnapshotArgs,
    ) -> VolumeSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let volume_name_binding = args.volume_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:netapp/volumeSnapshot:VolumeSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "volumeName".into(),
                    value: &volume_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VolumeSnapshotResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            used_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("usedBytes"),
            ),
            volume_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeName"),
            ),
        }
    }
}
