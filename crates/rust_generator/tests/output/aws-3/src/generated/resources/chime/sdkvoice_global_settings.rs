/// Resource for managing Amazon Chime SDK Voice Global Settings.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sdkvoice_global_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SdkvoiceGlobalSettingsArgs {
        /// The Voice Connector settings. See voice_connector.
        #[builder(into)]
        pub voice_connector: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::chime::SdkvoiceGlobalSettingsVoiceConnector,
        >,
    }
    #[allow(dead_code)]
    pub struct SdkvoiceGlobalSettingsResult {
        /// The Voice Connector settings. See voice_connector.
        pub voice_connector: pulumi_gestalt_rust::Output<
            super::super::types::chime::SdkvoiceGlobalSettingsVoiceConnector,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SdkvoiceGlobalSettingsArgs,
    ) -> SdkvoiceGlobalSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let voice_connector_binding_1 = args.voice_connector.get_output(context);
        let voice_connector_binding = voice_connector_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:chime/sdkvoiceGlobalSettings:SdkvoiceGlobalSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "voiceConnector".into(),
                    value: &voice_connector_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SdkvoiceGlobalSettingsResult {
            voice_connector: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("voiceConnector"),
            ),
        }
    }
}
