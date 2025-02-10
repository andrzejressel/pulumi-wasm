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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_source_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficSourceAttachmentArgs {
        /// The name of the Auto Scaling group.
        #[builder(into)]
        pub autoscaling_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique identifiers of a traffic sources.
        #[builder(into, default)]
        pub traffic_source: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::autoscaling::TrafficSourceAttachmentTrafficSource,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct TrafficSourceAttachmentResult {
        /// The name of the Auto Scaling group.
        pub autoscaling_group_name: pulumi_gestalt_rust::Output<String>,
        /// The unique identifiers of a traffic sources.
        pub traffic_source: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrafficSourceAttachmentArgs,
    ) -> TrafficSourceAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_group_name_binding = args
            .autoscaling_group_name
            .get_output(context);
        let traffic_source_binding = args.traffic_source.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:autoscaling/trafficSourceAttachment:TrafficSourceAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingGroupName".into(),
                    value: autoscaling_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficSource".into(),
                    value: traffic_source_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrafficSourceAttachmentResult {
            autoscaling_group_name: o.get_field("autoscalingGroupName"),
            traffic_source: o.get_field("trafficSource"),
        }
    }
}
