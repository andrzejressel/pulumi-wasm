/// Manages an Amazon MSK cluster.
///
/// > **Note:** This resource manages _provisioned_ clusters. To manage a _serverless_ Amazon MSK cluster, use the `aws.msk.ServerlessCluster` resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   vpc:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 192.168.0.0/22
///   subnetAz1:
///     type: aws:ec2:Subnet
///     name: subnet_az1
///     properties:
///       availabilityZone: ${azs.names[0]}
///       cidrBlock: 192.168.0.0/24
///       vpcId: ${vpc.id}
///   subnetAz2:
///     type: aws:ec2:Subnet
///     name: subnet_az2
///     properties:
///       availabilityZone: ${azs.names[1]}
///       cidrBlock: 192.168.1.0/24
///       vpcId: ${vpc.id}
///   subnetAz3:
///     type: aws:ec2:Subnet
///     name: subnet_az3
///     properties:
///       availabilityZone: ${azs.names[2]}
///       cidrBlock: 192.168.2.0/24
///       vpcId: ${vpc.id}
///   sg:
///     type: aws:ec2:SecurityGroup
///     properties:
///       vpcId: ${vpc.id}
///   kms:
///     type: aws:kms:Key
///     properties:
///       description: example
///   test:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: msk_broker_logs
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: msk-broker-logs-bucket
///   bucketAcl:
///     type: aws:s3:BucketAclV2
///     name: bucket_acl
///     properties:
///       bucket: ${bucket.id}
///       acl: private
///   firehoseRole:
///     type: aws:iam:Role
///     name: firehose_role
///     properties:
///       name: firehose_test_role
///       assumeRolePolicy: ${assumeRole.json}
///   testStream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: test_stream
///     properties:
///       name: kinesis-firehose-msk-broker-logs-stream
///       destination: extended_s3
///       extendedS3Configuration:
///         roleArn: ${firehoseRole.arn}
///         bucketArn: ${bucket.arn}
///       tags:
///         LogDeliveryEnabled: placeholder
///   example:
///     type: aws:msk:Cluster
///     properties:
///       clusterName: example
///       kafkaVersion: 3.2.0
///       numberOfBrokerNodes: 3
///       brokerNodeGroupInfo:
///         instanceType: kafka.m5.large
///         clientSubnets:
///           - ${subnetAz1.id}
///           - ${subnetAz2.id}
///           - ${subnetAz3.id}
///         storageInfo:
///           ebsStorageInfo:
///             volumeSize: 1000
///         securityGroups:
///           - ${sg.id}
///       encryptionInfo:
///         encryptionAtRestKmsKeyArn: ${kms.arn}
///       openMonitoring:
///         prometheus:
///           jmxExporter:
///             enabledInBroker: true
///           nodeExporter:
///             enabledInBroker: true
///       loggingInfo:
///         brokerLogs:
///           cloudwatchLogs:
///             enabled: true
///             logGroup: ${test.name}
///           firehose:
///             enabled: true
///             deliveryStream: ${testStream.name}
///           s3:
///             enabled: true
///             bucket: ${bucket.id}
///             prefix: logs/msk-
///       tags:
///         foo: bar
/// variables:
///   azs:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - firehose.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// outputs:
///   zookeeperConnectString: ${example.zookeeperConnectString}
///   bootstrapBrokersTls: ${example.bootstrapBrokersTls}
/// ```
///
/// ### With volume_throughput argument
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .broker_node_group_info(
///                 ClusterBrokerNodeGroupInfo::builder()
///                     .clientSubnets(
///                         vec!["${subnetAz1.id}", "${subnetAz2.id}", "${subnetAz3.id}",],
///                     )
///                     .instanceType("kafka.m5.4xlarge")
///                     .securityGroups(vec!["${sg.id}",])
///                     .storageInfo(
///                         ClusterBrokerNodeGroupInfoStorageInfo::builder()
///                             .ebsStorageInfo(
///                                 ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo::builder()
///                                     .provisionedThroughput(
///                                         ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput::builder()
///                                             .enabled(true)
///                                             .volumeThroughput(250)
///                                             .build_struct(),
///                                     )
///                                     .volumeSize(1000)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .cluster_name("example")
///             .kafka_version("2.7.1")
///             .number_of_broker_nodes(3)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK clusters using the cluster `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:msk/cluster:Cluster example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// Configuration block for the broker nodes of the Kafka cluster.
        #[builder(into)]
        pub broker_node_group_info: pulumi_wasm_rust::InputOrOutput<
            super::super::types::msk::ClusterBrokerNodeGroupInfo,
        >,
        /// Configuration block for specifying a client authentication. See below.
        #[builder(into, default)]
        pub client_authentication: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::msk::ClusterClientAuthentication>,
        >,
        /// Name of the MSK cluster.
        #[builder(into, default)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block for specifying a MSK Configuration to attach to Kafka brokers. See below.
        #[builder(into, default)]
        pub configuration_info: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::msk::ClusterConfigurationInfo>,
        >,
        /// Configuration block for specifying encryption. See below.
        #[builder(into, default)]
        pub encryption_info: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::msk::ClusterEncryptionInfo>,
        >,
        /// Specify the desired enhanced MSK CloudWatch monitoring level. See [Monitoring Amazon MSK with Amazon CloudWatch](https://docs.aws.amazon.com/msk/latest/developerguide/monitoring.html)
        #[builder(into, default)]
        pub enhanced_monitoring: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specify the desired Kafka software version.
        #[builder(into)]
        pub kafka_version: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configuration block for streaming broker logs to Cloudwatch/S3/Kinesis Firehose. See below.
        #[builder(into, default)]
        pub logging_info: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::msk::ClusterLoggingInfo>,
        >,
        /// The desired total number of broker nodes in the kafka cluster.  It must be a multiple of the number of specified client subnets.
        #[builder(into)]
        pub number_of_broker_nodes: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Configuration block for JMX and Node monitoring for the MSK cluster. See below.
        #[builder(into, default)]
        pub open_monitoring: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::msk::ClusterOpenMonitoring>,
        >,
        /// Controls storage mode for supported storage tiers. Valid values are: `LOCAL` or `TIERED`.
        #[builder(into, default)]
        pub storage_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// Amazon Resource Name (ARN) of the MSK cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Comma separated list of one or more hostname:port pairs of kafka brokers suitable to bootstrap connectivity to the kafka cluster. Contains a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `PLAINTEXT` or `TLS_PLAINTEXT`. The resource sorts values alphabetically. AWS may not always return all endpoints so this value is not guaranteed to be stable across applies.
        pub bootstrap_brokers: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9198`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_public_sasl_iam: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9196`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_public_sasl_scram: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-2-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194,b-3-public.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9194`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `broker_node_group_info.0.connectivity_info.0.public_access.0.type` is set to `SERVICE_PROVIDED_EIPS` and the cluster fulfill all other requirements for public access. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_public_tls: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL IAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9098`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.iam` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_sasl_iam: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and SASL SCRAM port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9096`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS` and `client_authentication.0.sasl.0.scram` is set to `true`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_sasl_scram: pulumi_wasm_rust::Output<String>,
        /// One or more DNS names (or IP addresses) and TLS port pairs. For example, `b-1.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-2.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094,b-3.exampleClusterName.abcde.c2.kafka.us-east-1.amazonaws.com:9094`. This attribute will have a value if `encryption_info.0.encryption_in_transit.0.client_broker` is set to `TLS_PLAINTEXT` or `TLS`. The resource sorts the list alphabetically. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_tls: pulumi_wasm_rust::Output<String>,
        /// A string containing one or more DNS names (or IP addresses) and SASL IAM port pairs for VPC connectivity. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_vpc_connectivity_sasl_iam: pulumi_wasm_rust::Output<
            String,
        >,
        /// A string containing one or more DNS names (or IP addresses) and SASL SCRAM port pairs for VPC connectivity. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_vpc_connectivity_sasl_scram: pulumi_wasm_rust::Output<
            String,
        >,
        /// A string containing one or more DNS names (or IP addresses) and TLS port pairs for VPC connectivity. AWS may not always return all endpoints so the values may not be stable across applies.
        pub bootstrap_brokers_vpc_connectivity_tls: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the broker nodes of the Kafka cluster.
        pub broker_node_group_info: pulumi_wasm_rust::Output<
            super::super::types::msk::ClusterBrokerNodeGroupInfo,
        >,
        /// Configuration block for specifying a client authentication. See below.
        pub client_authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::msk::ClusterClientAuthentication>,
        >,
        /// Name of the MSK cluster.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// UUID of the MSK cluster, for use in IAM policies.
        pub cluster_uuid: pulumi_wasm_rust::Output<String>,
        /// Configuration block for specifying a MSK Configuration to attach to Kafka brokers. See below.
        pub configuration_info: pulumi_wasm_rust::Output<
            Option<super::super::types::msk::ClusterConfigurationInfo>,
        >,
        /// Current version of the MSK Cluster used for updates, e.g., `K13V1IB3VIYZZH`
        pub current_version: pulumi_wasm_rust::Output<String>,
        /// Configuration block for specifying encryption. See below.
        pub encryption_info: pulumi_wasm_rust::Output<
            Option<super::super::types::msk::ClusterEncryptionInfo>,
        >,
        /// Specify the desired enhanced MSK CloudWatch monitoring level. See [Monitoring Amazon MSK with Amazon CloudWatch](https://docs.aws.amazon.com/msk/latest/developerguide/monitoring.html)
        pub enhanced_monitoring: pulumi_wasm_rust::Output<Option<String>>,
        /// Specify the desired Kafka software version.
        pub kafka_version: pulumi_wasm_rust::Output<String>,
        /// Configuration block for streaming broker logs to Cloudwatch/S3/Kinesis Firehose. See below.
        pub logging_info: pulumi_wasm_rust::Output<
            Option<super::super::types::msk::ClusterLoggingInfo>,
        >,
        /// The desired total number of broker nodes in the kafka cluster.  It must be a multiple of the number of specified client subnets.
        pub number_of_broker_nodes: pulumi_wasm_rust::Output<i32>,
        /// Configuration block for JMX and Node monitoring for the MSK cluster. See below.
        pub open_monitoring: pulumi_wasm_rust::Output<
            Option<super::super::types::msk::ClusterOpenMonitoring>,
        >,
        /// Controls storage mode for supported storage tiers. Valid values are: `LOCAL` or `TIERED`.
        pub storage_mode: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster. The returned values are sorted alphabetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
        pub zookeeper_connect_string: pulumi_wasm_rust::Output<String>,
        /// A comma separated list of one or more hostname:port pairs to use to connect to the Apache Zookeeper cluster via TLS. The returned values are sorted alphabetically. The AWS API may not return all endpoints, so this value is not guaranteed to be stable across applies.
        pub zookeeper_connect_string_tls: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let broker_node_group_info_binding = args
            .broker_node_group_info
            .get_output(context)
            .get_inner();
        let client_authentication_binding = args
            .client_authentication
            .get_output(context)
            .get_inner();
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let configuration_info_binding = args
            .configuration_info
            .get_output(context)
            .get_inner();
        let encryption_info_binding = args
            .encryption_info
            .get_output(context)
            .get_inner();
        let enhanced_monitoring_binding = args
            .enhanced_monitoring
            .get_output(context)
            .get_inner();
        let kafka_version_binding = args.kafka_version.get_output(context).get_inner();
        let logging_info_binding = args.logging_info.get_output(context).get_inner();
        let number_of_broker_nodes_binding = args
            .number_of_broker_nodes
            .get_output(context)
            .get_inner();
        let open_monitoring_binding = args
            .open_monitoring
            .get_output(context)
            .get_inner();
        let storage_mode_binding = args.storage_mode.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:msk/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "brokerNodeGroupInfo".into(),
                    value: &broker_node_group_info_binding,
                },
                register_interface::ObjectField {
                    name: "clientAuthentication".into(),
                    value: &client_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "configurationInfo".into(),
                    value: &configuration_info_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionInfo".into(),
                    value: &encryption_info_binding,
                },
                register_interface::ObjectField {
                    name: "enhancedMonitoring".into(),
                    value: &enhanced_monitoring_binding,
                },
                register_interface::ObjectField {
                    name: "kafkaVersion".into(),
                    value: &kafka_version_binding,
                },
                register_interface::ObjectField {
                    name: "loggingInfo".into(),
                    value: &logging_info_binding,
                },
                register_interface::ObjectField {
                    name: "numberOfBrokerNodes".into(),
                    value: &number_of_broker_nodes_binding,
                },
                register_interface::ObjectField {
                    name: "openMonitoring".into(),
                    value: &open_monitoring_binding,
                },
                register_interface::ObjectField {
                    name: "storageMode".into(),
                    value: &storage_mode_binding,
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
                    name: "bootstrapBrokers".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersPublicSaslIam".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersPublicSaslScram".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersPublicTls".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersSaslIam".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersSaslScram".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersTls".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersVpcConnectivitySaslIam".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersVpcConnectivitySaslScram".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapBrokersVpcConnectivityTls".into(),
                },
                register_interface::ResultField {
                    name: "brokerNodeGroupInfo".into(),
                },
                register_interface::ResultField {
                    name: "clientAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "clusterUuid".into(),
                },
                register_interface::ResultField {
                    name: "configurationInfo".into(),
                },
                register_interface::ResultField {
                    name: "currentVersion".into(),
                },
                register_interface::ResultField {
                    name: "encryptionInfo".into(),
                },
                register_interface::ResultField {
                    name: "enhancedMonitoring".into(),
                },
                register_interface::ResultField {
                    name: "kafkaVersion".into(),
                },
                register_interface::ResultField {
                    name: "loggingInfo".into(),
                },
                register_interface::ResultField {
                    name: "numberOfBrokerNodes".into(),
                },
                register_interface::ResultField {
                    name: "openMonitoring".into(),
                },
                register_interface::ResultField {
                    name: "storageMode".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "zookeeperConnectString".into(),
                },
                register_interface::ResultField {
                    name: "zookeeperConnectStringTls".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bootstrap_brokers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokers").unwrap(),
            ),
            bootstrap_brokers_public_sasl_iam: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersPublicSaslIam").unwrap(),
            ),
            bootstrap_brokers_public_sasl_scram: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersPublicSaslScram").unwrap(),
            ),
            bootstrap_brokers_public_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersPublicTls").unwrap(),
            ),
            bootstrap_brokers_sasl_iam: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersSaslIam").unwrap(),
            ),
            bootstrap_brokers_sasl_scram: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersSaslScram").unwrap(),
            ),
            bootstrap_brokers_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersTls").unwrap(),
            ),
            bootstrap_brokers_vpc_connectivity_sasl_iam: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersVpcConnectivitySaslIam").unwrap(),
            ),
            bootstrap_brokers_vpc_connectivity_sasl_scram: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersVpcConnectivitySaslScram").unwrap(),
            ),
            bootstrap_brokers_vpc_connectivity_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapBrokersVpcConnectivityTls").unwrap(),
            ),
            broker_node_group_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("brokerNodeGroupInfo").unwrap(),
            ),
            client_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientAuthentication").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            cluster_uuid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterUuid").unwrap(),
            ),
            configuration_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationInfo").unwrap(),
            ),
            current_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentVersion").unwrap(),
            ),
            encryption_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionInfo").unwrap(),
            ),
            enhanced_monitoring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enhancedMonitoring").unwrap(),
            ),
            kafka_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kafkaVersion").unwrap(),
            ),
            logging_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingInfo").unwrap(),
            ),
            number_of_broker_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfBrokerNodes").unwrap(),
            ),
            open_monitoring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("openMonitoring").unwrap(),
            ),
            storage_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageMode").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            zookeeper_connect_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zookeeperConnectString").unwrap(),
            ),
            zookeeper_connect_string_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zookeeperConnectStringTls").unwrap(),
            ),
        }
    }
}
