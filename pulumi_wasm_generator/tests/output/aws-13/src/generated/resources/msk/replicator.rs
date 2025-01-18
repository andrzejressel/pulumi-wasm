/// Resource for managing an AWS Managed Streaming for Kafka Replicator.
///
/// ## Example Usage
///
/// ### Basic Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import MSK replicators using the replicator ARN. For example:
///
/// ```sh
/// $ pulumi import aws:msk/replicator:Replicator example arn:aws:kafka:us-west-2:123456789012:configuration/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
pub mod replicator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicatorArgs {
        /// A summary description of the replicator.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Kafka clusters which are targets of the replicator.
        #[builder(into)]
        pub kafka_clusters: pulumi_wasm_rust::Output<
            Vec<super::super::types::msk::ReplicatorKafkaCluster>,
        >,
        /// A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.
        #[builder(into)]
        pub replication_info_list: pulumi_wasm_rust::Output<
            super::super::types::msk::ReplicatorReplicationInfoList,
        >,
        /// The name of the replicator.
        #[builder(into)]
        pub replicator_name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters).
        #[builder(into)]
        pub service_execution_role_arn: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicatorResult {
        /// ARN of the Replicator. Do not begin the description with "An", "The", "Defines", "Indicates", or "Specifies," as these are verbose. In other words, "Indicates the amount of storage," can be rewritten as "Amount of storage," without losing any information.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub current_version: pulumi_wasm_rust::Output<String>,
        /// A summary description of the replicator.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Kafka clusters which are targets of the replicator.
        pub kafka_clusters: pulumi_wasm_rust::Output<
            Vec<super::super::types::msk::ReplicatorKafkaCluster>,
        >,
        /// A list of replication configurations, where each configuration targets a given source cluster to target cluster replication flow.
        pub replication_info_list: pulumi_wasm_rust::Output<
            super::super::types::msk::ReplicatorReplicationInfoList,
        >,
        /// The name of the replicator.
        pub replicator_name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM role used by the replicator to access resources in the customer's account (e.g source and target clusters).
        pub service_execution_role_arn: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ReplicatorArgs) -> ReplicatorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let kafka_clusters_binding = args.kafka_clusters.get_inner();
        let replication_info_list_binding = args.replication_info_list.get_inner();
        let replicator_name_binding = args.replicator_name.get_inner();
        let service_execution_role_arn_binding = args
            .service_execution_role_arn
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:msk/replicator:Replicator".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaClusters".into(),
                    value: &kafka_clusters_binding,
                },
                register_interface::ObjectField {
                    name: "replicationInfoList".into(),
                    value: &replication_info_list_binding,
                },
                register_interface::ObjectField {
                    name: "replicatorName".into(),
                    value: &replicator_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceExecutionRoleArn".into(),
                    value: &service_execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "currentVersion".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "kafkaClusters".into(),
                },
                register_interface::ResultField {
                    name: "replicationInfoList".into(),
                },
                register_interface::ResultField {
                    name: "replicatorName".into(),
                },
                register_interface::ResultField {
                    name: "serviceExecutionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicatorResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            current_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentVersion").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            kafka_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaClusters").unwrap(),
            ),
            replication_info_list: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInfoList").unwrap(),
            ),
            replicator_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicatorName").unwrap(),
            ),
            service_execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceExecutionRoleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
