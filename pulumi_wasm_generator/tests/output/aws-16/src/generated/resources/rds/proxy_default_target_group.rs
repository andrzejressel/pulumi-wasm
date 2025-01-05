/// Provides a resource to manage an RDS DB proxy default target group resource.
///
/// The `aws.rds.ProxyDefaultTargetGroup` behaves differently from normal resources, in that the provider does not _create_ or _destroy_ this resource, since it implicitly exists as part of an RDS DB Proxy. On the provider resource creation it is automatically imported and on resource destruction, the provider performs no actions in RDS.
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
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DB proxy default target groups using the `db_proxy_name`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/proxyDefaultTargetGroup:ProxyDefaultTargetGroup example example
/// ```
pub mod proxy_default_target_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyDefaultTargetGroupArgs {
        /// The settings that determine the size and behavior of the connection pool for the target group.
        #[builder(into, default)]
        pub connection_pool_config: pulumi_wasm_rust::Output<
            Option<super::super::types::rds::ProxyDefaultTargetGroupConnectionPoolConfig>,
        >,
        /// Name of the RDS DB Proxy.
        #[builder(into)]
        pub db_proxy_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyDefaultTargetGroupResult {
        /// The Amazon Resource Name (ARN) representing the target group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The settings that determine the size and behavior of the connection pool for the target group.
        pub connection_pool_config: pulumi_wasm_rust::Output<
            super::super::types::rds::ProxyDefaultTargetGroupConnectionPoolConfig,
        >,
        /// Name of the RDS DB Proxy.
        pub db_proxy_name: pulumi_wasm_rust::Output<String>,
        /// The name of the default target group.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ProxyDefaultTargetGroupArgs,
    ) -> ProxyDefaultTargetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_pool_config_binding = args.connection_pool_config.get_inner();
        let db_proxy_name_binding = args.db_proxy_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/proxyDefaultTargetGroup:ProxyDefaultTargetGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionPoolConfig".into(),
                    value: &connection_pool_config_binding,
                },
                register_interface::ObjectField {
                    name: "dbProxyName".into(),
                    value: &db_proxy_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "connectionPoolConfig".into(),
                },
                register_interface::ResultField {
                    name: "dbProxyName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProxyDefaultTargetGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            connection_pool_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionPoolConfig").unwrap(),
            ),
            db_proxy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbProxyName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
