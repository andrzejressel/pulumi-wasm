/// An AlloyDB Backup.
///
///
/// To get more information about Backup, see:
///
/// * [API documentation](https://cloud.google.com/alloydb/docs/reference/rest/v1/projects.locations.backups/create)
/// * How-to Guides
///     * [AlloyDB](https://cloud.google.com/alloydb/docs/)
///
/// ## Example Usage
///
/// ### Alloydb Backup Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backup::create(
///         "default",
///         BackupArgs::builder()
///             .backup_id("alloydb-backup")
///             .cluster_name("${defaultCluster.name}")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let defaultCluster = cluster::create(
///         "defaultCluster",
///         ClusterArgs::builder()
///             .cluster_id("alloydb-cluster")
///             .location("us-central1")
///             .network_config(
///                 ClusterNetworkConfig::builder()
///                     .network("${defaultNetwork.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let defaultInstance = instance::create(
///         "defaultInstance",
///         InstanceArgs::builder()
///             .cluster("${defaultCluster.name}")
///             .instance_id("alloydb-instance")
///             .instance_type("PRIMARY")
///             .build_struct(),
///     );
///     let defaultNetwork = network::create(
///         "defaultNetwork",
///         NetworkArgs::builder().name("alloydb-network").build_struct(),
///     );
///     let privateIpAlloc = global_address::create(
///         "privateIpAlloc",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("alloydb-cluster")
///             .network("${defaultNetwork.id}")
///             .prefix_length(16)
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
///     let vpcConnection = connection::create(
///         "vpcConnection",
///         ConnectionArgs::builder()
///             .network("${defaultNetwork.id}")
///             .reserved_peering_ranges(vec!["${privateIpAlloc.name}",])
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Alloydb Backup Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:alloydb:Backup
///     properties:
///       location: us-central1
///       backupId: alloydb-backup
///       clusterName: ${defaultCluster.name}
///       description: example description
///       type: ON_DEMAND
///       labels:
///         label: key
///     options:
///       dependsOn:
///         - ${defaultInstance}
///   defaultCluster:
///     type: gcp:alloydb:Cluster
///     name: default
///     properties:
///       clusterId: alloydb-cluster
///       location: us-central1
///       networkConfig:
///         network: ${defaultNetwork.id}
///   defaultInstance:
///     type: gcp:alloydb:Instance
///     name: default
///     properties:
///       cluster: ${defaultCluster.name}
///       instanceId: alloydb-instance
///       instanceType: PRIMARY
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${defaultNetwork.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${defaultNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: alloydb-network
/// ```
///
/// ## Import
///
/// Backup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backups/{{backup_id}}`
///
/// * `{{project}}/{{location}}/{{backup_id}}`
///
/// * `{{location}}/{{backup_id}}`
///
/// When using the `pulumi import` command, Backup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:alloydb/backup:Backup default projects/{{project}}/locations/{{location}}/backups/{{backup_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/backup:Backup default {{project}}/{{location}}/{{backup_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/backup:Backup default {{location}}/{{backup_id}}
/// ```
///
pub mod backup {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupArgs {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the alloydb backup.
        #[builder(into)]
        pub backup_id: pulumi_wasm_rust::Output<String>,
        /// The full resource name of the backup source cluster (e.g., projects/{project}/locations/{location}/clusters/{clusterId}).
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// User-provided description of the backup.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User-settable and human-readable display name for the Backup.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::BackupEncryptionConfig>,
        >,
        /// User-defined labels for the alloydb backup. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb backup should reside.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The backup type, which suggests the trigger for the backup.
        /// Possible values are: `TYPE_UNSPECIFIED`, `ON_DEMAND`, `AUTOMATED`, `CONTINUOUS`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackupResult {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels. https://google.aip.dev/128
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the alloydb backup.
        pub backup_id: pulumi_wasm_rust::Output<String>,
        /// The full resource name of the backup source cluster (e.g., projects/{project}/locations/{location}/clusters/{clusterId}).
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Output only. The system-generated UID of the cluster which was used to create this resource.
        pub cluster_uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Create time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. Delete time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// User-provided description of the backup.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User-settable and human-readable display name for the Backup.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// EncryptionConfig describes the encryption config of a cluster or a backup that is encrypted with a CMEK (customer-managed encryption key).
        /// Structure is documented below.
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::alloydb::BackupEncryptionConfig>,
        >,
        /// EncryptionInfo describes the encryption information of a cluster or a backup.
        /// Structure is documented below.
        pub encryption_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::alloydb::BackupEncryptionInfo>,
        >,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Output only. The QuantityBasedExpiry of the backup, specified by the backup's retention policy.
        /// Once the expiry quantity is over retention, the backup is eligible to be garbage collected.
        /// Structure is documented below.
        pub expiry_quantities: pulumi_wasm_rust::Output<
            Vec<super::super::types::alloydb::BackupExpiryQuantity>,
        >,
        /// Output only. The time at which after the backup is eligible to be garbage collected.
        /// It is the duration specified by the backup's retention policy, added to the backup's createTime.
        pub expiry_time: pulumi_wasm_rust::Output<String>,
        /// User-defined labels for the alloydb backup. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the alloydb backup should reside.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. The name of the backup resource with the format: * projects/{project}/locations/{region}/backups/{backupId}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Reconciling (https://google.aip.dev/128#reconciliation), if true, indicates that the service is actively updating the resource.
        /// This can happen due to user-triggered updates or system actions like failover or maintenance.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Output only. The size of the backup in bytes.
        pub size_bytes: pulumi_wasm_rust::Output<String>,
        /// Output only. The current state of the backup.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The backup type, which suggests the trigger for the backup.
        /// Possible values are: `TYPE_UNSPECIFIED`, `ON_DEMAND`, `AUTOMATED`, `CONTINUOUS`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Output only. The system-generated UID of the resource. The UID is assigned when the resource is created, and it is retained until it is deleted.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Update time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupArgs) -> BackupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let backup_id_binding = args.backup_id.get_inner();
        let cluster_name_binding = args.cluster_name.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let encryption_config_binding = args.encryption_config.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:alloydb/backup:Backup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "backupId".into(),
                    value: &backup_id_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfig".into(),
                    value: &encryption_config_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "backupId".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "clusterUid".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfig".into(),
                },
                register_interface::ResultField {
                    name: "encryptionInfos".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "expiryQuantities".into(),
                },
                register_interface::ResultField {
                    name: "expiryTime".into(),
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
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "sizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
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
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            backup_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupId").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            cluster_uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterUid").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            encryption_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfig").unwrap(),
            ),
            encryption_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionInfos").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            expiry_quantities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiryQuantities").unwrap(),
            ),
            expiry_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiryTime").unwrap(),
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
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sizeBytes").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
