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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalSettingsArgs {
        /// A list of resources along with the opt-in preferences for the account.
        #[builder(into)]
        pub global_settings: pulumi_wasm_rust::Output<
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
    pub fn create(name: &str, args: GlobalSettingsArgs) -> GlobalSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let global_settings_binding = args.global_settings.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "globalSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GlobalSettingsResult {
            global_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalSettings").unwrap(),
            ),
        }
    }
}
