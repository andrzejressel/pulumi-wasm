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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod proxy_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyTargetArgs {
        /// DB cluster identifier.
        ///
        /// **NOTE:** Either `db_instance_identifier` or `db_cluster_identifier` should be specified and both should not be specified together
        #[builder(into, default)]
        pub db_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DB instance identifier.
        #[builder(into, default)]
        pub db_instance_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the DB proxy.
        #[builder(into)]
        pub db_proxy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the target group.
        #[builder(into)]
        pub target_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyTargetResult {
        /// DB cluster identifier.
        ///
        /// **NOTE:** Either `db_instance_identifier` or `db_cluster_identifier` should be specified and both should not be specified together
        pub db_cluster_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// DB instance identifier.
        pub db_instance_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the DB proxy.
        pub db_proxy_name: pulumi_gestalt_rust::Output<String>,
        /// Hostname for the target RDS DB Instance. Only returned for `RDS_INSTANCE` type.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Port for the target RDS DB Instance or Aurora DB Cluster.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Identifier representing the DB Instance or DB Cluster target.
        pub rds_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) for the DB instance or DB cluster. Currently not returned by the RDS API.
        pub target_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the target group.
        pub target_group_name: pulumi_gestalt_rust::Output<String>,
        /// DB Cluster identifier for the DB Instance target. Not returned unless manually importing an `RDS_INSTANCE` target that is part of a DB Cluster.
        pub tracked_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Type of targetE.g., `RDS_INSTANCE` or `TRACKED_CLUSTER`
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyTargetArgs,
    ) -> ProxyTargetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_cluster_identifier_binding = args
            .db_cluster_identifier
            .get_output(context);
        let db_instance_identifier_binding = args
            .db_instance_identifier
            .get_output(context);
        let db_proxy_name_binding = args.db_proxy_name.get_output(context);
        let target_group_name_binding = args.target_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/proxyTarget:ProxyTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: db_cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbInstanceIdentifier".into(),
                    value: db_instance_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbProxyName".into(),
                    value: db_proxy_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetGroupName".into(),
                    value: target_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProxyTargetResult {
            db_cluster_identifier: o.get_field("dbClusterIdentifier"),
            db_instance_identifier: o.get_field("dbInstanceIdentifier"),
            db_proxy_name: o.get_field("dbProxyName"),
            endpoint: o.get_field("endpoint"),
            port: o.get_field("port"),
            rds_resource_id: o.get_field("rdsResourceId"),
            target_arn: o.get_field("targetArn"),
            target_group_name: o.get_field("targetGroupName"),
            tracked_cluster_id: o.get_field("trackedClusterId"),
            type_: o.get_field("type"),
        }
    }
}
