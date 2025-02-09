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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetConfigurationProfilesArgs,
    ) -> GetConfigurationProfilesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_id_binding_1 = args.application_id.get_output(context);
        let application_id_binding = application_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:appconfig/getConfigurationProfiles:getConfigurationProfiles"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConfigurationProfilesResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            configuration_profile_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationProfileIds"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
