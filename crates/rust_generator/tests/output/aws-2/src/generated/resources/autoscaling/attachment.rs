/// Attaches a load balancer to an Auto Scaling group.
///
/// > **NOTE on Auto Scaling Groups, Attachments and Traffic Source Attachments:** Pulumi provides standalone Attachment (for attaching Classic Load Balancers and Application Load Balancer, Gateway Load Balancer, or Network Load Balancer target groups) and Traffic Source Attachment (for attaching Load Balancers and VPC Lattice target groups) resources and an Auto Scaling Group resource with `load_balancers`, `target_group_arns` and `traffic_source` attributes. Do not use the same traffic source in more than one of these resources. Doing so will cause a conflict of attachments. A `lifecycle` configuration block can be used to suppress differences if necessary.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = attachment::create(
///         "example",
///         AttachmentArgs::builder()
///             .autoscaling_group_name("${exampleAwsAutoscalingGroup.id}")
///             .elb("${exampleAwsElb.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = attachment::create(
///         "example",
///         AttachmentArgs::builder()
///             .autoscaling_group_name("${exampleAwsAutoscalingGroup.id}")
///             .lb_target_group_arn("${exampleAwsLbTargetGroup.arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachmentArgs {
        /// Name of ASG to associate with the ELB.
        #[builder(into)]
        pub autoscaling_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the ELB.
        #[builder(into, default)]
        pub elb: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of a load balancer target group.
        #[builder(into, default)]
        pub lb_target_group_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AttachmentResult {
        /// Name of ASG to associate with the ELB.
        pub autoscaling_group_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the ELB.
        pub elb: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of a load balancer target group.
        pub lb_target_group_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AttachmentArgs,
    ) -> AttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let autoscaling_group_name_binding = args
            .autoscaling_group_name
            .get_output(context)
            .get_inner();
        let elb_binding = args.elb.get_output(context).get_inner();
        let lb_target_group_arn_binding = args
            .lb_target_group_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscaling/attachment:Attachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingGroupName".into(),
                    value: &autoscaling_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "elb".into(),
                    value: &elb_binding,
                },
                register_interface::ObjectField {
                    name: "lbTargetGroupArn".into(),
                    value: &lb_target_group_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AttachmentResult {
            autoscaling_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoscalingGroupName"),
            ),
            elb: pulumi_gestalt_rust::__private::into_domain(o.extract_field("elb")),
            lb_target_group_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lbTargetGroupArn"),
            ),
        }
    }
}
