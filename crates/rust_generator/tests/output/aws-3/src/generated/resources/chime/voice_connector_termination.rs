/// Enable Termination settings to control outbound calling from your SIP infrastructure.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod voice_connector_termination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationArgs {
        /// The countries to which calls are allowed, in ISO 3166-1 alpha-2 format.
        #[builder(into)]
        pub calling_regions: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The IP addresses allowed to make calls, in CIDR format.
        #[builder(into)]
        pub cidr_allow_lists: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The limit on calls per second. Max value based on account service quota. Default value of `1`.
        #[builder(into, default)]
        pub cps_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The default caller ID phone number.
        #[builder(into, default)]
        pub default_phone_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When termination settings are disabled, outbound calls can not be made.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorTerminationResult {
        /// The countries to which calls are allowed, in ISO 3166-1 alpha-2 format.
        pub calling_regions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The IP addresses allowed to make calls, in CIDR format.
        pub cidr_allow_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The limit on calls per second. Max value based on account service quota. Default value of `1`.
        pub cps_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The default caller ID phone number.
        pub default_phone_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// When termination settings are disabled, outbound calls can not be made.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VoiceConnectorTerminationArgs,
    ) -> VoiceConnectorTerminationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let calling_regions_binding = args.calling_regions.get_output(context);
        let cidr_allow_lists_binding = args.cidr_allow_lists.get_output(context);
        let cps_limit_binding = args.cps_limit.get_output(context);
        let default_phone_number_binding = args.default_phone_number.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let voice_connector_id_binding = args.voice_connector_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorTermination:VoiceConnectorTermination"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "callingRegions".into(),
                    value: calling_regions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrAllowLists".into(),
                    value: cidr_allow_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpsLimit".into(),
                    value: cps_limit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultPhoneNumber".into(),
                    value: default_phone_number_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceConnectorId".into(),
                    value: voice_connector_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VoiceConnectorTerminationResult {
            calling_regions: o.get_field("callingRegions"),
            cidr_allow_lists: o.get_field("cidrAllowLists"),
            cps_limit: o.get_field("cpsLimit"),
            default_phone_number: o.get_field("defaultPhoneNumber"),
            disabled: o.get_field("disabled"),
            voice_connector_id: o.get_field("voiceConnectorId"),
        }
    }
}
