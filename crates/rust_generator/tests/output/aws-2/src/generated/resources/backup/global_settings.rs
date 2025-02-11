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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod global_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalSettingsArgs {
        /// A list of resources along with the opt-in preferences for the account.
        #[builder(into)]
        pub global_settings: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlobalSettingsResult {
        /// A list of resources along with the opt-in preferences for the account.
        pub global_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalSettingsArgs,
    ) -> GlobalSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let global_settings_binding = args.global_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/globalSettings:GlobalSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalSettings".into(),
                    value: &global_settings_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GlobalSettingsResult {
            global_settings: o.get_field("globalSettings"),
        }
    }
}
