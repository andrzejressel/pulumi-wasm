/// Provides an AWS Route 53 Recovery Control Config Control Panel.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ControlPanelArgs {
        /// ARN of the cluster in which this control panel will reside.
        #[builder(into)]
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// Name describing the control panel.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ControlPanelResult {
        /// ARN of the control panel.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the cluster in which this control panel will reside.
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// Whether a control panel is default.
        pub default_control_panel: pulumi_wasm_rust::Output<bool>,
        /// Name describing the control panel.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Number routing controls in a control panel.
        pub routing_control_count: pulumi_wasm_rust::Output<i32>,
        /// Status of control panel: `PENDING` when it is being created/updated, `PENDING_DELETION` when it is being deleted, and `DEPLOYED` otherwise.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ControlPanelArgs) -> ControlPanelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53recoverycontrol/controlPanel:ControlPanel".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clusterArn".into(),
                },
                register_interface::ResultField {
                    name: "defaultControlPanel".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "routingControlCount".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ControlPanelResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterArn").unwrap(),
            ),
            default_control_panel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultControlPanel").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            routing_control_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingControlCount").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
