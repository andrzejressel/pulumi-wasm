pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the cluster.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of key-value pairs assigned to the cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// ARN of the MSK cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Comma separated list of one or more hostname:port pairs of kafka brokers suitable to bootstrap connectivity to the kafka cluster. Contains a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `PLAINTEXT` or `TLS_PLAINTEXT`. The resource sorts values alphabetically. AWS may not always return all endpoints so this value is not guaranteed to be stable across applies.
        pub bootstrap_brokers: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_public_sasl_iam: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_public_sasl_scram: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_public_tls: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_sasl_iam: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_sasl_scram: pulumi_gestalt_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_tls: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the broker nodes of the Kafka cluster.
        pub broker_node_group_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::msk::GetClusterBrokerNodeGroupInfo>,
        >,
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// UUID of the MSK cluster, for use in IAM policies.
        pub cluster_uuid: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Apache Kafka version.
        pub kafka_version: pulumi_gestalt_rust::Output<String>,
        /// Number of broker nodes in the cluster.
        pub number_of_broker_nodes: pulumi_gestalt_rust::Output<i32>,
        /// Map of key-value pairs assigned to the cluster.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster. The returned values are sorted alphbetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
        pub zookeeper_connect_string: pulumi_gestalt_rust::Output<String>,
        /// A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster via TLS. The returned values are sorted alphabetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
        pub zookeeper_connect_string_tls: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:msk/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            bootstrap_brokers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootstrapBrokers"),
            ),
            bootstrap_brokers_public_sasl_iam: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootstrapBrokersPublicSaslIam"),
            ),
            bootstrap_brokers_public_sasl_scram: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootstrapBrokersPublicSaslScram"),
            ),
            bootstrap_brokers_public_tls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootstrapBrokersPublicTls"),
            ),
            bootstrap_brokers_sasl_iam: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootstrapBrokersSaslIam"),
            ),
            bootstrap_brokers_sasl_scram: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootstrapBrokersSaslScram"),
            ),
            bootstrap_brokers_tls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootstrapBrokersTls"),
            ),
            broker_node_group_infos: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("brokerNodeGroupInfos"),
            ),
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            cluster_uuid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterUuid"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kafka_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kafkaVersion"),
            ),
            number_of_broker_nodes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numberOfBrokerNodes"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zookeeper_connect_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zookeeperConnectString"),
            ),
            zookeeper_connect_string_tls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zookeeperConnectStringTls"),
            ),
        }
    }
}
