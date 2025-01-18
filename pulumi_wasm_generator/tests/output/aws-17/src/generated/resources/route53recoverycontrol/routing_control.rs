/// Provides an AWS Route 53 Recovery Control Config Routing Control.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = routing_control::create(
///         "example",
///         RoutingControlArgs::builder()
///             .cluster_arn(
///                 "arn:aws:route53-recovery-control::881188118811:cluster/8d47920e-d789-437d-803a-2dcc4b204393",
///             )
///             .name("tinlicker")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = routing_control::create(
///         "example",
///         RoutingControlArgs::builder()
///             .cluster_arn(
///                 "arn:aws:route53-recovery-control::881188118811:cluster/8d47920e-d789-437d-803a-2dcc4b204393",
///             )
///             .control_panel_arn(
///                 "arn:aws:route53-recovery-control::428113431245:controlpanel/abd5fbfc052d4844a082dbf400f61da8",
///             )
///             .name("thomasoliver")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Control Config Routing Control using the routing control arn. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoverycontrol/routingControl:RoutingControl mycontrol arn:aws:route53-recovery-control::313517334327:controlpanel/abd5fbfc052d4844a082dbf400f61da8/routingcontrol/d5d90e587870494b
/// ```
pub mod routing_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoutingControlArgs {
        /// ARN of the cluster in which this routing control will reside.
        #[builder(into)]
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the control panel in which this routing control will reside.
        #[builder(into, default)]
        pub control_panel_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name describing the routing control.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RoutingControlResult {
        /// ARN of the routing control.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the cluster in which this routing control will reside.
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the control panel in which this routing control will reside.
        pub control_panel_arn: pulumi_wasm_rust::Output<String>,
        /// The name describing the routing control.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Status of routing control. `PENDING` when it is being created/updated, `PENDING_DELETION` when it is being deleted, and `DEPLOYED` otherwise.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RoutingControlArgs) -> RoutingControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_inner();
        let control_panel_arn_binding = args.control_panel_arn.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53recoverycontrol/routingControl:RoutingControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding,
                },
                register_interface::ObjectField {
                    name: "controlPanelArn".into(),
                    value: &control_panel_arn_binding,
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
                    name: "controlPanelArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        RoutingControlResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterArn").unwrap(),
            ),
            control_panel_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPanelArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
