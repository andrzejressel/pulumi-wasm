/// Provides an IAM Security Token Service Preferences resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_token_service_preferences {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityTokenServicePreferencesArgs {
        /// The version of the STS global endpoint token. Valid values: `v1Token`, `v2Token`.
        #[builder(into)]
        pub global_endpoint_token_version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityTokenServicePreferencesResult {
        /// The version of the STS global endpoint token. Valid values: `v1Token`, `v2Token`.
        pub global_endpoint_token_version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityTokenServicePreferencesArgs,
    ) -> SecurityTokenServicePreferencesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let global_endpoint_token_version_binding = args
            .global_endpoint_token_version
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/securityTokenServicePreferences:SecurityTokenServicePreferences"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalEndpointTokenVersion".into(),
                    value: &global_endpoint_token_version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityTokenServicePreferencesResult {
            global_endpoint_token_version: o.get_field("globalEndpointTokenVersion"),
        }
    }
}
