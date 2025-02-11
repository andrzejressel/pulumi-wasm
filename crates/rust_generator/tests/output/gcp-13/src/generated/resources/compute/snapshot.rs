/// Represents a Persistent Disk Snapshot resource.
///
/// Use snapshots to back up data from your persistent disks. Snapshots are
/// different from public images and custom images, which are used primarily
/// to create instances or configure instance templates. Snapshots are useful
/// for periodic backup of the data on your persistent disks. You can create
/// snapshots from persistent disks even while they are attached to running
/// instances.
///
/// Snapshots are incremental, so you can create regular snapshots on a
/// persistent disk faster and at a much lower cost than if you regularly
/// created a full image of the disk.
///
///
/// To get more information about Snapshot, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/snapshots)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/disks/create-snapshots)
///
///
///
/// ## Example Usage
///
/// ### Snapshot Basic
///
///
/// ```yaml
/// resources:
///   snapshot:
///     type: gcp:compute:Snapshot
///     properties:
///       name: my-snapshot
///       sourceDisk: ${persistent.id}
///       zone: us-central1-a
///       labels:
///         my_label: value
///       storageLocations:
///         - us-central1
///   persistent:
///     type: gcp:compute:Disk
///     properties:
///       name: debian-disk
///       image: ${debian.selfLink}
///       size: 10
///       type: pd-ssd
///       zone: us-central1-a
/// variables:
///   debian:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Snapshot Chainname
///
///
/// ```yaml
/// resources:
///   snapshot:
///     type: gcp:compute:Snapshot
///     properties:
///       name: my-snapshot
///       sourceDisk: ${persistent.id}
///       zone: us-central1-a
///       chainName: snapshot-chain
///       labels:
///         my_label: value
///       storageLocations:
///         - us-central1
///   persistent:
///     type: gcp:compute:Disk
///     properties:
///       name: debian-disk
///       image: ${debian.selfLink}
///       size: 10
///       type: pd-ssd
///       zone: us-central1-a
/// variables:
///   debian:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
///
/// ## Import
///
/// Snapshot can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/snapshots/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Snapshot can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/snapshot:Snapshot default projects/{{project}}/global/snapshots/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/snapshot:Snapshot default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/snapshot:Snapshot default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// Creates the new snapshot in the snapshot chain labeled with the
        /// specified name. The chain name must be 1-63 characters long and
        /// comply with RFC1035. This is an uncommon option only for advanced
        /// service owners who needs to create separate snapshot chains, for
        /// example, for chargeback tracking.  When you describe your snapshot
        /// resource, this field is visible only if it has a non-empty value.
        #[builder(into, default)]
        pub chain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels to apply to this Snapshot.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Encrypts the snapshot using a customer-supplied encryption key.
        /// After you encrypt a snapshot using a customer-supplied key, you must
        /// provide the same key if you use the snapshot later. For example, you
        /// must provide the encryption key when you create a disk from the
        /// encrypted snapshot in a future request.
        /// Customer-supplied encryption keys do not protect access to metadata of
        /// the snapshot.
        /// If you do not provide an encryption key when creating the snapshot,
        /// then the snapshot will be encrypted using an automatically generated
        /// key and you do not need to provide a key to use the snapshot later.
        /// Structure is documented below.
        #[builder(into, default)]
        pub snapshot_encryption_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SnapshotSnapshotEncryptionKey>,
        >,
        /// A reference to the disk used to create this snapshot.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub source_disk: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub source_disk_encryption_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SnapshotSourceDiskEncryptionKey>,
        >,
        /// Cloud Storage bucket storage location of the snapshot (regional or multi-regional).
        #[builder(into, default)]
        pub storage_locations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A reference to the zone where the disk is hosted.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Creates the new snapshot in the snapshot chain labeled with the
        /// specified name. The chain name must be 1-63 characters long and
        /// comply with RFC1035. This is an uncommon option only for advanced
        /// service owners who needs to create separate snapshot chains, for
        /// example, for chargeback tracking.  When you describe your snapshot
        /// resource, this field is visible only if it has a non-empty value.
        pub chain_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Size of the snapshot, specified in GB.
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The fingerprint used for optimistic locking of this resource. Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels to apply to this Snapshot.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of public visible licenses that apply to this snapshot. This
        /// can be because the original image had licenses attached (such as a
        /// Windows image).  snapshotEncryptionKey nested object Encrypts the
        /// snapshot using a customer-supplied encryption key.
        pub licenses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the resource; provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Encrypts the snapshot using a customer-supplied encryption key.
        /// After you encrypt a snapshot using a customer-supplied key, you must
        /// provide the same key if you use the snapshot later. For example, you
        /// must provide the encryption key when you create a disk from the
        /// encrypted snapshot in a future request.
        /// Customer-supplied encryption keys do not protect access to metadata of
        /// the snapshot.
        /// If you do not provide an encryption key when creating the snapshot,
        /// then the snapshot will be encrypted using an automatically generated
        /// key and you do not need to provide a key to use the snapshot later.
        /// Structure is documented below.
        pub snapshot_encryption_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SnapshotSnapshotEncryptionKey>,
        >,
        /// The unique identifier for the resource.
        pub snapshot_id: pulumi_gestalt_rust::Output<i32>,
        /// A reference to the disk used to create this snapshot.
        ///
        ///
        /// - - -
        pub source_disk: pulumi_gestalt_rust::Output<String>,
        /// The customer-supplied encryption key of the source snapshot. Required
        /// if the source snapshot is protected by a customer-supplied encryption
        /// key.
        /// Structure is documented below.
        pub source_disk_encryption_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SnapshotSourceDiskEncryptionKey>,
        >,
        /// A size of the storage used by the snapshot. As snapshots share
        /// storage, this number is expected to change with snapshot
        /// creation/deletion.
        pub storage_bytes: pulumi_gestalt_rust::Output<i32>,
        /// Cloud Storage bucket storage location of the snapshot (regional or multi-regional).
        pub storage_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A reference to the zone where the disk is hosted.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let chain_name_binding = args.chain_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let snapshot_encryption_key_binding = args
            .snapshot_encryption_key
            .get_output(context);
        let source_disk_binding = args.source_disk.get_output(context);
        let source_disk_encryption_key_binding = args
            .source_disk_encryption_key
            .get_output(context);
        let storage_locations_binding = args.storage_locations.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "chainName".into(),
                    value: &chain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotEncryptionKey".into(),
                    value: &snapshot_encryption_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceDisk".into(),
                    value: &source_disk_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceDiskEncryptionKey".into(),
                    value: &source_disk_encryption_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageLocations".into(),
                    value: &storage_locations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotResult {
            chain_name: o.get_field("chainName"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            disk_size_gb: o.get_field("diskSizeGb"),
            effective_labels: o.get_field("effectiveLabels"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            licenses: o.get_field("licenses"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            self_link: o.get_field("selfLink"),
            snapshot_encryption_key: o.get_field("snapshotEncryptionKey"),
            snapshot_id: o.get_field("snapshotId"),
            source_disk: o.get_field("sourceDisk"),
            source_disk_encryption_key: o.get_field("sourceDiskEncryptionKey"),
            storage_bytes: o.get_field("storageBytes"),
            storage_locations: o.get_field("storageLocations"),
            zone: o.get_field("zone"),
        }
    }
}
