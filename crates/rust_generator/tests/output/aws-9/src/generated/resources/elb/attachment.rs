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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AttachmentArgs,
    ) -> AttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let elb_binding = args.elb.get_output(context).get_inner();
        let instance_binding = args.instance.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elb/attachment:Attachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "elb".into(),
                    value: &elb_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AttachmentResult {
            elb: pulumi_gestalt_rust::__private::into_domain(o.extract_field("elb")),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
        }
    }
}
