/// Resource for managing Amazon Chime SDK Voice Global Settings.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = sdkvoice_global_settings::create(
///         "example",
///         SdkvoiceGlobalSettingsArgs::builder()
///             .voice_connector(
///                 SdkvoiceGlobalSettingsVoiceConnector::builder()
///                     .cdrBucket("example-bucket-name")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Chime SDK Voice Global Settings using the `id` (AWS account ID). For example:
///
/// ```sh
/// $ pulumi import aws:chime/sdkvoiceGlobalSettings:SdkvoiceGlobalSettings example 123456789012
/// ```
pub mod sdkvoice_global_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceGlobalSettingsArgs {
        /// The Voice Connector settings. See voice_connector.
        #[builder(into)]
        pub voice_connector: pulumi_wasm_rust::Output<
            super::super::types::chime::SdkvoiceGlobalSettingsVoiceConnector,
        >,
    }
    #[allow(dead_code)]
    pub struct SdkvoiceGlobalSettingsResult {
        /// The Voice Connector settings. See voice_connector.
        pub voice_connector: pulumi_wasm_rust::Output<
            super::super::types::chime::SdkvoiceGlobalSettingsVoiceConnector,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SdkvoiceGlobalSettingsArgs,
    ) -> SdkvoiceGlobalSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let voice_connector_binding = args.voice_connector.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceGlobalSettings:SdkvoiceGlobalSettings".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "voiceConnector".into(),
                    value: &voice_connector_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "voiceConnector".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SdkvoiceGlobalSettingsResult {
            voice_connector: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("voiceConnector").unwrap(),
            ),
        }
    }
}
