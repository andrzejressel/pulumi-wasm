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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityTokenServicePreferencesArgs {
        /// The version of the STS global endpoint token. Valid values: `v1Token`, `v2Token`.
        #[builder(into)]
        pub global_endpoint_token_version: pulumi_wasm_rust::InputOrOutput<String>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SecurityTokenServicePreferencesArgs,
    ) -> SecurityTokenServicePreferencesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let global_endpoint_token_version_binding = args
            .global_endpoint_token_version
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/securityTokenServicePreferences:SecurityTokenServicePreferences"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "globalEndpointTokenVersion".into(),
                    value: &global_endpoint_token_version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SecurityTokenServicePreferencesResult {
            global_endpoint_token_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("globalEndpointTokenVersion"),
            ),
        }
    }
}
