pub mod get_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// CodeStar Connection ARN.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// CodeStar Connection name.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of key-value resource tags to associate with the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectionResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// CodeStar Connection status. Possible values are `PENDING`, `AVAILABLE` and `ERROR`.
        pub connection_status: pulumi_wasm_rust::Output<String>,
        /// ARN of the host associated with the connection.
        pub host_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the CodeStar Connection. The name is unique in the calling AWS account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of the external provider where your third-party code repository is configured. Possible values are `Bitbucket`, `GitHub` and `GitLab`. For connections to GitHub Enterprise Server or GitLab Self-Managed instances, you must create an aws.codestarconnections.Host resource and use `host_arn` instead.
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// Map of key-value resource tags to associate with the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConnectionArgs) -> GetConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codestarconnections/getConnection:getConnection".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "connectionStatus".into(),
                },
                register_interface::ResultField {
                    name: "hostArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "providerType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            connection_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStatus").unwrap(),
            ),
            host_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}