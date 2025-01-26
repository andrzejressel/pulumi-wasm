pub mod get_client_config {
    #[allow(dead_code)]
    pub struct GetClientConfigResult {
        /// The OAuth2 access token used by the client to authenticate against the Google Cloud API.
        pub access_token: pulumi_wasm_rust::Output<String>,
        /// The default labels configured on the provider.
        pub default_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project to apply any resources to.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region to operate under.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The zone to operate under.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_wasm_rust::PulumiContext) -> GetClientConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getClientConfig:getClientConfig".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClientConfigResult {
            access_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessToken"),
            ),
            default_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultLabels"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
