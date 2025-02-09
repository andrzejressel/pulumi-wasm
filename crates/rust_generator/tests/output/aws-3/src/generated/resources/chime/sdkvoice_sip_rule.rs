/// A SIP rule associates your SIP media application with a phone number or a Request URI hostname. You can associate a SIP rule with more than one SIP media application. Each application then runs only that rule.
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
///     let example = sdkvoice_sip_rule::create(
///         "example",
///         SdkvoiceSipRuleArgs::builder()
///             .name("example-sip-rule")
///             .target_applications(
///                 vec![
///                     SdkvoiceSipRuleTargetApplication::builder().awsRegion("us-east-1")
///                     .priority(1).sipMediaApplicationId("${[\"example-sma\"].id}")
///                     .build_struct(),
///                 ],
///             )
///             .trigger_type("RequestUriHostname")
///             .trigger_value("${[\"example-voice-connector\"].outboundHostName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a ChimeSDKVoice SIP Rule using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/sdkvoiceSipRule:SdkvoiceSipRule example abcdef123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sdkvoice_sip_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceSipRuleArgs {
        /// Enables or disables a rule. You must disable rules before you can delete them.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the SIP rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of SIP media applications with priority and AWS Region. Only one SIP application per AWS Region can be used. See `target_applications`.
        #[builder(into)]
        pub target_applications: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::chime::SdkvoiceSipRuleTargetApplication>,
        >,
        /// The type of trigger assigned to the SIP rule in `trigger_value`. Valid values are `RequestUriHostname` or `ToPhoneNumber`.
        #[builder(into)]
        pub trigger_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If `trigger_type` is `RequestUriHostname`, the value can be the outbound host name of an Amazon Chime Voice Connector. If `trigger_type` is `ToPhoneNumber`, the value can be a customer-owned phone number in the E164 format. The Sip Media Application specified in the Sip Rule is triggered if the request URI in an incoming SIP request matches the `RequestUriHostname`, or if the "To" header in the incoming SIP request matches the `ToPhoneNumber` value.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub trigger_value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SdkvoiceSipRuleResult {
        /// Enables or disables a rule. You must disable rules before you can delete them.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the SIP rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of SIP media applications with priority and AWS Region. Only one SIP application per AWS Region can be used. See `target_applications`.
        pub target_applications: pulumi_gestalt_rust::Output<
            Vec<super::super::types::chime::SdkvoiceSipRuleTargetApplication>,
        >,
        /// The type of trigger assigned to the SIP rule in `trigger_value`. Valid values are `RequestUriHostname` or `ToPhoneNumber`.
        pub trigger_type: pulumi_gestalt_rust::Output<String>,
        /// If `trigger_type` is `RequestUriHostname`, the value can be the outbound host name of an Amazon Chime Voice Connector. If `trigger_type` is `ToPhoneNumber`, the value can be a customer-owned phone number in the E164 format. The Sip Media Application specified in the Sip Rule is triggered if the request URI in an incoming SIP request matches the `RequestUriHostname`, or if the "To" header in the incoming SIP request matches the `ToPhoneNumber` value.
        ///
        /// The following arguments are optional:
        pub trigger_value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SdkvoiceSipRuleArgs,
    ) -> SdkvoiceSipRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let disabled_binding_1 = args.disabled.get_output(context);
        let disabled_binding = disabled_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let target_applications_binding_1 = args.target_applications.get_output(context);
        let target_applications_binding = target_applications_binding_1.get_inner();
        let trigger_type_binding_1 = args.trigger_type.get_output(context);
        let trigger_type_binding = trigger_type_binding_1.get_inner();
        let trigger_value_binding_1 = args.trigger_value.get_output(context);
        let trigger_value_binding = trigger_value_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceSipRule:SdkvoiceSipRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "targetApplications".into(),
                    value: &target_applications_binding,
                },
                register_interface::ObjectField {
                    name: "triggerType".into(),
                    value: &trigger_type_binding,
                },
                register_interface::ObjectField {
                    name: "triggerValue".into(),
                    value: &trigger_value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SdkvoiceSipRuleResult {
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            target_applications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetApplications"),
            ),
            trigger_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerType"),
            ),
            trigger_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerValue"),
            ),
        }
    }
}
