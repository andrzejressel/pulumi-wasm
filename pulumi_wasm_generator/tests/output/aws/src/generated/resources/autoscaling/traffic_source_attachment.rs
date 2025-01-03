/// Attaches a traffic source to an Auto Scaling group.
///
/// > **NOTE on Auto Scaling Groups, Attachments and Traffic Source Attachments:** Pulumi provides standalone Attachment (for attaching Classic Load Balancers and Application Load Balancer, Gateway Load Balancer, or Network Load Balancer target groups) and Traffic Source Attachment (for attaching Load Balancers and VPC Lattice target groups) resources and an Auto Scaling Group resource with `load_balancers`, `target_group_arns` and `traffic_source` attributes. Do not use the same traffic source in more than one of these resources. Doing so will cause a conflict of attachments. A `lifecycle` configuration block can be used to suppress differences if necessary.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:autoscaling:TrafficSourceAttachment
///     properties:
///       autoscalingGroupName: ${exampleAwsAutoscalingGroup.id}
///       trafficSource:
///         identifier: ${exampleAwsLbTargetGroup.arn}
///         type: elbv2
/// ```
pub mod traffic_source_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficSourceAttachmentArgs {
        /// The name of the Auto Scaling group.
        #[builder(into)]
        pub autoscaling_group_name: pulumi_wasm_rust::Output<String>,
        /// The unique identifiers of a traffic sources.
        #[builder(into, default)]
        pub traffic_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::autoscaling::TrafficSourceAttachmentTrafficSource,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct TrafficSourceAttachmentResult {
        /// The name of the Auto Scaling group.
        pub autoscaling_group_name: pulumi_wasm_rust::Output<String>,
        /// The unique identifiers of a traffic sources.
        pub traffic_source: pulumi_wasm_rust::Output<
            Option<
                super::super::types::autoscaling::TrafficSourceAttachmentTrafficSource,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TrafficSourceAttachmentArgs,
    ) -> TrafficSourceAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscaling_group_name_binding = args.autoscaling_group_name.get_inner();
        let traffic_source_binding = args.traffic_source.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscaling/trafficSourceAttachment:TrafficSourceAttachment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingGroupName".into(),
                    value: &autoscaling_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "trafficSource".into(),
                    value: &traffic_source_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoscalingGroupName".into(),
                },
                register_interface::ResultField {
                    name: "trafficSource".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrafficSourceAttachmentResult {
            autoscaling_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingGroupName").unwrap(),
            ),
            traffic_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficSource").unwrap(),
            ),
        }
    }
}
