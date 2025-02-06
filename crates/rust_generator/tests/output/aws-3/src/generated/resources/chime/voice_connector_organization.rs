/// Enable origination settings to control inbound calling to your SIP infrastructure.
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
///             .name("test")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let defaultVoiceConnectorOrganization = voice_connector_organization::create(
///         "defaultVoiceConnectorOrganization",
///         VoiceConnectorOrganizationArgs::builder()
///             .disabled(false)
///             .routes(
///                 vec![
///                     VoiceConnectorOrganizationRoute::builder().host("127.0.0.1")
///                     .port(8081).priority(1).protocol("TCP").weight(1).build_struct(),
///                     VoiceConnectorOrganizationRoute::builder().host("127.0.0.2")
///                     .port(8082).priority(2).protocol("TCP").weight(10).build_struct(),
///                 ],
///             )
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chime Voice Connector Origination using the `voice_connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorOrganization:VoiceConnectorOrganization default abcdef1ghij2klmno3pqr4
/// ```
pub mod voice_connector_organization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorOrganizationArgs {
        /// When origination settings are disabled, inbound calls are not enabled for your Amazon Chime Voice Connector.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Set of call distribution properties defined for your SIP hosts. See route below for more details. Minimum of 1. Maximum of 20.
        #[builder(into)]
        pub routes: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::chime::VoiceConnectorOrganizationRoute>,
        >,
        /// The Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorOrganizationResult {
        /// When origination settings are disabled, inbound calls are not enabled for your Amazon Chime Voice Connector.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set of call distribution properties defined for your SIP hosts. See route below for more details. Minimum of 1. Maximum of 20.
        pub routes: pulumi_wasm_rust::Output<
            Vec<super::super::types::chime::VoiceConnectorOrganizationRoute>,
        >,
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
        args: VoiceConnectorOrganizationArgs,
    ) -> VoiceConnectorOrganizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let routes_binding = args.routes.get_output(context).get_inner();
        let voice_connector_id_binding = args
            .voice_connector_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorOrganization:VoiceConnectorOrganization"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "routes".into(),
                    value: &routes_binding,
                },
                register_interface::ObjectField {
                    name: "voiceConnectorId".into(),
                    value: &voice_connector_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VoiceConnectorOrganizationResult {
            disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(o.extract_field("routes")),
            voice_connector_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("voiceConnectorId"),
            ),
        }
    }
}
