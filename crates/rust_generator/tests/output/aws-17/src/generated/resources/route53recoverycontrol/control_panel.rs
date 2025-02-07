/// Provides an AWS Route 53 Recovery Control Config Control Panel.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = control_panel::create(
///         "example",
///         ControlPanelArgs::builder()
///             .cluster_arn(
///                 "arn:aws:route53-recovery-control::123456789012:cluster/8d47920e-d789-437d-803a-2dcc4b204393",
///             )
///             .name("balmorhea")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Control Config Control Panel using the control panel arn. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoverycontrol/controlPanel:ControlPanel mypanel arn:aws:route53-recovery-control::313517334327:controlpanel/1bfba17df8684f5dab0467b71424f7e8
/// ```
pub mod control_panel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ControlPanelArgs {
        /// ARN of the cluster in which this control panel will reside.
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name describing the control panel.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ControlPanelResult {
        /// ARN of the control panel.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the cluster in which this control panel will reside.
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether a control panel is default.
        pub default_control_panel: pulumi_gestalt_rust::Output<bool>,
        /// Name describing the control panel.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Number routing controls in a control panel.
        pub routing_control_count: pulumi_gestalt_rust::Output<i32>,
        /// Status of control panel: `PENDING` when it is being created/updated, `PENDING_DELETION` when it is being deleted, and `DEPLOYED` otherwise.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ControlPanelArgs,
    ) -> ControlPanelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53recoverycontrol/controlPanel:ControlPanel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ControlPanelResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cluster_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterArn"),
            ),
            default_control_panel: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultControlPanel"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            routing_control_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingControlCount"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
