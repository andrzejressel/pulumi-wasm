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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod safety_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SafetyRuleArgs {
        /// Routing controls that are part of transactions that are evaluated to determine if a request to change a routing control state is allowed.
        #[builder(into, default)]
        pub asserted_controls: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// ARN of the control panel in which this safety rule will reside.
        #[builder(into)]
        pub control_panel_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Gating controls for the new gating rule. That is, routing controls that are evaluated by the rule configuration that you specify.
        #[builder(into, default)]
        pub gating_controls: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name describing the safety rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for safety rule criteria. See below.
        #[builder(into)]
        pub rule_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::route53recoverycontrol::SafetyRuleRuleConfig,
        >,
        /// Routing controls that can only be set or unset if the specified `rule_config` evaluates to true for the specified `gating_controls`.
        #[builder(into, default)]
        pub target_controls: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Evaluation period, in milliseconds (ms), during which any request against the target routing controls will fail.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub wait_period_ms: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct SafetyRuleResult {
        /// ARN of the safety rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Routing controls that are part of transactions that are evaluated to determine if a request to change a routing control state is allowed.
        pub asserted_controls: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// ARN of the control panel in which this safety rule will reside.
        pub control_panel_arn: pulumi_gestalt_rust::Output<String>,
        /// Gating controls for the new gating rule. That is, routing controls that are evaluated by the rule configuration that you specify.
        pub gating_controls: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name describing the safety rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for safety rule criteria. See below.
        pub rule_config: pulumi_gestalt_rust::Output<
            super::super::types::route53recoverycontrol::SafetyRuleRuleConfig,
        >,
        /// Status of the safety rule. `PENDING` when it is being created/updated, `PENDING_DELETION` when it is being deleted, and `DEPLOYED` otherwise.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Routing controls that can only be set or unset if the specified `rule_config` evaluates to true for the specified `gating_controls`.
        pub target_controls: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Evaluation period, in milliseconds (ms), during which any request against the target routing controls will fail.
        ///
        /// The following arguments are optional:
        pub wait_period_ms: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SafetyRuleArgs,
    ) -> SafetyRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let asserted_controls_binding = args.asserted_controls.get_output(context);
        let control_panel_arn_binding = args.control_panel_arn.get_output(context);
        let gating_controls_binding = args.gating_controls.get_output(context);
        let name_binding = args.name.get_output(context);
        let rule_config_binding = args.rule_config.get_output(context);
        let target_controls_binding = args.target_controls.get_output(context);
        let wait_period_ms_binding = args.wait_period_ms.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53recoverycontrol/safetyRule:SafetyRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assertedControls".into(),
                    value: &asserted_controls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPanelArn".into(),
                    value: &control_panel_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatingControls".into(),
                    value: &gating_controls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleConfig".into(),
                    value: &rule_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetControls".into(),
                    value: &target_controls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitPeriodMs".into(),
                    value: &wait_period_ms_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SafetyRuleResult {
            arn: o.get_field("arn"),
            asserted_controls: o.get_field("assertedControls"),
            control_panel_arn: o.get_field("controlPanelArn"),
            gating_controls: o.get_field("gatingControls"),
            name: o.get_field("name"),
            rule_config: o.get_field("ruleConfig"),
            status: o.get_field("status"),
            target_controls: o.get_field("targetControls"),
            wait_period_ms: o.get_field("waitPeriodMs"),
        }
    }
}
