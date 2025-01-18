/// A Managed Service for Apache Kafka cluster. Apache Kafka is a trademark owned by the Apache Software Foundation.
///
///
///
/// ## Example Usage
///
/// ### Managedkafka Cluster Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:managedkafka:Cluster
///     properties:
///       clusterId: my-cluster
///       location: us-central1
///       capacityConfig:
///         vcpuCount: 3
///         memoryBytes: 3.221225472e+09
///       gcpConfig:
///         accessConfig:
///           networkConfigs:
///             - subnet: projects/${project.number}/regions/us-central1/subnetworks/default
///       rebalanceConfig:
///         mode: NO_REBALANCE
///       labels:
///         key: value
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Managedkafka Cluster Cmek
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:managedkafka:Cluster
///     properties:
///       clusterId: my-cluster
///       location: us-central1
///       capacityConfig:
///         vcpuCount: 3
///         memoryBytes: 3.221225472e+09
///       gcpConfig:
///         accessConfig:
///           networkConfigs:
///             - subnet: projects/${project.number}/regions/us-central1/subnetworks/default
///         kmsKey: ${key.id}
///   kafkaServiceIdentity:
///     type: gcp:projects:ServiceIdentity
///     name: kafka_service_identity
///     properties:
///       project: ${project.projectId}
///       service: managedkafka.googleapis.com
///   key:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: example-key
///       keyRing: ${keyRing.id}
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: example-key-ring
///       location: us-central1
///   cryptoKeyBinding:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: crypto_key_binding
///     properties:
///       cryptoKeyId: ${key.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-managedkafka.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{cluster_id}}`
///
/// * `{{project}}/{{location}}/{{cluster_id}}`
///
/// * `{{location}}/{{cluster_id}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:managedkafka/cluster:Cluster default projects/{{project}}/locations/{{location}}/clusters/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:managedkafka/cluster:Cluster default {{project}}/{{location}}/{{cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:managedkafka/cluster:Cluster default {{location}}/{{cluster_id}}
/// ```
///
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// A capacity configuration of a Kafka cluster.
        /// Structure is documented below.
        #[builder(into)]
        pub capacity_config: pulumi_wasm_rust::Output<
            super::super::types::managedkafka::ClusterCapacityConfig,
        >,
        /// The ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression `a-z?` to comply with RFC 1035. This value is structured like: `my-cluster-id`.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Configuration properties for a Kafka cluster deployed to Google Cloud Platform.
        /// Structure is documented below.
        #[builder(into)]
        pub gcp_config: pulumi_wasm_rust::Output<
            super::super::types::managedkafka::ClusterGcpConfig,
        >,
        /// List of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-),
        /// underscores ( ), lowercase characters, and numbers. Values must contain only hyphens (-), underscores ( ), lowercase
        /// characters, and numbers. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines rebalancing behavior of a Kafka cluster.
        #[builder(into, default)]
        pub rebalance_config: pulumi_wasm_rust::Output<
            Option<super::super::types::managedkafka::ClusterRebalanceConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// A capacity configuration of a Kafka cluster.
        /// Structure is documented below.
        pub capacity_config: pulumi_wasm_rust::Output<
            super::super::types::managedkafka::ClusterCapacityConfig,
        >,
        /// The ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression `a-z?` to comply with RFC 1035. This value is structured like: `my-cluster-id`.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The time when the cluster was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration properties for a Kafka cluster deployed to Google Cloud Platform.
        /// Structure is documented below.
        pub gcp_config: pulumi_wasm_rust::Output<
            super::super::types::managedkafka::ClusterGcpConfig,
        >,
        /// List of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-),
        /// underscores ( ), lowercase characters, and numbers. Values must contain only hyphens (-), underscores ( ), lowercase
        /// characters, and numbers. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the cluster. Structured like: `projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID`.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines rebalancing behavior of a Kafka cluster.
        pub rebalance_config: pulumi_wasm_rust::Output<
            Option<super::super::types::managedkafka::ClusterRebalanceConfig>,
        >,
        /// The current state of the cluster. Possible values: `STATE_UNSPECIFIED`, `CREATING`, `ACTIVE`, `DELETING`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The time when the cluster was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_config_binding = args.capacity_config.get_inner();
        let cluster_id_binding = args.cluster_id.get_inner();
        let gcp_config_binding = args.gcp_config.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let rebalance_config_binding = args.rebalance_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:managedkafka/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityConfig".into(),
                    value: &capacity_config_binding,
                },
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "gcpConfig".into(),
                    value: &gcp_config_binding,
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
                    name: "rebalanceConfig".into(),
                    value: &rebalance_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "capacityConfig".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "gcpConfig".into(),
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
                    name: "rebalanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        ClusterResult {
            capacity_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityConfig").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            gcp_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gcpConfig").unwrap(),
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
            rebalance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rebalanceConfig").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
