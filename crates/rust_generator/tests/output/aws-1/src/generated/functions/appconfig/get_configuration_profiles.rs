#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration_profiles {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationProfilesArgs {
        /// ID of the AppConfig Application.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationProfilesResult {
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Set of Configuration Profile IDs associated with the AppConfig Application.
        pub configuration_profile_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationProfilesArgs,
    ) -> GetConfigurationProfilesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:appconfig/getConfigurationProfiles:getConfigurationProfiles"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConfigurationProfilesResult {
            application_id: o.get_field("applicationId"),
            configuration_profile_ids: o.get_field("configurationProfileIds"),
            id: o.get_field("id"),
        }
    }
}
