///
///
/// ## Import
///
/// Using `pulumi import`, import DB proxies using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/proxy:Proxy example example
/// ```
pub mod proxy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyArgs {
        /// Configuration block(s) with authorization mechanisms to connect to the associated instances or clusters. Described below.
        #[builder(into)]
        pub auths: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::rds::ProxyAuth>,
        >,
        /// Whether the proxy includes detailed information about SQL statements in its logs. This information helps you to debug issues involving SQL behavior or the performance and scalability of the proxy connections. The debug information includes the text of SQL statements that you submit through the proxy. Thus, only enable this setting when needed for debugging, and only when you have security measures in place to safeguard any sensitive information that appears in the logs.
        #[builder(into, default)]
        pub debug_logging: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The kinds of databases that the proxy can connect to. This value determines which database network protocol the proxy recognizes when it interprets network traffic to and from the database. For Aurora MySQL, RDS for MariaDB, and RDS for MySQL databases, specify `MYSQL`. For Aurora PostgreSQL and RDS for PostgreSQL databases, specify `POSTGRESQL`. For RDS for Microsoft SQL Server, specify `SQLSERVER`. Valid values are `MYSQL`, `POSTGRESQL`, and `SQLSERVER`.
        #[builder(into)]
        pub engine_family: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of seconds that a connection to the proxy can be inactive before the proxy disconnects it. You can set this value higher or lower than the connection timeout limit for the associated database.
        #[builder(into, default)]
        pub idle_client_timeout: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The identifier for the proxy. This name must be unique for all proxies owned by your AWS account in the specified AWS Region. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A Boolean parameter that specifies whether Transport Layer Security (TLS) encryption is required for connections to the proxy. By enabling this setting, you can enforce encrypted TLS connections to the proxy.
        #[builder(into, default)]
        pub require_tls: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the IAM role that the proxy uses to access secrets in AWS Secrets Manager.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// One or more VPC security group IDs to associate with the new proxy.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more VPC subnet IDs to associate with the new proxy.
        #[builder(into)]
        pub vpc_subnet_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ProxyResult {
        /// The Amazon Resource Name (ARN) for the proxy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) with authorization mechanisms to connect to the associated instances or clusters. Described below.
        pub auths: pulumi_wasm_rust::Output<Vec<super::super::types::rds::ProxyAuth>>,
        /// Whether the proxy includes detailed information about SQL statements in its logs. This information helps you to debug issues involving SQL behavior or the performance and scalability of the proxy connections. The debug information includes the text of SQL statements that you submit through the proxy. Thus, only enable this setting when needed for debugging, and only when you have security measures in place to safeguard any sensitive information that appears in the logs.
        pub debug_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// The endpoint that you can use to connect to the proxy. You include the endpoint value in the connection string for a database client application.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The kinds of databases that the proxy can connect to. This value determines which database network protocol the proxy recognizes when it interprets network traffic to and from the database. For Aurora MySQL, RDS for MariaDB, and RDS for MySQL databases, specify `MYSQL`. For Aurora PostgreSQL and RDS for PostgreSQL databases, specify `POSTGRESQL`. For RDS for Microsoft SQL Server, specify `SQLSERVER`. Valid values are `MYSQL`, `POSTGRESQL`, and `SQLSERVER`.
        pub engine_family: pulumi_wasm_rust::Output<String>,
        /// The number of seconds that a connection to the proxy can be inactive before the proxy disconnects it. You can set this value higher or lower than the connection timeout limit for the associated database.
        pub idle_client_timeout: pulumi_wasm_rust::Output<i32>,
        /// The identifier for the proxy. This name must be unique for all proxies owned by your AWS account in the specified AWS Region. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A Boolean parameter that specifies whether Transport Layer Security (TLS) encryption is required for connections to the proxy. By enabling this setting, you can enforce encrypted TLS connections to the proxy.
        pub require_tls: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the IAM role that the proxy uses to access secrets in AWS Secrets Manager.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// One or more VPC security group IDs to associate with the new proxy.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more VPC subnet IDs to associate with the new proxy.
        pub vpc_subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProxyArgs,
    ) -> ProxyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auths_binding = args.auths.get_output(context).get_inner();
        let debug_logging_binding = args.debug_logging.get_output(context).get_inner();
        let engine_family_binding = args.engine_family.get_output(context).get_inner();
        let idle_client_timeout_binding = args
            .idle_client_timeout
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let require_tls_binding = args.require_tls.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context)
            .get_inner();
        let vpc_subnet_ids_binding = args.vpc_subnet_ids.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/proxy:Proxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auths".into(),
                    value: &auths_binding,
                },
                register_interface::ObjectField {
                    name: "debugLogging".into(),
                    value: &debug_logging_binding,
                },
                register_interface::ObjectField {
                    name: "engineFamily".into(),
                    value: &engine_family_binding,
                },
                register_interface::ObjectField {
                    name: "idleClientTimeout".into(),
                    value: &idle_client_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requireTls".into(),
                    value: &require_tls_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSubnetIds".into(),
                    value: &vpc_subnet_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "auths".into(),
                },
                register_interface::ResultField {
                    name: "debugLogging".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "engineFamily".into(),
                },
                register_interface::ResultField {
                    name: "idleClientTimeout".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "requireTls".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "vpcSubnetIds".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProxyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auths").unwrap(),
            ),
            debug_logging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("debugLogging").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            engine_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineFamily").unwrap(),
            ),
            idle_client_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idleClientTimeout").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            require_tls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requireTls").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
            vpc_subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSubnetIds").unwrap(),
            ),
        }
    }
}
