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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let baz = attachment::create(
///         "baz",
///         AttachmentArgs::builder().elb("${bar.id}").instance("${foo.id}").build_struct(),
///     );
/// }
/// ```
pub mod attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachmentArgs {
        /// The name of the ELB.
        #[builder(into)]
        pub elb: pulumi_wasm_rust::Output<String>,
        /// Instance ID to place in the ELB pool.
        #[builder(into)]
        pub instance: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AttachmentResult {
        /// The name of the ELB.
        pub elb: pulumi_wasm_rust::Output<String>,
        /// Instance ID to place in the ELB pool.
        pub instance: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AttachmentArgs) -> AttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let elb_binding = args.elb.get_inner();
        let instance_binding = args.instance.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elb/attachment:Attachment".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "elb".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AttachmentResult {
            elb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elb").unwrap(),
            ),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
        }
    }
}