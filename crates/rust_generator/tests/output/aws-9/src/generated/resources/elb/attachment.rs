/// Attaches an EC2 instance to an Elastic Load Balancer (ELB). For attaching resources with Application Load Balancer (ALB) or Network Load Balancer (NLB), see the `aws.lb.TargetGroupAttachment` resource.
///
/// > **NOTE on ELB Instances and ELB Attachments:** This provider currently provides
/// both a standalone ELB Attachment resource (describing an instance attached to
/// an ELB), and an Elastic Load Balancer resource with
/// `instances` defined in-line. At this time you cannot use an ELB with in-line
/// instances in conjunction with an ELB Attachment resource. Doing so will cause a
/// conflict and will overwrite attachments.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let baz = attachment::create(
///         "baz",
///         AttachmentArgs::builder().elb("${bar.id}").instance("${foo.id}").build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachmentArgs {
        /// The name of the ELB.
        #[builder(into)]
        pub elb: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Instance ID to place in the ELB pool.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AttachmentResult {
        /// The name of the ELB.
        pub elb: pulumi_gestalt_rust::Output<String>,
        /// Instance ID to place in the ELB pool.
        pub instance: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AttachmentArgs,
    ) -> AttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let elb_binding = args.elb.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elb/attachment:Attachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elb".into(),
                    value: elb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AttachmentResult {
            elb: o.get_field("elb"),
            instance: o.get_field("instance"),
        }
    }
}
