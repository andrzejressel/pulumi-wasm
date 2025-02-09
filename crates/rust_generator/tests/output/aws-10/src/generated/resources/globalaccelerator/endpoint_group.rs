/// Provides a Global Accelerator endpoint group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint_group::create(
///         "example",
///         EndpointGroupArgs::builder()
///             .endpoint_configurations(
///                 vec![
///                     EndpointGroupEndpointConfiguration::builder()
///                     .endpointId("${exampleAwsLb.arn}").weight(100).build_struct(),
///                 ],
///             )
///             .listener_arn("${exampleAwsGlobalacceleratorListener.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Global Accelerator endpoint groups using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/endpointGroup:EndpointGroup example arn:aws:globalaccelerator::111111111111:accelerator/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/listener/xxxxxxx/endpoint-group/xxxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointGroupArgs {
        /// The list of endpoint objects. Fields documented below.
        #[builder(into, default)]
        pub endpoint_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::globalaccelerator::EndpointGroupEndpointConfiguration,
                >,
            >,
        >,
        /// The name of the AWS Region where the endpoint group is located.
        #[builder(into, default)]
        pub endpoint_group_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The time—10 seconds or 30 seconds—between each health check for an endpoint. The default value is 30.
        #[builder(into, default)]
        pub health_check_interval_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// If the protocol is HTTP/S, then this specifies the path that is the destination for health check targets. The default value is slash (`/`). the provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub health_check_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default port is the listener port that this endpoint group is associated with. If listener port is a list of ports, Global Accelerator uses the first port in the list.
        /// the provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub health_check_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The protocol that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default value is TCP.
        #[builder(into, default)]
        pub health_check_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the listener.
        #[builder(into)]
        pub listener_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Override specific listener ports used to route traffic to endpoints that are part of this endpoint group. Fields documented below.
        #[builder(into, default)]
        pub port_overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::globalaccelerator::EndpointGroupPortOverride>,
            >,
        >,
        /// The number of consecutive health checks required to set the state of a healthy endpoint to unhealthy, or to set an unhealthy endpoint to healthy. The default value is 3.
        #[builder(into, default)]
        pub threshold_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The percentage of traffic to send to an AWS Region. Additional traffic is distributed to other endpoint groups for this listener. The default value is 100.
        #[builder(into, default)]
        pub traffic_dial_percentage: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    }
    #[allow(dead_code)]
    pub struct EndpointGroupResult {
        /// The Amazon Resource Name (ARN) of the endpoint group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The list of endpoint objects. Fields documented below.
        pub endpoint_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::globalaccelerator::EndpointGroupEndpointConfiguration,
                >,
            >,
        >,
        /// The name of the AWS Region where the endpoint group is located.
        pub endpoint_group_region: pulumi_gestalt_rust::Output<String>,
        /// The time—10 seconds or 30 seconds—between each health check for an endpoint. The default value is 30.
        pub health_check_interval_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If the protocol is HTTP/S, then this specifies the path that is the destination for health check targets. The default value is slash (`/`). the provider will only perform drift detection of its value when present in a configuration.
        pub health_check_path: pulumi_gestalt_rust::Output<String>,
        /// The port that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default port is the listener port that this endpoint group is associated with. If listener port is a list of ports, Global Accelerator uses the first port in the list.
        /// the provider will only perform drift detection of its value when present in a configuration.
        pub health_check_port: pulumi_gestalt_rust::Output<i32>,
        /// The protocol that AWS Global Accelerator uses to check the health of endpoints that are part of this endpoint group. The default value is TCP.
        pub health_check_protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the listener.
        pub listener_arn: pulumi_gestalt_rust::Output<String>,
        /// Override specific listener ports used to route traffic to endpoints that are part of this endpoint group. Fields documented below.
        pub port_overrides: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::globalaccelerator::EndpointGroupPortOverride>,
            >,
        >,
        /// The number of consecutive health checks required to set the state of a healthy endpoint to unhealthy, or to set an unhealthy endpoint to healthy. The default value is 3.
        pub threshold_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The percentage of traffic to send to an AWS Region. Additional traffic is distributed to other endpoint groups for this listener. The default value is 100.
        pub traffic_dial_percentage: pulumi_gestalt_rust::Output<Option<f64>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointGroupArgs,
    ) -> EndpointGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_configurations_binding = args
            .endpoint_configurations
            .get_output(context);
        let endpoint_group_region_binding = args
            .endpoint_group_region
            .get_output(context);
        let health_check_interval_seconds_binding = args
            .health_check_interval_seconds
            .get_output(context);
        let health_check_path_binding = args.health_check_path.get_output(context);
        let health_check_port_binding = args.health_check_port.get_output(context);
        let health_check_protocol_binding = args
            .health_check_protocol
            .get_output(context);
        let listener_arn_binding = args.listener_arn.get_output(context);
        let port_overrides_binding = args.port_overrides.get_output(context);
        let threshold_count_binding = args.threshold_count.get_output(context);
        let traffic_dial_percentage_binding = args
            .traffic_dial_percentage
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:globalaccelerator/endpointGroup:EndpointGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointConfigurations".into(),
                    value: endpoint_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointGroupRegion".into(),
                    value: endpoint_group_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckIntervalSeconds".into(),
                    value: health_check_interval_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckPath".into(),
                    value: health_check_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckPort".into(),
                    value: health_check_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthCheckProtocol".into(),
                    value: health_check_protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listenerArn".into(),
                    value: listener_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "portOverrides".into(),
                    value: port_overrides_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thresholdCount".into(),
                    value: threshold_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficDialPercentage".into(),
                    value: traffic_dial_percentage_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointGroupResult {
            arn: o.get_field("arn"),
            endpoint_configurations: o.get_field("endpointConfigurations"),
            endpoint_group_region: o.get_field("endpointGroupRegion"),
            health_check_interval_seconds: o.get_field("healthCheckIntervalSeconds"),
            health_check_path: o.get_field("healthCheckPath"),
            health_check_port: o.get_field("healthCheckPort"),
            health_check_protocol: o.get_field("healthCheckProtocol"),
            listener_arn: o.get_field("listenerArn"),
            port_overrides: o.get_field("portOverrides"),
            threshold_count: o.get_field("thresholdCount"),
            traffic_dial_percentage: o.get_field("trafficDialPercentage"),
        }
    }
}
