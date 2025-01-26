/// Creates an Amazon Chime Voice Connector group under the administrator's AWS account. You can associate Amazon Chime Voice Connectors with the Amazon Chime Voice Connector group by including VoiceConnectorItems in the request.
///
/// You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let group = voice_connector_group::create(
///         "group",
///         VoiceConnectorGroupArgs::builder()
///             .connectors(
///                 vec![
///                     VoiceConnectorGroupConnector::builder().priority(1)
///                     .voiceConnectorId("${vc1.id}").build_struct(),
///                     VoiceConnectorGroupConnector::builder().priority(3)
///                     .voiceConnectorId("${vc2.id}").build_struct(),
///                 ],
///             )
///             .name("test-group")
///             .build_struct(),
///     );
///     let vc1 = voice_connector::create(
///         "vc1",
///         VoiceConnectorArgs::builder()
///             .aws_region("us-east-1")
///             .name("connector-test-1")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let vc2 = voice_connector::create(
///         "vc2",
///         VoiceConnectorArgs::builder()
///             .aws_region("us-west-2")
///             .name("connector-test-2")
///             .require_encryption(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Recorder using the name. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorGroup:VoiceConnectorGroup default example
/// ```
pub mod voice_connector_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorGroupArgs {
        /// The Amazon Chime Voice Connectors to route inbound calls to.
        #[builder(into, default)]
        pub connectors: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::chime::VoiceConnectorGroupConnector>>,
        >,
        /// The name of the Amazon Chime Voice Connector group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorGroupResult {
        /// The Amazon Chime Voice Connectors to route inbound calls to.
        pub connectors: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::chime::VoiceConnectorGroupConnector>>,
        >,
        /// The name of the Amazon Chime Voice Connector group.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VoiceConnectorGroupArgs,
    ) -> VoiceConnectorGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connectors_binding = args.connectors.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorGroup:VoiceConnectorGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectors".into(),
                    value: &connectors_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectors".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VoiceConnectorGroupResult {
            connectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectors").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
