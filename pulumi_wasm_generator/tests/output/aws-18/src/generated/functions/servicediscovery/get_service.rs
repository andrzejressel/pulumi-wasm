pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Name of the service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the namespace that the service belongs to.
        #[builder(into)]
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// (**Deprecated**) Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        #[builder(into, default)]
        pub tags_all: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// ARN of the service.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the service.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Complex type that contains information about the resource record sets that you want Amazon Route 53 to create when you register an instance. See `dns_config` Block for details.
        pub dns_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::servicediscovery::GetServiceDnsConfig>,
        >,
        /// Complex type that contains settings for an optional health check. Only for Public DNS namespaces. See `health_check_config` Block for details.
        pub health_check_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::servicediscovery::GetServiceHealthCheckConfig,
            >,
        >,
        /// A complex type that contains settings for ECS managed health checks. See `health_check_custom_config` Block for details.
        pub health_check_custom_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::servicediscovery::GetServiceHealthCheckCustomConfig,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the namespace to use for DNS configuration.
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// (**Deprecated**) Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceArgs) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let namespace_id_binding = args.namespace_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let tags_all_binding = args.tags_all.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicediscovery/getService:getService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tagsAll".into(),
                    value: &tags_all_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dnsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckConfigs".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckCustomConfigs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespaceId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dns_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsConfigs").unwrap(),
            ),
            health_check_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckConfigs").unwrap(),
            ),
            health_check_custom_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckCustomConfigs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
