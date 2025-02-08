/// Allows the application of pre-defined controls to organizational units. For more information on usage, please see the
/// [AWS Control Tower User Guide](https://docs.aws.amazon.com/controltower/latest/userguide/enable-guardrails.html).
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import Control Tower Controls using their `organizational_unit_arn/control_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:controltower/controlTowerControl:ControlTowerControl example arn:aws:organizations::123456789101:ou/o-qqaejywet/ou-qg5o-ufbhdtv3,arn:aws:controltower:us-east-1::control/WTDSMKDKDNLE
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod control_tower_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ControlTowerControlArgs {
        /// The ARN of the control. Only Strongly recommended and Elective controls are permitted, with the exception of the Region deny guardrail.
        #[builder(into)]
        pub control_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Parameter values which are specified to configure the control when you enable it. See Parameters for more details.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::controltower::ControlTowerControlParameter>>,
        >,
        /// The ARN of the organizational unit.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ControlTowerControlResult {
        /// The ARN of the EnabledControl resource.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the control. Only Strongly recommended and Elective controls are permitted, with the exception of the Region deny guardrail.
        pub control_identifier: pulumi_gestalt_rust::Output<String>,
        /// Parameter values which are specified to configure the control when you enable it. See Parameters for more details.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::controltower::ControlTowerControlParameter>>,
        >,
        /// The ARN of the organizational unit.
        ///
        /// The following arguments are optional:
        pub target_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ControlTowerControlArgs,
    ) -> ControlTowerControlResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let control_identifier_binding = args
            .control_identifier
            .get_output(context)
            .get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let target_identifier_binding = args
            .target_identifier
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:controltower/controlTowerControl:ControlTowerControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "controlIdentifier".into(),
                    value: &control_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "targetIdentifier".into(),
                    value: &target_identifier_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ControlTowerControlResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            control_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("controlIdentifier"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            target_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetIdentifier"),
            ),
        }
    }
}
