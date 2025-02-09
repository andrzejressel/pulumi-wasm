/// Provides the ability to register instances and containers with an Application Load Balancer (ALB) or Network Load Balancer (NLB) target group. For attaching resources with Elastic Load Balancer (ELB), see the `aws.elb.Attachment` resource.
///
/// > **Note:** `aws.alb.TargetGroupAttachment` is known as `aws.lb.TargetGroupAttachment`. The functionality is identical.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = target_group_attachment::create(
///         "test",
///         TargetGroupAttachmentArgs::builder()
///             .port(80)
///             .target_group_arn("${testTargetGroup.arn}")
///             .target_id("${testInstance.id}")
///             .build_struct(),
///     );
///     let testInstance = instance::create(
///         "testInstance",
///         InstanceArgs::builder().build_struct(),
///     );
///     let testTargetGroup = target_group::create(
///         "testTargetGroup",
///         TargetGroupArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Lambda Target
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = target_group::create(
///         "test",
///         TargetGroupArgs::builder().name("test").target_type("lambda").build_struct(),
///     );
///     let testFunction = function::create(
///         "testFunction",
///         FunctionArgs::builder().build_struct(),
///     );
///     let testTargetGroupAttachment = target_group_attachment::create(
///         "testTargetGroupAttachment",
///         TargetGroupAttachmentArgs::builder()
///             .target_group_arn("${test.arn}")
///             .target_id("${testFunction.arn}")
///             .build_struct(),
///     );
///     let withLb = permission::create(
///         "withLb",
///         PermissionArgs::builder()
///             .action("lambda:InvokeFunction")
///             .function("${testFunction.name}")
///             .principal("elasticloadbalancing.amazonaws.com")
///             .source_arn("${test.arn}")
///             .statement_id("AllowExecutionFromlb")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Registering Multiple Targets
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance::create("example", InstanceArgs::builder().build_struct());
///     let exampleTargetGroup = target_group::create(
///         "exampleTargetGroup",
///         TargetGroupArgs::builder().build_struct(),
///     );
///     let exampleTargetGroupAttachment = target_group_attachment::create(
///         "exampleTargetGroupAttachment",
///         TargetGroupAttachmentArgs::builder()
///             .port(80)
///             .target_group_arn("${exampleTargetGroup.arn}")
///             .target_id("${range.value.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import Target Group Attachments.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_group_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentArgs {
        /// The Availability Zone where the IP address of the target is to be registered. If the private IP address is outside of the VPC scope, this value must be set to `all`.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port on which targets receive traffic.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ARN of the target group with which to register targets.
        #[builder(into)]
        pub target_group_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the target. This is the Instance ID for an instance, or the container ID for an ECS container. If the target type is `ip`, specify an IP address. If the target type is `lambda`, specify the Lambda function ARN. If the target type is `alb`, specify the ALB ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentResult {
        /// The Availability Zone where the IP address of the target is to be registered. If the private IP address is outside of the VPC scope, this value must be set to `all`.
        pub availability_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The port on which targets receive traffic.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ARN of the target group with which to register targets.
        pub target_group_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the target. This is the Instance ID for an instance, or the container ID for an ECS container. If the target type is `ip`, specify an IP address. If the target type is `lambda`, specify the Lambda function ARN. If the target type is `alb`, specify the ALB ARN.
        ///
        /// The following arguments are optional:
        pub target_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TargetGroupAttachmentArgs,
    ) -> TargetGroupAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let availability_zone_binding_1 = args.availability_zone.get_output(context);
        let availability_zone_binding = availability_zone_binding_1.get_inner();
        let port_binding_1 = args.port.get_output(context);
        let port_binding = port_binding_1.get_inner();
        let target_group_arn_binding_1 = args.target_group_arn.get_output(context);
        let target_group_arn_binding = target_group_arn_binding_1.get_inner();
        let target_id_binding_1 = args.target_id.get_output(context);
        let target_id_binding = target_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lb/targetGroupAttachment:TargetGroupAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "targetGroupArn".into(),
                    value: &target_group_arn_binding,
                },
                register_interface::ObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TargetGroupAttachmentResult {
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            target_group_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetGroupArn"),
            ),
            target_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetId"),
            ),
        }
    }
}
