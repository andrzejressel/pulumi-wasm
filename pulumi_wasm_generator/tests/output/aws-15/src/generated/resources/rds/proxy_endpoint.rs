/// Provides an RDS DB proxy endpoint resource. For additional information, see the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-proxy-endpoints.html).
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import DB proxy endpoints using the `DB-PROXY-NAME/DB-PROXY-ENDPOINT-NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/proxyEndpoint:ProxyEndpoint example example/example
/// ```
pub mod proxy_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyEndpointArgs {
        /// The identifier for the proxy endpoint. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.
        #[builder(into)]
        pub db_proxy_endpoint_name: pulumi_wasm_rust::Output<String>,
        /// The name of the DB proxy associated with the DB proxy endpoint that you create.
        #[builder(into)]
        pub db_proxy_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether the DB proxy endpoint can be used for read/write or read-only operations. The default is `READ_WRITE`. Valid values are `READ_WRITE` and `READ_ONLY`.
        #[builder(into, default)]
        pub target_role: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more VPC security group IDs to associate with the new proxy.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more VPC subnet IDs to associate with the new proxy.
        #[builder(into)]
        pub vpc_subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ProxyEndpointResult {
        /// The Amazon Resource Name (ARN) for the proxy endpoint.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The identifier for the proxy endpoint. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.
        pub db_proxy_endpoint_name: pulumi_wasm_rust::Output<String>,
        /// The name of the DB proxy associated with the DB proxy endpoint that you create.
        pub db_proxy_name: pulumi_wasm_rust::Output<String>,
        /// The endpoint that you can use to connect to the proxy. You include the endpoint value in the connection string for a database client application.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Indicates whether this endpoint is the default endpoint for the associated DB proxy.
        pub is_default: pulumi_wasm_rust::Output<bool>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether the DB proxy endpoint can be used for read/write or read-only operations. The default is `READ_WRITE`. Valid values are `READ_WRITE` and `READ_ONLY`.
        pub target_role: pulumi_wasm_rust::Output<Option<String>>,
        /// The VPC ID of the DB proxy endpoint.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// One or more VPC security group IDs to associate with the new proxy.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more VPC subnet IDs to associate with the new proxy.
        pub vpc_subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProxyEndpointArgs) -> ProxyEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_proxy_endpoint_name_binding = args.db_proxy_endpoint_name.get_inner();
        let db_proxy_name_binding = args.db_proxy_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_role_binding = args.target_role.get_inner();
        let vpc_security_group_ids_binding = args.vpc_security_group_ids.get_inner();
        let vpc_subnet_ids_binding = args.vpc_subnet_ids.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/proxyEndpoint:ProxyEndpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbProxyEndpointName".into(),
                    value: &db_proxy_endpoint_name_binding,
                },
                register_interface::ObjectField {
                    name: "dbProxyName".into(),
                    value: &db_proxy_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetRole".into(),
                    value: &target_role_binding,
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
                    name: "dbProxyEndpointName".into(),
                },
                register_interface::ResultField {
                    name: "dbProxyName".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "isDefault".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetRole".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "vpcSubnetIds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProxyEndpointResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            db_proxy_endpoint_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbProxyEndpointName").unwrap(),
            ),
            db_proxy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbProxyName").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            is_default: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isDefault").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRole").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
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
