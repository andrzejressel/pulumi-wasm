/// Provides a Traffic mirror target.
/// Read [limits and considerations](https://docs.aws.amazon.com/vpc/latest/mirroring/traffic-mirroring-considerations.html) for traffic mirroring
///
/// ## Example Usage
///
/// To create a basic traffic mirror session
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let eni = traffic_mirror_target::create(
///         "eni",
///         TrafficMirrorTargetArgs::builder()
///             .description("ENI target")
///             .network_interface_id("${test.primaryNetworkInterfaceId}")
///             .build_struct(),
///     );
///     let gwlb = traffic_mirror_target::create(
///         "gwlb",
///         TrafficMirrorTargetArgs::builder()
///             .description("GWLB target")
///             .gateway_load_balancer_endpoint_id("${example.id}")
///             .build_struct(),
///     );
///     let nlb = traffic_mirror_target::create(
///         "nlb",
///         TrafficMirrorTargetArgs::builder()
///             .description("NLB target")
///             .network_load_balancer_arn("${lb.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import traffic mirror targets using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/trafficMirrorTarget:TrafficMirrorTarget target tmt-0c13a005422b86606
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_mirror_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficMirrorTargetArgs {
        /// A description of the traffic mirror session.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The VPC Endpoint Id of the Gateway Load Balancer that is associated with the target.
        #[builder(into, default)]
        pub gateway_load_balancer_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The network interface ID that is associated with the target.
        #[builder(into, default)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.
        #[builder(into, default)]
        pub network_load_balancer_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// **NOTE:** Either `network_interface_id` or `network_load_balancer_arn` should be specified and both should not be specified together
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrafficMirrorTargetResult {
        /// The ARN of the traffic mirror target.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the traffic mirror session.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The VPC Endpoint Id of the Gateway Load Balancer that is associated with the target.
        pub gateway_load_balancer_endpoint_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The network interface ID that is associated with the target.
        pub network_interface_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.
        pub network_load_balancer_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the AWS account that owns the traffic mirror target.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// **NOTE:** Either `network_interface_id` or `network_load_balancer_arn` should be specified and both should not be specified together
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrafficMirrorTargetArgs,
    ) -> TrafficMirrorTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let gateway_load_balancer_endpoint_id_binding = args
            .gateway_load_balancer_endpoint_id
            .get_output(context);
        let network_interface_id_binding = args.network_interface_id.get_output(context);
        let network_load_balancer_arn_binding = args
            .network_load_balancer_arn
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/trafficMirrorTarget:TrafficMirrorTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayLoadBalancerEndpointId".into(),
                    value: &gateway_load_balancer_endpoint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkLoadBalancerArn".into(),
                    value: &network_load_balancer_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrafficMirrorTargetResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            gateway_load_balancer_endpoint_id: o
                .get_field("gatewayLoadBalancerEndpointId"),
            network_interface_id: o.get_field("networkInterfaceId"),
            network_load_balancer_arn: o.get_field("networkLoadBalancerArn"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
