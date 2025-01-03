/// Provides an IAM Security Token Service Preferences resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_token_service_preferences::create(
///         "example",
///         SecurityTokenServicePreferencesArgs::builder()
///             .global_endpoint_token_version("v2Token")
///             .build_struct(),
///     );
/// }
/// ```
pub mod security_token_service_preferences {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityTokenServicePreferencesArgs {
        /// The version of the STS global endpoint token. Valid values: `v1Token`, `v2Token`.
        #[builder(into)]
        pub global_endpoint_token_version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityTokenServicePreferencesResult {
        /// The version of the STS global endpoint token. Valid values: `v1Token`, `v2Token`.
        pub global_endpoint_token_version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SecurityTokenServicePreferencesArgs,
    ) -> SecurityTokenServicePreferencesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let global_endpoint_token_version_binding = args
            .global_endpoint_token_version
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/securityTokenServicePreferences:SecurityTokenServicePreferences"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "globalEndpointTokenVersion".into(),
                    value: &global_endpoint_token_version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "globalEndpointTokenVersion".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecurityTokenServicePreferencesResult {
            global_endpoint_token_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalEndpointTokenVersion").unwrap(),
            ),
        }
    }
}
