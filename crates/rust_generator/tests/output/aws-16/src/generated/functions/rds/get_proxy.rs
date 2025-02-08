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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetProxyArgs,
    ) -> GetProxyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getProxy:getProxy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProxyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auths: pulumi_gestalt_rust::__private::into_domain(o.extract_field("auths")),
            debug_logging: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("debugLogging"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            engine_family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineFamily"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            idle_client_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("idleClientTimeout"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            require_tls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requireTls"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcId"),
            ),
            vpc_security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcSecurityGroupIds"),
            ),
            vpc_subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcSubnetIds"),
            ),
        }
    }
}
