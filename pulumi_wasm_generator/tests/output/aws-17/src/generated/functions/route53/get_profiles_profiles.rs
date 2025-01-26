pub mod get_profiles_profiles {
    #[allow(dead_code)]
    pub struct GetProfilesProfilesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of Profiles.
        pub profiles: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::route53::GetProfilesProfilesProfile>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
    ) -> GetProfilesProfilesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getProfilesProfiles:getProfilesProfiles".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProfilesProfilesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            profiles: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("profiles"),
            ),
        }
    }
}
