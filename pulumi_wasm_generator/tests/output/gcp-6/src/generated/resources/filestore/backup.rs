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
pub mod backup {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupArgs {
        /// A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the backup. The name must be unique within the specified instance.
        /// The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the file share in the source Cloud Filestore instance that the backup is created from.
        #[builder(into)]
        pub source_file_share: pulumi_wasm_rust::Output<String>,
        /// The resource name of the source Cloud Filestore instance, in the format projects/{projectId}/locations/{locationId}/instances/{instanceId}, used to create this backup.
        #[builder(into)]
        pub source_instance: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BackupResult {
        /// The amount of bytes needed to allocate a full copy of the snapshot content.
        pub capacity_gb: pulumi_wasm_rust::Output<String>,
        /// The time when the snapshot was created in RFC3339 text format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A description of the backup with 2048 characters or less. Requests with longer descriptions will be rejected.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of bytes that will be downloaded if the backup is restored.
        pub download_bytes: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// KMS key name used for data encryption.
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the backup. The name must be unique within the specified instance.
        /// The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the file share in the source Cloud Filestore instance that the backup is created from.
        pub source_file_share: pulumi_wasm_rust::Output<String>,
        /// The resource name of the source Cloud Filestore instance, in the format projects/{projectId}/locations/{locationId}/instances/{instanceId}, used to create this backup.
        pub source_instance: pulumi_wasm_rust::Output<String>,
        /// The service tier of the source Cloud Filestore instance that this backup is created from.
        pub source_instance_tier: pulumi_wasm_rust::Output<String>,
        /// The backup state.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The size of the storage used by the backup. As backups share storage, this number is expected to change with backup creation/deletion.
        pub storage_bytes: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupArgs) -> BackupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let source_file_share_binding = args.source_file_share.get_inner();
        let source_instance_binding = args.source_instance.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:filestore/backup:Backup".into(),
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
                    name: "sourceFileShare".into(),
                    value: &source_file_share_binding,
                },
                register_interface::ObjectField {
                    name: "sourceInstance".into(),
                    value: &source_instance_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "capacityGb".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "downloadBytes".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "sourceFileShare".into(),
                },
                register_interface::ResultField {
                    name: "sourceInstance".into(),
                },
                register_interface::ResultField {
                    name: "sourceInstanceTier".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "storageBytes".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupResult {
            capacity_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityGb").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            download_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("downloadBytes").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            source_file_share: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceFileShare").unwrap(),
            ),
            source_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceInstance").unwrap(),
            ),
            source_instance_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceInstanceTier").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            storage_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageBytes").unwrap(),
            ),
        }
    }
}
