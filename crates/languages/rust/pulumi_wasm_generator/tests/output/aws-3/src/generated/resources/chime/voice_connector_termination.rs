/// Enable Termination settings to control outbound calling from your SIP infrastructure.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = voice_connector::create(
///         "default",
///         VoiceConnectorArgs::builder()
///             .name("vc-name-test")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let defaultVoiceConnectorTermination = voice_connector_termination::create(
///         "defaultVoiceConnectorTermination",
///         VoiceConnectorTerminationArgs::builder()
///             .calling_regions(vec!["US", "CA",])
///             .cidr_allow_lists(vec!["50.35.78.96/31",])
///             .cps_limit(1)
///             .disabled(false)
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chime Voice Connector Termination using the `voice_connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorTermination:VoiceConnectorTermination default abcdef1ghij2klmno3pqr4
/// ```
pub mod voice_connector_termination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationArgs {
        /// The countries to which calls are allowed, in ISO 3166-1 alpha-2 format.
        #[builder(into)]
        pub calling_regions: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The IP addresses allowed to make calls, in CIDR format.
        #[builder(into)]
        pub cidr_allow_lists: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The limit on calls per second. Max value based on account service quota. Default value of `1`.
        #[builder(into, default)]
        pub cps_limit: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The default caller ID phone number.
        #[builder(into, default)]
        pub default_phone_number: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When termination settings are disabled, outbound calls can not be made.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationResult {
        /// The countries to which calls are allowed, in ISO 3166-1 alpha-2 format.
        pub calling_regions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The IP addresses allowed to make calls, in CIDR format.
        pub cidr_allow_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The limit on calls per second. Max value based on account service quota. Default value of `1`.
        pub cps_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// The default caller ID phone number.
        pub default_phone_number: pulumi_wasm_rust::Output<Option<String>>,
        /// When termination settings are disabled, outbound calls can not be made.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VoiceConnectorTerminationArgs,
    ) -> VoiceConnectorTerminationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let calling_regions_binding = args
            .calling_regions
            .get_output(context)
            .get_inner();
        let cidr_allow_lists_binding = args
            .cidr_allow_lists
            .get_output(context)
            .get_inner();
        let cps_limit_binding = args.cps_limit.get_output(context).get_inner();
        let default_phone_number_binding = args
            .default_phone_number
            .get_output(context)
            .get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let voice_connector_id_binding = args
            .voice_connector_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorTermination:VoiceConnectorTermination"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "callingRegions".into(),
                    value: &calling_regions_binding,
                },
                register_interface::ObjectField {
                    name: "cidrAllowLists".into(),
                    value: &cidr_allow_lists_binding,
                },
                register_interface::ObjectField {
                    name: "cpsLimit".into(),
                    value: &cps_limit_binding,
                },
                register_interface::ObjectField {
                    name: "defaultPhoneNumber".into(),
                    value: &default_phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "voiceConnectorId".into(),
                    value: &voice_connector_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VoiceConnectorTerminationResult {
            calling_regions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("callingRegions"),
            ),
            cidr_allow_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrAllowLists"),
            ),
            cps_limit: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cpsLimit"),
            ),
            default_phone_number: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultPhoneNumber"),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            voice_connector_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("voiceConnectorId"),
            ),
        }
    }
}
