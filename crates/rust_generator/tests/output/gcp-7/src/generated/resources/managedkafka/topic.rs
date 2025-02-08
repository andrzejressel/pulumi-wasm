/// A Managed Service for Apache Kafka topic. Apache Kafka is a trademark owned by the Apache Software Foundation.
///
///
///
/// ## Example Usage
///
/// ### Managedkafka Topic Basic
///
///
/// ```yaml
/// resources:
///   cluster:
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
///   example:
///     type: gcp:managedkafka:Topic
///     properties:
///       topicId: my-topic
///       cluster: ${cluster.clusterId}
///       location: us-central1
///       partitionCount: 2
///       replicationFactor: 3
///       configs:
///         cleanup.policy: compact
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Topic can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/topics/{{topic_id}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{topic_id}}`
///
/// * `{{location}}/{{cluster}}/{{topic_id}}`
///
/// When using the `pulumi import` command, Topic can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:managedkafka/topic:Topic default projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/topics/{{topic_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:managedkafka/topic:Topic default {{project}}/{{location}}/{{cluster}}/{{topic_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:managedkafka/topic:Topic default {{location}}/{{cluster}}/{{topic_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicArgs {
        /// The cluster name.
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for the topic that are overridden from the cluster defaults. The key of the map is a Kafka topic property name, for example: `cleanup.policy=compact`, `compression.type=producer`.
        #[builder(into, default)]
        pub configs: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of partitions in a topic. You can increase the partition count for a topic, but you cannot decrease it. Increasing partitions for a topic that uses a key might change how messages are distributed.
        #[builder(into, default)]
        pub partition_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of replicas of each partition. A replication factor of 3 is recommended for high availability.
        #[builder(into)]
        pub replication_factor: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID to use for the topic, which will become the final component of the topic's name. This value is structured like: `my-topic-name`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub topic_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TopicResult {
        /// The cluster name.
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// Configuration for the topic that are overridden from the cluster defaults. The key of the map is a Kafka topic property name, for example: `cleanup.policy=compact`, `compression.type=producer`.
        pub configs: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the topic. The `topic` segment is used when connecting directly to the cluster. Must be in the format `projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID/topics/TOPIC_ID`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of partitions in a topic. You can increase the partition count for a topic, but you cannot decrease it. Increasing partitions for a topic that uses a key might change how messages are distributed.
        pub partition_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The number of replicas of each partition. A replication factor of 3 is recommended for high availability.
        pub replication_factor: pulumi_gestalt_rust::Output<i32>,
        /// The ID to use for the topic, which will become the final component of the topic's name. This value is structured like: `my-topic-name`.
        ///
        ///
        /// - - -
        pub topic_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TopicArgs,
    ) -> TopicResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let configs_binding = args.configs.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let partition_count_binding = args
            .partition_count
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let replication_factor_binding = args
            .replication_factor
            .get_output(context)
            .get_inner();
        let topic_id_binding = args.topic_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:managedkafka/topic:Topic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "configs".into(),
                    value: &configs_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "partitionCount".into(),
                    value: &partition_count_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "replicationFactor".into(),
                    value: &replication_factor_binding,
                },
                register_interface::ObjectField {
                    name: "topicId".into(),
                    value: &topic_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TopicResult {
            cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configs"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            partition_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionCount"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            replication_factor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationFactor"),
            ),
            topic_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("topicId"),
            ),
        }
    }
}
