/// Provides a DAX Cluster resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = cluster::create(
///         "bar",
///         ClusterArgs::builder()
///             .cluster_name("cluster-example")
///             .iam_role_arn("${example.arn}")
///             .node_type("dax.r4.large")
///             .replication_factor(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DAX Clusters using the `cluster_name`. For example:
///
/// ```sh
/// $ pulumi import aws:dax/cluster:Cluster my_cluster my_cluster
/// ```
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// List of Availability Zones in which the
        /// nodes will be created
        #[builder(into, default)]
        pub availability_zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The type of encryption the
        /// cluster's endpoint should support. Valid values are: `NONE` and `TLS`.
        /// Default value is `NONE`.
        #[builder(into, default)]
        pub cluster_endpoint_encryption_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Group identifier. DAX converts this name to
        /// lowercase
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Description for the cluster
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid Amazon Resource Name (ARN) that identifies
        /// an IAM role. At runtime, DAX will assume this role and use the role's
        /// permissions to access DynamoDB on your behalf
        #[builder(into)]
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the weekly time range for when
        /// maintenance on the cluster is performed. The format is `ddd:hh24:mi-ddd:hh24:mi`
        /// (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example:
        /// `sun:05:00-sun:09:00`
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::Output<Option<String>>,
        /// The compute and memory capacity of the nodes. See
        /// [Nodes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DAX.concepts.cluster.html#DAX.concepts.nodes) for supported node types
        #[builder(into)]
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// An Amazon Resource Name (ARN) of an
        /// SNS topic to send DAX notifications to. Example:
        /// `arn:aws:sns:us-east-1:012345678999:my_sns_topic`
        #[builder(into, default)]
        pub notification_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the parameter group to associate
        /// with this DAX cluster
        #[builder(into, default)]
        pub parameter_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of nodes in the DAX cluster. A
        /// replication factor of 1 will create a single-node cluster, without any read
        /// replicas
        #[builder(into)]
        pub replication_factor: pulumi_wasm_rust::Output<i32>,
        /// One or more VPC security groups associated
        /// with the cluster
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Encrypt at rest options
        #[builder(into, default)]
        pub server_side_encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::dax::ClusterServerSideEncryption>,
        >,
        /// Name of the subnet group to be used for the
        /// cluster
        #[builder(into, default)]
        pub subnet_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The ARN of the DAX cluster
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of Availability Zones in which the
        /// nodes will be created
        pub availability_zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The DNS name of the DAX cluster without the port appended
        pub cluster_address: pulumi_wasm_rust::Output<String>,
        /// The type of encryption the
        /// cluster's endpoint should support. Valid values are: `NONE` and `TLS`.
        /// Default value is `NONE`.
        pub cluster_endpoint_encryption_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Group identifier. DAX converts this name to
        /// lowercase
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The configuration endpoint for this DAX cluster,
        /// consisting of a DNS name and a port number
        pub configuration_endpoint: pulumi_wasm_rust::Output<String>,
        /// Description for the cluster
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid Amazon Resource Name (ARN) that identifies
        /// an IAM role. At runtime, DAX will assume this role and use the role's
        /// permissions to access DynamoDB on your behalf
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the weekly time range for when
        /// maintenance on the cluster is performed. The format is `ddd:hh24:mi-ddd:hh24:mi`
        /// (24H Clock UTC). The minimum maintenance window is a 60 minute period. Example:
        /// `sun:05:00-sun:09:00`
        pub maintenance_window: pulumi_wasm_rust::Output<String>,
        /// The compute and memory capacity of the nodes. See
        /// [Nodes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DAX.concepts.cluster.html#DAX.concepts.nodes) for supported node types
        pub node_type: pulumi_wasm_rust::Output<String>,
        /// List of node objects including `id`, `address`, `port` and
        /// `availability_zone`. Referenceable e.g., as
        /// `${aws_dax_cluster.test.nodes.0.address}`
        pub nodes: pulumi_wasm_rust::Output<Vec<super::super::types::dax::ClusterNode>>,
        /// An Amazon Resource Name (ARN) of an
        /// SNS topic to send DAX notifications to. Example:
        /// `arn:aws:sns:us-east-1:012345678999:my_sns_topic`
        pub notification_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the parameter group to associate
        /// with this DAX cluster
        pub parameter_group_name: pulumi_wasm_rust::Output<String>,
        /// The port used by the configuration endpoint
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The number of nodes in the DAX cluster. A
        /// replication factor of 1 will create a single-node cluster, without any read
        /// replicas
        pub replication_factor: pulumi_wasm_rust::Output<i32>,
        /// One or more VPC security groups associated
        /// with the cluster
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Encrypt at rest options
        pub server_side_encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::dax::ClusterServerSideEncryption>,
        >,
        /// Name of the subnet group to be used for the
        /// cluster
        pub subnet_group_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zones_binding = args.availability_zones.get_inner();
        let cluster_endpoint_encryption_type_binding = args
            .cluster_endpoint_encryption_type
            .get_inner();
        let cluster_name_binding = args.cluster_name.get_inner();
        let description_binding = args.description.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let maintenance_window_binding = args.maintenance_window.get_inner();
        let node_type_binding = args.node_type.get_inner();
        let notification_topic_arn_binding = args.notification_topic_arn.get_inner();
        let parameter_group_name_binding = args.parameter_group_name.get_inner();
        let replication_factor_binding = args.replication_factor.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let server_side_encryption_binding = args.server_side_encryption.get_inner();
        let subnet_group_name_binding = args.subnet_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dax/cluster:Cluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZones".into(),
                    value: &availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "clusterEndpointEncryptionType".into(),
                    value: &cluster_endpoint_encryption_type_binding,
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
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "nodeType".into(),
                    value: &node_type_binding,
                },
                register_interface::ObjectField {
                    name: "notificationTopicArn".into(),
                    value: &notification_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "parameterGroupName".into(),
                    value: &parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "replicationFactor".into(),
                    value: &replication_factor_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryption".into(),
                    value: &server_side_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "subnetGroupName".into(),
                    value: &subnet_group_name_binding,
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
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "clusterAddress".into(),
                },
                register_interface::ResultField {
                    name: "clusterEndpointEncryptionType".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "configurationEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "nodeType".into(),
                },
                register_interface::ResultField {
                    name: "nodes".into(),
                },
                register_interface::ResultField {
                    name: "notificationTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "parameterGroupName".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "replicationFactor".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "serverSideEncryption".into(),
                },
                register_interface::ResultField {
                    name: "subnetGroupName".into(),
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
        ClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            cluster_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterAddress").unwrap(),
            ),
            cluster_endpoint_encryption_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterEndpointEncryptionType").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            configuration_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationEndpoint").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeType").unwrap(),
            ),
            nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodes").unwrap(),
            ),
            notification_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationTopicArn").unwrap(),
            ),
            parameter_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterGroupName").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            replication_factor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationFactor").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            server_side_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverSideEncryption").unwrap(),
            ),
            subnet_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetGroupName").unwrap(),
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