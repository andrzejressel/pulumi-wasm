pub mod get_proxy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProxyArgs {
        /// Name of the DB proxy.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetProxyResult {
        /// ARN of the DB Proxy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration(s) with authorization mechanisms to connect to the associated instance or cluster.
        pub auths: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::rds::GetProxyAuth>,
        >,
        /// Whether the proxy includes detailed information about SQL statements in its logs.
        pub debug_logging: pulumi_wasm_rust::Output<bool>,
        /// Endpoint that you can use to connect to the DB proxy.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Kinds of databases that the proxy can connect to.
        pub engine_family: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Number of seconds a connection to the proxy can have no activity before the proxy drops the client connection.
        pub idle_client_timeout: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether Transport Layer Security (TLS) encryption is required for connections to the proxy.
        pub require_tls: pulumi_wasm_rust::Output<bool>,
        /// ARN for the IAM role that the proxy uses to access Amazon Secrets Manager.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Provides the VPC ID of the DB proxy.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// Provides a list of VPC security groups that the proxy belongs to.
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// EC2 subnet IDs for the proxy.
        pub vpc_subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProxyArgs) -> GetProxyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getProxy:getProxy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProxyResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
