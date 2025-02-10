#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Name of the service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the namespace that the service belongs to.
        #[builder(into)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// (**Deprecated**) Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        #[builder(into, default)]
        pub tags_all: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// ARN of the service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the service.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Complex type that contains information about the resource record sets that you want Amazon Route 53 to create when you register an instance. See `dns_config` Block for details.
        pub dns_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::servicediscovery::GetServiceDnsConfig>,
        >,
        /// Complex type that contains settings for an optional health check. Only for Public DNS namespaces. See `health_check_config` Block for details.
        pub health_check_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::servicediscovery::GetServiceHealthCheckConfig,
            >,
        >,
        /// A complex type that contains settings for ECS managed health checks. See `health_check_custom_config` Block for details.
        pub health_check_custom_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::servicediscovery::GetServiceHealthCheckCustomConfig,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the namespace to use for DNS configuration.
        pub namespace_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// (**Deprecated**) Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tags_all_binding = args.tags_all.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:servicediscovery/getService:getService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceId".into(),
                    value: namespace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagsAll".into(),
                    value: tags_all_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            dns_configs: o.get_field("dnsConfigs"),
            health_check_configs: o.get_field("healthCheckConfigs"),
            health_check_custom_configs: o.get_field("healthCheckCustomConfigs"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
