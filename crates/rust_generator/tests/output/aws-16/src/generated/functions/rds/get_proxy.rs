#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProxyArgs {
        /// Name of the DB proxy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProxyResult {
        /// ARN of the DB Proxy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration(s) with authorization mechanisms to connect to the associated instance or cluster.
        pub auths: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::rds::GetProxyAuth>,
        >,
        /// Whether the proxy includes detailed information about SQL statements in its logs.
        pub debug_logging: pulumi_gestalt_rust::Output<bool>,
        /// Endpoint that you can use to connect to the DB proxy.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Kinds of databases that the proxy can connect to.
        pub engine_family: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Number of seconds a connection to the proxy can have no activity before the proxy drops the client connection.
        pub idle_client_timeout: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether Transport Layer Security (TLS) encryption is required for connections to the proxy.
        pub require_tls: pulumi_gestalt_rust::Output<bool>,
        /// ARN for the IAM role that the proxy uses to access Amazon Secrets Manager.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Provides the VPC ID of the DB proxy.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// Provides a list of VPC security groups that the proxy belongs to.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// EC2 subnet IDs for the proxy.
        pub vpc_subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProxyArgs,
    ) -> GetProxyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getProxy:getProxy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProxyResult {
            arn: o.get_field("arn"),
            auths: o.get_field("auths"),
            debug_logging: o.get_field("debugLogging"),
            endpoint: o.get_field("endpoint"),
            engine_family: o.get_field("engineFamily"),
            id: o.get_field("id"),
            idle_client_timeout: o.get_field("idleClientTimeout"),
            name: o.get_field("name"),
            require_tls: o.get_field("requireTls"),
            role_arn: o.get_field("roleArn"),
            vpc_id: o.get_field("vpcId"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
            vpc_subnet_ids: o.get_field("vpcSubnetIds"),
        }
    }
}
