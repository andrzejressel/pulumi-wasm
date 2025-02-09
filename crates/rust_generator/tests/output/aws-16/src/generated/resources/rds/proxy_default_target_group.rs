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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod proxy_default_target_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyDefaultTargetGroupArgs {
        /// The settings that determine the size and behavior of the connection pool for the target group.
        #[builder(into, default)]
        pub connection_pool_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ProxyDefaultTargetGroupConnectionPoolConfig>,
        >,
        /// Name of the RDS DB Proxy.
        #[builder(into)]
        pub db_proxy_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyDefaultTargetGroupResult {
        /// The Amazon Resource Name (ARN) representing the target group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The settings that determine the size and behavior of the connection pool for the target group.
        pub connection_pool_config: pulumi_gestalt_rust::Output<
            super::super::types::rds::ProxyDefaultTargetGroupConnectionPoolConfig,
        >,
        /// Name of the RDS DB Proxy.
        pub db_proxy_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the default target group.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyDefaultTargetGroupArgs,
    ) -> ProxyDefaultTargetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_pool_config_binding = args
            .connection_pool_config
            .get_output(context);
        let db_proxy_name_binding = args.db_proxy_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/proxyDefaultTargetGroup:ProxyDefaultTargetGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionPoolConfig".into(),
                    value: connection_pool_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbProxyName".into(),
                    value: db_proxy_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProxyDefaultTargetGroupResult {
            arn: o.get_field("arn"),
            connection_pool_config: o.get_field("connectionPoolConfig"),
            db_proxy_name: o.get_field("dbProxyName"),
            name: o.get_field("name"),
        }
    }
}
