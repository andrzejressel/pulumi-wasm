pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Name of the service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the namespace that the service belongs to.
        #[builder(into)]
        pub namespace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// (**Deprecated**) Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        #[builder(into, default)]
        pub tags_all: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_id_binding = args.namespace_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tags_all_binding = args.tags_all.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsConfigs"),
            ),
            health_check_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheckConfigs"),
            ),
            health_check_custom_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheckCustomConfigs"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
