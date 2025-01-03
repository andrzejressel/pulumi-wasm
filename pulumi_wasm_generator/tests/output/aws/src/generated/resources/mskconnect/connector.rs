/// Provides an Amazon MSK Connect Connector resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:mskconnect:Connector
///     properties:
///       name: example
///       kafkaconnectVersion: 2.7.1
///       capacity:
///         autoscaling:
///           mcuCount: 1
///           minWorkerCount: 1
///           maxWorkerCount: 2
///           scaleInPolicy:
///             cpuUtilizationPercentage: 20
///           scaleOutPolicy:
///             cpuUtilizationPercentage: 80
///       connectorConfiguration:
///         connector.class: com.github.jcustenborder.kafka.connect.simulator.SimulatorSinkConnector
///         tasks.max: '1'
///         topics: example
///       kafkaCluster:
///         apacheKafkaCluster:
///           bootstrapServers: ${exampleAwsMskCluster.bootstrapBrokersTls}
///           vpc:
///             securityGroups:
///               - ${exampleAwsSecurityGroup.id}
///             subnets:
///               - ${example1.id}
///               - ${example2.id}
///               - ${example3.id}
///       kafkaClusterClientAuthentication:
///         authenticationType: NONE
///       kafkaClusterEncryptionInTransit:
///         encryptionType: TLS
///       plugins:
///         - customPlugin:
///             arn: ${exampleAwsMskconnectCustomPlugin.arn}
///             revision: ${exampleAwsMskconnectCustomPlugin.latestRevision}
///       serviceExecutionRoleArn: ${exampleAwsIamRole.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK Connect Connector using the connector's `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:mskconnect/connector:Connector example 'arn:aws:kafkaconnect:eu-central-1:123456789012:connector/example/264edee4-17a3-412e-bd76-6681cfc93805-3'
/// ```
pub mod connector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectorArgs {
        /// Information about the capacity allocated to the connector. See `capacity` Block for details.
        #[builder(into)]
        pub capacity: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorCapacity,
        >,
        /// A map of keys to values that represent the configuration for the connector.
        #[builder(into)]
        pub connector_configuration: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A summary description of the connector.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies which Apache Kafka cluster to connect to. See `kafka_cluster` Block for details.
        #[builder(into)]
        pub kafka_cluster: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaCluster,
        >,
        /// Details of the client authentication used by the Apache Kafka cluster. See `kafka_cluster_client_authentication` Block for details.
        #[builder(into)]
        pub kafka_cluster_client_authentication: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaClusterClientAuthentication,
        >,
        /// Details of encryption in transit to the Apache Kafka cluster. See `kafka_cluster_encryption_in_transit` Block for details.
        #[builder(into)]
        pub kafka_cluster_encryption_in_transit: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaClusterEncryptionInTransit,
        >,
        /// The version of Kafka Connect. It has to be compatible with both the Apache Kafka cluster's version and the plugins.
        #[builder(into)]
        pub kafkaconnect_version: pulumi_wasm_rust::Output<String>,
        /// Details about log delivery. See `log_delivery` Block for details.
        #[builder(into, default)]
        pub log_delivery: pulumi_wasm_rust::Output<
            Option<super::super::types::mskconnect::ConnectorLogDelivery>,
        >,
        /// The name of the connector.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies which plugins to use for the connector. See `plugin` Block for details.
        #[builder(into)]
        pub plugins: pulumi_wasm_rust::Output<
            Vec<super::super::types::mskconnect::ConnectorPlugin>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role used by the connector to access the Amazon Web Services resources that it needs. The types of resources depends on the logic of the connector. For example, a connector that has Amazon S3 as a destination must have permissions that allow it to write to the S3 destination bucket.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub service_execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies which worker configuration to use with the connector. See `worker_configuration` Block for details.
        #[builder(into, default)]
        pub worker_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::mskconnect::ConnectorWorkerConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectorResult {
        /// The Amazon Resource Name (ARN) of the connector.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Information about the capacity allocated to the connector. See `capacity` Block for details.
        pub capacity: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorCapacity,
        >,
        /// A map of keys to values that represent the configuration for the connector.
        pub connector_configuration: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A summary description of the connector.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies which Apache Kafka cluster to connect to. See `kafka_cluster` Block for details.
        pub kafka_cluster: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaCluster,
        >,
        /// Details of the client authentication used by the Apache Kafka cluster. See `kafka_cluster_client_authentication` Block for details.
        pub kafka_cluster_client_authentication: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaClusterClientAuthentication,
        >,
        /// Details of encryption in transit to the Apache Kafka cluster. See `kafka_cluster_encryption_in_transit` Block for details.
        pub kafka_cluster_encryption_in_transit: pulumi_wasm_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaClusterEncryptionInTransit,
        >,
        /// The version of Kafka Connect. It has to be compatible with both the Apache Kafka cluster's version and the plugins.
        pub kafkaconnect_version: pulumi_wasm_rust::Output<String>,
        /// Details about log delivery. See `log_delivery` Block for details.
        pub log_delivery: pulumi_wasm_rust::Output<
            Option<super::super::types::mskconnect::ConnectorLogDelivery>,
        >,
        /// The name of the connector.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies which plugins to use for the connector. See `plugin` Block for details.
        pub plugins: pulumi_wasm_rust::Output<
            Vec<super::super::types::mskconnect::ConnectorPlugin>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role used by the connector to access the Amazon Web Services resources that it needs. The types of resources depends on the logic of the connector. For example, a connector that has Amazon S3 as a destination must have permissions that allow it to write to the S3 destination bucket.
        ///
        /// The following arguments are optional:
        pub service_execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current version of the connector.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Specifies which worker configuration to use with the connector. See `worker_configuration` Block for details.
        pub worker_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::mskconnect::ConnectorWorkerConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectorArgs) -> ConnectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_binding = args.capacity.get_inner();
        let connector_configuration_binding = args.connector_configuration.get_inner();
        let description_binding = args.description.get_inner();
        let kafka_cluster_binding = args.kafka_cluster.get_inner();
        let kafka_cluster_client_authentication_binding = args
            .kafka_cluster_client_authentication
            .get_inner();
        let kafka_cluster_encryption_in_transit_binding = args
            .kafka_cluster_encryption_in_transit
            .get_inner();
        let kafkaconnect_version_binding = args.kafkaconnect_version.get_inner();
        let log_delivery_binding = args.log_delivery.get_inner();
        let name_binding = args.name.get_inner();
        let plugins_binding = args.plugins.get_inner();
        let service_execution_role_arn_binding = args
            .service_execution_role_arn
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let worker_configuration_binding = args.worker_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mskconnect/connector:Connector".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "connectorConfiguration".into(),
                    value: &connector_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaCluster".into(),
                    value: &kafka_cluster_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaClusterClientAuthentication".into(),
                    value: &kafka_cluster_client_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaClusterEncryptionInTransit".into(),
                    value: &kafka_cluster_encryption_in_transit_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaconnectVersion".into(),
                    value: &kafkaconnect_version_binding,
                },
                register_interface::ObjectField {
                    name: "logDelivery".into(),
                    value: &log_delivery_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "plugins".into(),
                    value: &plugins_binding,
                },
                register_interface::ObjectField {
                    name: "serviceExecutionRoleArn".into(),
                    value: &service_execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workerConfiguration".into(),
                    value: &worker_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "connectorConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "kafkaCluster".into(),
                },
                register_interface::ResultField {
                    name: "kafkaClusterClientAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "kafkaClusterEncryptionInTransit".into(),
                },
                register_interface::ResultField {
                    name: "kafkaconnectVersion".into(),
                },
                register_interface::ResultField {
                    name: "logDelivery".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "plugins".into(),
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
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "workerConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectorResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            connector_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorConfiguration").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            kafka_cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaCluster").unwrap(),
            ),
            kafka_cluster_client_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaClusterClientAuthentication").unwrap(),
            ),
            kafka_cluster_encryption_in_transit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaClusterEncryptionInTransit").unwrap(),
            ),
            kafkaconnect_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaconnectVersion").unwrap(),
            ),
            log_delivery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDelivery").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plugins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plugins").unwrap(),
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
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            worker_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerConfiguration").unwrap(),
            ),
        }
    }
}
