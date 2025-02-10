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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectorArgs {
        /// Information about the capacity allocated to the connector. See `capacity` Block for details.
        #[builder(into)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mskconnect::ConnectorCapacity,
        >,
        /// A map of keys to values that represent the configuration for the connector.
        #[builder(into)]
        pub connector_configuration: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// A summary description of the connector.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies which Apache Kafka cluster to connect to. See `kafka_cluster` Block for details.
        #[builder(into)]
        pub kafka_cluster: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mskconnect::ConnectorKafkaCluster,
        >,
        /// Details of the client authentication used by the Apache Kafka cluster. See `kafka_cluster_client_authentication` Block for details.
        #[builder(into)]
        pub kafka_cluster_client_authentication: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mskconnect::ConnectorKafkaClusterClientAuthentication,
        >,
        /// Details of encryption in transit to the Apache Kafka cluster. See `kafka_cluster_encryption_in_transit` Block for details.
        #[builder(into)]
        pub kafka_cluster_encryption_in_transit: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mskconnect::ConnectorKafkaClusterEncryptionInTransit,
        >,
        /// The version of Kafka Connect. It has to be compatible with both the Apache Kafka cluster's version and the plugins.
        #[builder(into)]
        pub kafkaconnect_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Details about log delivery. See `log_delivery` Block for details.
        #[builder(into, default)]
        pub log_delivery: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mskconnect::ConnectorLogDelivery>,
        >,
        /// The name of the connector.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies which plugins to use for the connector. See `plugin` Block for details.
        #[builder(into)]
        pub plugins: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::mskconnect::ConnectorPlugin>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role used by the connector to access the Amazon Web Services resources that it needs. The types of resources depends on the logic of the connector. For example, a connector that has Amazon S3 as a destination must have permissions that allow it to write to the S3 destination bucket.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub service_execution_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies which worker configuration to use with the connector. See `worker_configuration` Block for details.
        #[builder(into, default)]
        pub worker_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mskconnect::ConnectorWorkerConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectorResult {
        /// The Amazon Resource Name (ARN) of the connector.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Information about the capacity allocated to the connector. See `capacity` Block for details.
        pub capacity: pulumi_gestalt_rust::Output<
            super::super::types::mskconnect::ConnectorCapacity,
        >,
        /// A map of keys to values that represent the configuration for the connector.
        pub connector_configuration: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A summary description of the connector.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies which Apache Kafka cluster to connect to. See `kafka_cluster` Block for details.
        pub kafka_cluster: pulumi_gestalt_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaCluster,
        >,
        /// Details of the client authentication used by the Apache Kafka cluster. See `kafka_cluster_client_authentication` Block for details.
        pub kafka_cluster_client_authentication: pulumi_gestalt_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaClusterClientAuthentication,
        >,
        /// Details of encryption in transit to the Apache Kafka cluster. See `kafka_cluster_encryption_in_transit` Block for details.
        pub kafka_cluster_encryption_in_transit: pulumi_gestalt_rust::Output<
            super::super::types::mskconnect::ConnectorKafkaClusterEncryptionInTransit,
        >,
        /// The version of Kafka Connect. It has to be compatible with both the Apache Kafka cluster's version and the plugins.
        pub kafkaconnect_version: pulumi_gestalt_rust::Output<String>,
        /// Details about log delivery. See `log_delivery` Block for details.
        pub log_delivery: pulumi_gestalt_rust::Output<
            Option<super::super::types::mskconnect::ConnectorLogDelivery>,
        >,
        /// The name of the connector.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies which plugins to use for the connector. See `plugin` Block for details.
        pub plugins: pulumi_gestalt_rust::Output<
            Vec<super::super::types::mskconnect::ConnectorPlugin>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role used by the connector to access the Amazon Web Services resources that it needs. The types of resources depends on the logic of the connector. For example, a connector that has Amazon S3 as a destination must have permissions that allow it to write to the S3 destination bucket.
        ///
        /// The following arguments are optional:
        pub service_execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current version of the connector.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Specifies which worker configuration to use with the connector. See `worker_configuration` Block for details.
        pub worker_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::mskconnect::ConnectorWorkerConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectorArgs,
    ) -> ConnectorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_binding = args.capacity.get_output(context);
        let connector_configuration_binding = args
            .connector_configuration
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let kafka_cluster_binding = args.kafka_cluster.get_output(context);
        let kafka_cluster_client_authentication_binding = args
            .kafka_cluster_client_authentication
            .get_output(context);
        let kafka_cluster_encryption_in_transit_binding = args
            .kafka_cluster_encryption_in_transit
            .get_output(context);
        let kafkaconnect_version_binding = args.kafkaconnect_version.get_output(context);
        let log_delivery_binding = args.log_delivery.get_output(context);
        let name_binding = args.name.get_output(context);
        let plugins_binding = args.plugins.get_output(context);
        let service_execution_role_arn_binding = args
            .service_execution_role_arn
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let worker_configuration_binding = args.worker_configuration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:mskconnect/connector:Connector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectorConfiguration".into(),
                    value: connector_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kafkaCluster".into(),
                    value: kafka_cluster_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kafkaClusterClientAuthentication".into(),
                    value: kafka_cluster_client_authentication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kafkaClusterEncryptionInTransit".into(),
                    value: kafka_cluster_encryption_in_transit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kafkaconnectVersion".into(),
                    value: kafkaconnect_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logDelivery".into(),
                    value: log_delivery_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plugins".into(),
                    value: plugins_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceExecutionRoleArn".into(),
                    value: service_execution_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workerConfiguration".into(),
                    value: worker_configuration_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectorResult {
            arn: o.get_field("arn"),
            capacity: o.get_field("capacity"),
            connector_configuration: o.get_field("connectorConfiguration"),
            description: o.get_field("description"),
            kafka_cluster: o.get_field("kafkaCluster"),
            kafka_cluster_client_authentication: o
                .get_field("kafkaClusterClientAuthentication"),
            kafka_cluster_encryption_in_transit: o
                .get_field("kafkaClusterEncryptionInTransit"),
            kafkaconnect_version: o.get_field("kafkaconnectVersion"),
            log_delivery: o.get_field("logDelivery"),
            name: o.get_field("name"),
            plugins: o.get_field("plugins"),
            service_execution_role_arn: o.get_field("serviceExecutionRoleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
            worker_configuration: o.get_field("workerConfiguration"),
        }
    }
}
