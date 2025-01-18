/// Provides a Traffic mirror target.
/// Read [limits and considerations](https://docs.aws.amazon.com/vpc/latest/mirroring/traffic-mirroring-considerations.html) for traffic mirroring
///
/// ## Example Usage
///
/// To create a basic traffic mirror session
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod traffic_mirror_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficMirrorTargetArgs {
        /// A description of the traffic mirror session.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The VPC Endpoint Id of the Gateway Load Balancer that is associated with the target.
        #[builder(into, default)]
        pub gateway_load_balancer_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The network interface ID that is associated with the target.
        #[builder(into, default)]
        pub network_interface_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.
        #[builder(into, default)]
        pub network_load_balancer_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// **NOTE:** Either `network_interface_id` or `network_load_balancer_arn` should be specified and both should not be specified together
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrafficMirrorTargetResult {
        /// The ARN of the traffic mirror target.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description of the traffic mirror session.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The VPC Endpoint Id of the Gateway Load Balancer that is associated with the target.
        pub gateway_load_balancer_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The network interface ID that is associated with the target.
        pub network_interface_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.
        pub network_load_balancer_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AWS account that owns the traffic mirror target.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// **NOTE:** Either `network_interface_id` or `network_load_balancer_arn` should be specified and both should not be specified together
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TrafficMirrorTargetArgs,
    ) -> TrafficMirrorTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let gateway_load_balancer_endpoint_id_binding = args
            .gateway_load_balancer_endpoint_id
            .get_inner();
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let network_load_balancer_arn_binding = args
            .network_load_balancer_arn
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/trafficMirrorTarget:TrafficMirrorTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayLoadBalancerEndpointId".into(),
                    value: &gateway_load_balancer_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkLoadBalancerArn".into(),
                    value: &network_load_balancer_arn_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "gatewayLoadBalancerEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "networkLoadBalancerArn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrafficMirrorTargetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            gateway_load_balancer_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayLoadBalancerEndpointId").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            network_load_balancer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkLoadBalancerArn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
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
