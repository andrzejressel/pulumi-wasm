#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_profiles_profiles {
    #[allow(dead_code)]
    pub struct GetProfilesProfilesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of Profiles.
        pub profiles: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::route53::GetProfilesProfilesProfile>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetProfilesProfilesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getProfilesProfiles:getProfilesProfiles".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetProfilesProfilesResult {
            id: o.get_field("id"),
            profiles: o.get_field("profiles"),
        }
    }
}
