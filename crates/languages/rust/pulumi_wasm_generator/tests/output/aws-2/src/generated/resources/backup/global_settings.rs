/// Provides an AWS Backup Global Settings resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:backup:GlobalSettings
///     properties:
///       globalSettings:
///         isCrossAccountBackupEnabled: 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Global Settings using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/globalSettings:GlobalSettings example 123456789012
/// ```
pub mod global_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalSettingsArgs {
        /// A list of resources along with the opt-in preferences for the account.
        #[builder(into)]
        pub global_settings: pulumi_wasm_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlobalSettingsResult {
        /// A list of resources along with the opt-in preferences for the account.
        pub global_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GlobalSettingsArgs,
    ) -> GlobalSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let global_settings_binding = args
            .global_settings
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/globalSettings:GlobalSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "globalSettings".into(),
                    value: &global_settings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GlobalSettingsResult {
            global_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("globalSettings"),
            ),
        }
    }
}
