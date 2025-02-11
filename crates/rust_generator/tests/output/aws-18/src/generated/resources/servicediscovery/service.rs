/// Provides a Service Discovery Service resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc::create(
///         "example",
///         VpcArgs::builder()
///             .cidr_block("10.0.0.0/16")
///             .enable_dns_hostnames(true)
///             .enable_dns_support(true)
///             .build_struct(),
///     );
///     let examplePrivateDnsNamespace = private_dns_namespace::create(
///         "examplePrivateDnsNamespace",
///         PrivateDnsNamespaceArgs::builder()
///             .description("example")
///             .name("example.mydomain.local")
///             .vpc("${example.id}")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .dns_config(
///                 ServiceDnsConfig::builder()
///                     .dnsRecords(
///                         vec![
///                             ServiceDnsConfigDnsRecord::builder().ttl(10). type ("A")
///                             .build_struct(),
///                         ],
///                     )
///                     .namespaceId("${examplePrivateDnsNamespace.id}")
///                     .routingPolicy("MULTIVALUE")
///                     .build_struct(),
///             )
///             .health_check_custom_config(
///                 ServiceHealthCheckCustomConfig::builder()
///                     .failureThreshold(1)
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicediscovery:PublicDnsNamespace
///     properties:
///       name: example.mydomain.com
///       description: example
///   exampleService:
///     type: aws:servicediscovery:Service
///     name: example
///     properties:
///       name: example
///       dnsConfig:
///         namespaceId: ${example.id}
///         dnsRecords:
///           - ttl: 10
///             type: A
///       healthCheckConfig:
///         failureThreshold: 10
///         resourcePath: path
///         type: HTTP
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Discovery Service using the service ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicediscovery/service:Service example 0123456789
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// The description of the service.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A complex type that contains information about the resource record sets that you want Amazon Route 53 to create when you register an instance. See `dns_config` Block for details.
        #[builder(into, default)]
        pub dns_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicediscovery::ServiceDnsConfig>,
        >,
        /// A boolean that indicates all instances should be deleted from the service so that the service can be destroyed without error. These instances are not recoverable. Defaults to `false`.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A complex type that contains settings for an optional health check. Only for Public DNS namespaces. See `health_check_config` Block for details.
        #[builder(into, default)]
        pub health_check_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicediscovery::ServiceHealthCheckConfig>,
        >,
        /// A complex type that contains settings for ECS managed health checks. See `health_check_custom_config` Block for details.
        #[builder(into, default)]
        pub health_check_custom_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicediscovery::ServiceHealthCheckCustomConfig>,
        >,
        /// The name of the service.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the namespace that you want to use to create the service.
        #[builder(into, default)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If present, specifies that the service instances are only discoverable using the `DiscoverInstances` API operation. No DNS records is registered for the service instances. The only valid value is `HTTP`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// The ARN of the service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A complex type that contains information about the resource record sets that you want Amazon Route 53 to create when you register an instance. See `dns_config` Block for details.
        pub dns_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicediscovery::ServiceDnsConfig>,
        >,
        /// A boolean that indicates all instances should be deleted from the service so that the service can be destroyed without error. These instances are not recoverable. Defaults to `false`.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A complex type that contains settings for an optional health check. Only for Public DNS namespaces. See `health_check_config` Block for details.
        pub health_check_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicediscovery::ServiceHealthCheckConfig>,
        >,
        /// A complex type that contains settings for ECS managed health checks. See `health_check_custom_config` Block for details.
        pub health_check_custom_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicediscovery::ServiceHealthCheckCustomConfig>,
        >,
        /// The name of the service.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the namespace that you want to use to create the service.
        pub namespace_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the service. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If present, specifies that the service instances are only discoverable using the `DiscoverInstances` API operation. No DNS records is registered for the service instances. The only valid value is `HTTP`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let dns_config_binding = args.dns_config.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let health_check_config_binding = args.health_check_config.get_output(context);
        let health_check_custom_config_binding = args
            .health_check_custom_config
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicediscovery/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsConfig".into(),
                    value: &dns_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckConfig".into(),
                    value: &health_check_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckCustomConfig".into(),
                    value: &health_check_custom_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            dns_config: o.get_field("dnsConfig"),
            force_destroy: o.get_field("forceDestroy"),
            health_check_config: o.get_field("healthCheckConfig"),
            health_check_custom_config: o.get_field("healthCheckCustomConfig"),
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}
