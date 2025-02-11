/// Provides a resource to create an EventBridge Global Endpoint.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let this = event_endpoint::create(
///         "this",
///         EventEndpointArgs::builder()
///             .event_buses(
///                 vec![
///                     EventEndpointEventBus::builder().eventBusArn("${primary.arn}")
///                     .build_struct(), EventEndpointEventBus::builder()
///                     .eventBusArn("${secondary.arn}").build_struct(),
///                 ],
///             )
///             .name("global-endpoint")
///             .replication_config(
///                 EventEndpointReplicationConfig::builder()
///                     .state("DISABLED")
///                     .build_struct(),
///             )
///             .role_arn("${replication.arn}")
///             .routing_config(
///                 EventEndpointRoutingConfig::builder()
///                     .failoverConfig(
///                         EventEndpointRoutingConfigFailoverConfig::builder()
///                             .primary(
///                                 EventEndpointRoutingConfigFailoverConfigPrimary::builder()
///                                     .healthCheck("${primaryAwsRoute53HealthCheck.arn}")
///                                     .build_struct(),
///                             )
///                             .secondary(
///                                 EventEndpointRoutingConfigFailoverConfigSecondary::builder()
///                                     .route("us-east-2")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge Global Endpoints using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventEndpoint:EventEndpoint imported_endpoint example-endpoint
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventEndpointArgs {
        /// A description of the global endpoint.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The event buses to use. The names of the event buses must be identical in each Region. Exactly two event buses are required. Documented below.
        #[builder(into)]
        pub event_buses: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::cloudwatch::EventEndpointEventBus>,
        >,
        /// The name of the global endpoint.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters used for replication. Documented below.
        #[builder(into, default)]
        pub replication_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventEndpointReplicationConfig>,
        >,
        /// The ARN of the IAM role used for replication between event buses.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters used for routing, including the health check and secondary Region. Documented below.
        #[builder(into)]
        pub routing_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudwatch::EventEndpointRoutingConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct EventEndpointResult {
        /// The ARN of the endpoint that was created.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the global endpoint.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL of the endpoint that was created.
        pub endpoint_url: pulumi_gestalt_rust::Output<String>,
        /// The event buses to use. The names of the event buses must be identical in each Region. Exactly two event buses are required. Documented below.
        pub event_buses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudwatch::EventEndpointEventBus>,
        >,
        /// The name of the global endpoint.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Parameters used for replication. Documented below.
        pub replication_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventEndpointReplicationConfig>,
        >,
        /// The ARN of the IAM role used for replication between event buses.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Parameters used for routing, including the health check and secondary Region. Documented below.
        pub routing_config: pulumi_gestalt_rust::Output<
            super::super::types::cloudwatch::EventEndpointRoutingConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventEndpointArgs,
    ) -> EventEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let event_buses_binding = args.event_buses.get_output(context);
        let name_binding = args.name.get_output(context);
        let replication_config_binding = args.replication_config.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let routing_config_binding = args.routing_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventEndpoint:EventEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventBuses".into(),
                    value: &event_buses_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationConfig".into(),
                    value: &replication_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingConfig".into(),
                    value: &routing_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventEndpointResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            endpoint_url: o.get_field("endpointUrl"),
            event_buses: o.get_field("eventBuses"),
            name: o.get_field("name"),
            replication_config: o.get_field("replicationConfig"),
            role_arn: o.get_field("roleArn"),
            routing_config: o.get_field("routingConfig"),
        }
    }
}
