/// Provides an AWS Route 53 Recovery Control Config Safety Rule
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53recoverycontrol:SafetyRule
///     properties:
///       assertedControls:
///         - ${exampleAwsRoute53recoverycontrolconfigRoutingControl.arn}
///       controlPanelArn: arn:aws:route53-recovery-control::313517334327:controlpanel/abd5fbfc052d4844a082dbf400f61da8
///       name: daisyguttridge
///       waitPeriodMs: 5000
///       ruleConfig:
///         inverted: false
///         threshold: 1
///         type: ATLEAST
/// ```
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53recoverycontrol:SafetyRule
///     properties:
///       name: i_o
///       controlPanelArn: arn:aws:route53-recovery-control::313517334327:controlpanel/abd5fbfc052d4844a082dbf400f61da8
///       waitPeriodMs: 5000
///       gatingControls:
///         - ${exampleAwsRoute53recoverycontrolconfigRoutingControl.arn}
///       targetControls:
///         - ${exampleAwsRoute53recoverycontrolconfigRoutingControl.arn}
///       ruleConfig:
///         inverted: false
///         threshold: 1
///         type: ATLEAST
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Control Config Safety Rule using the safety rule ARN. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoverycontrol/safetyRule:SafetyRule myrule arn:aws:route53-recovery-control::313517334327:controlpanel/1bfba17df8684f5dab0467b71424f7e8/safetyrule/3bacc77003364c0f
/// ```
pub mod safety_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SafetyRuleArgs {
        /// Routing controls that are part of transactions that are evaluated to determine if a request to change a routing control state is allowed.
        #[builder(into, default)]
        pub asserted_controls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ARN of the control panel in which this safety rule will reside.
        #[builder(into)]
        pub control_panel_arn: pulumi_wasm_rust::Output<String>,
        /// Gating controls for the new gating rule. That is, routing controls that are evaluated by the rule configuration that you specify.
        #[builder(into, default)]
        pub gating_controls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name describing the safety rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for safety rule criteria. See below.
        #[builder(into)]
        pub rule_config: pulumi_wasm_rust::Output<
            super::super::types::route53recoverycontrol::SafetyRuleRuleConfig,
        >,
        /// Routing controls that can only be set or unset if the specified `rule_config` evaluates to true for the specified `gating_controls`.
        #[builder(into, default)]
        pub target_controls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Evaluation period, in milliseconds (ms), during which any request against the target routing controls will fail.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub wait_period_ms: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct SafetyRuleResult {
        /// ARN of the safety rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Routing controls that are part of transactions that are evaluated to determine if a request to change a routing control state is allowed.
        pub asserted_controls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ARN of the control panel in which this safety rule will reside.
        pub control_panel_arn: pulumi_wasm_rust::Output<String>,
        /// Gating controls for the new gating rule. That is, routing controls that are evaluated by the rule configuration that you specify.
        pub gating_controls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name describing the safety rule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for safety rule criteria. See below.
        pub rule_config: pulumi_wasm_rust::Output<
            super::super::types::route53recoverycontrol::SafetyRuleRuleConfig,
        >,
        /// Status of the safety rule. `PENDING` when it is being created/updated, `PENDING_DELETION` when it is being deleted, and `DEPLOYED` otherwise.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Routing controls that can only be set or unset if the specified `rule_config` evaluates to true for the specified `gating_controls`.
        pub target_controls: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Evaluation period, in milliseconds (ms), during which any request against the target routing controls will fail.
        ///
        /// The following arguments are optional:
        pub wait_period_ms: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SafetyRuleArgs) -> SafetyRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let asserted_controls_binding = args.asserted_controls.get_inner();
        let control_panel_arn_binding = args.control_panel_arn.get_inner();
        let gating_controls_binding = args.gating_controls.get_inner();
        let name_binding = args.name.get_inner();
        let rule_config_binding = args.rule_config.get_inner();
        let target_controls_binding = args.target_controls.get_inner();
        let wait_period_ms_binding = args.wait_period_ms.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53recoverycontrol/safetyRule:SafetyRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assertedControls".into(),
                    value: &asserted_controls_binding,
                },
                register_interface::ObjectField {
                    name: "controlPanelArn".into(),
                    value: &control_panel_arn_binding,
                },
                register_interface::ObjectField {
                    name: "gatingControls".into(),
                    value: &gating_controls_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ruleConfig".into(),
                    value: &rule_config_binding,
                },
                register_interface::ObjectField {
                    name: "targetControls".into(),
                    value: &target_controls_binding,
                },
                register_interface::ObjectField {
                    name: "waitPeriodMs".into(),
                    value: &wait_period_ms_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "assertedControls".into(),
                },
                register_interface::ResultField {
                    name: "controlPanelArn".into(),
                },
                register_interface::ResultField {
                    name: "gatingControls".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ruleConfig".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "targetControls".into(),
                },
                register_interface::ResultField {
                    name: "waitPeriodMs".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SafetyRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            asserted_controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assertedControls").unwrap(),
            ),
            control_panel_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPanelArn").unwrap(),
            ),
            gating_controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatingControls").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rule_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleConfig").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            target_controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetControls").unwrap(),
            ),
            wait_period_ms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitPeriodMs").unwrap(),
            ),
        }
    }
}
