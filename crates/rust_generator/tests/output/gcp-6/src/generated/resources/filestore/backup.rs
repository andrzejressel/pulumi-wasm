/// A Google Cloud Filestore backup.
///
///
/// To get more information about Backup, see:
///
/// * [API documentation](https://cloud.google.com/filestore/docs/reference/rest/v1/projects.locations.instances.backups)
/// * How-to Guides
///     * [Creating Backups](https://cloud.google.com/filestore/docs/create-backups)
///     * [Official Documentation](https://cloud.google.com/filestore/docs/backups)
///
/// ## Example Usage
///
/// ### Filestore Backup Basic
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:filestore:Instance
///     properties:
///       name: tf-fs-inst
///       location: us-central1-b
///       tier: BASIC_HDD
///       fileShares:
///         capacityGb: 1024
///         name: share1
///       networks:
///         - network: default
///           modes:
///             - MODE_IPV4
///           connectMode: DIRECT_PEERING
///   backup:
///     type: gcp:filestore:Backup
///     properties:
///       name: tf-fs-bkup
///       location: us-central1
///       description: This is a filestore backup for the test instance
///       sourceInstance: ${instance.id}
///       sourceFileShare: share1
///       labels:
///         files: label1
///         other-label: label2
/// ```
///
/// ## Import
///
/// Backup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backups/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Backup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:filestore/backup:Backup default projects/{{project}}/locations/{{location}}/backups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:filestore/backup:Backup default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:filestore/backup:Backup default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupArgs {
        /// A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the backup. The name must be unique within the specified instance.
        /// The name must be 1-63 characters long, and comply with
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
        /// Name of the file share in the source Cloud Filestore instance that the backup is created from.
        #[builder(into)]
        pub source_file_share: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the source Cloud Filestore instance, in the format projects/{projectId}/locations/{locationId}/instances/{instanceId}, used to create this backup.
        #[builder(into)]
        pub source_instance: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupResult {
        /// The amount of bytes needed to allocate a full copy of the snapshot content.
        pub capacity_gb: pulumi_gestalt_rust::Output<String>,
        /// The time when the snapshot was created in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amount of bytes that will be downloaded if the backup is restored.
        pub download_bytes: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// KMS key name used for data encryption.
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the backup. The name must be unique within the specified instance.
        /// The name must be 1-63 characters long, and comply with
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
        /// Name of the file share in the source Cloud Filestore instance that the backup is created from.
        pub source_file_share: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the source Cloud Filestore instance, in the format projects/{projectId}/locations/{locationId}/instances/{instanceId}, used to create this backup.
        pub source_instance: pulumi_gestalt_rust::Output<String>,
        /// The service tier of the source Cloud Filestore instance that this backup is created from.
        pub source_instance_tier: pulumi_gestalt_rust::Output<String>,
        /// The backup state.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion.
        pub storage_bytes: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupArgs,
    ) -> BackupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let source_file_share_binding = args.source_file_share.get_output(context);
        let source_instance_binding = args.source_instance.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:filestore/backup:Backup".into(),
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
                    name: "sourceFileShare".into(),
                    value: source_file_share_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceInstance".into(),
                    value: source_instance_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupResult {
            capacity_gb: o.get_field("capacityGb"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            download_bytes: o.get_field("downloadBytes"),
            effective_labels: o.get_field("effectiveLabels"),
            kms_key_name: o.get_field("kmsKeyName"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            source_file_share: o.get_field("sourceFileShare"),
            source_instance: o.get_field("sourceInstance"),
            source_instance_tier: o.get_field("sourceInstanceTier"),
            state: o.get_field("state"),
            storage_bytes: o.get_field("storageBytes"),
        }
    }
}
