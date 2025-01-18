/// Provides an RDS DB proxy target resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:rds:Proxy
///     properties:
///       name: example
///       debugLogging: false
///       engineFamily: MYSQL
///       idleClientTimeout: 1800
///       requireTls: true
///       roleArn: ${exampleAwsIamRole.arn}
///       vpcSecurityGroupIds:
///         - ${exampleAwsSecurityGroup.id}
///       vpcSubnetIds:
///         - ${exampleAwsSubnet.id}
///       auths:
///         - authScheme: SECRETS
///           description: example
///           iamAuth: DISABLED
///           secretArn: ${exampleAwsSecretsmanagerSecret.arn}
///       tags:
///         Name: example
///         Key: value
///   exampleProxyDefaultTargetGroup:
///     type: aws:rds:ProxyDefaultTargetGroup
///     name: example
///     properties:
///       dbProxyName: ${example.name}
///       connectionPoolConfig:
///         connectionBorrowTimeout: 120
///         initQuery: SET x=1, y=2
///         maxConnectionsPercent: 100
///         maxIdleConnectionsPercent: 50
///         sessionPinningFilters:
///           - EXCLUDE_VARIABLE_SETS
///   exampleProxyTarget:
///     type: aws:rds:ProxyTarget
///     name: example
///     properties:
///       dbInstanceIdentifier: ${exampleAwsDbInstance.identifier}
///       dbProxyName: ${example.name}
///       targetGroupName: ${exampleProxyDefaultTargetGroup.name}
/// ```
///
/// ## Import
///
/// Provisioned Clusters:
///
/// __Using `pulumi import` to import__ RDS DB Proxy Targets using the `db_proxy_name`, `target_group_name`, target type (such as `RDS_INSTANCE` or `TRACKED_CLUSTER`), and resource identifier separated by forward slashes (`/`). For example:
///
/// Instances:
///
/// ```sh
/// $ pulumi import aws:rds/proxyTarget:ProxyTarget example example-proxy/default/RDS_INSTANCE/example-instance
/// ```
/// Provisioned Clusters:
///
/// ```sh
/// $ pulumi import aws:rds/proxyTarget:ProxyTarget example example-proxy/default/TRACKED_CLUSTER/example-cluster
/// ```
pub mod proxy_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyTargetArgs {
        /// DB cluster identifier.
        ///
        /// **NOTE:** Either `db_instance_identifier` or `db_cluster_identifier` should be specified and both should not be specified together
        #[builder(into, default)]
        pub db_cluster_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// DB instance identifier.
        #[builder(into, default)]
        pub db_instance_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the DB proxy.
        #[builder(into)]
        pub db_proxy_name: pulumi_wasm_rust::Output<String>,
        /// The name of the target group.
        #[builder(into)]
        pub target_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyTargetResult {
        /// DB cluster identifier.
        ///
        /// **NOTE:** Either `db_instance_identifier` or `db_cluster_identifier` should be specified and both should not be specified together
        pub db_cluster_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// DB instance identifier.
        pub db_instance_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the DB proxy.
        pub db_proxy_name: pulumi_wasm_rust::Output<String>,
        /// Hostname for the target RDS DB Instance. Only returned for `RDS_INSTANCE` type.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Port for the target RDS DB Instance or Aurora DB Cluster.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Identifier representing the DB Instance or DB Cluster target.
        pub rds_resource_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) for the DB instance or DB cluster. Currently not returned by the RDS API.
        pub target_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the target group.
        pub target_group_name: pulumi_wasm_rust::Output<String>,
        /// DB Cluster identifier for the DB Instance target. Not returned unless manually importing an `RDS_INSTANCE` target that is part of a DB Cluster.
        pub tracked_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Type of targetE.g., `RDS_INSTANCE` or `TRACKED_CLUSTER`
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProxyTargetArgs) -> ProxyTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_cluster_identifier_binding = args.db_cluster_identifier.get_inner();
        let db_instance_identifier_binding = args.db_instance_identifier.get_inner();
        let db_proxy_name_binding = args.db_proxy_name.get_inner();
        let target_group_name_binding = args.target_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/proxyTarget:ProxyTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: &db_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dbInstanceIdentifier".into(),
                    value: &db_instance_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "dbProxyName".into(),
                    value: &db_proxy_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetGroupName".into(),
                    value: &target_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dbClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbInstanceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "dbProxyName".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "rdsResourceId".into(),
                },
                register_interface::ResultField {
                    name: "targetArn".into(),
                },
                register_interface::ResultField {
                    name: "targetGroupName".into(),
                },
                register_interface::ResultField {
                    name: "trackedClusterId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProxyTargetResult {
            db_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterIdentifier").unwrap(),
            ),
            db_instance_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstanceIdentifier").unwrap(),
            ),
            db_proxy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbProxyName").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            rds_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rdsResourceId").unwrap(),
            ),
            target_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetArn").unwrap(),
            ),
            target_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetGroupName").unwrap(),
            ),
            tracked_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackedClusterId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
