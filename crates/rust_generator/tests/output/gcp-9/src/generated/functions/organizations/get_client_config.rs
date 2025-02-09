#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_client_config {
    #[allow(dead_code)]
    pub struct GetClientConfigResult {
        /// The OAuth2 access token used by the client to authenticate against the Google Cloud API.
        pub access_token: pulumi_gestalt_rust::Output<String>,
        /// The default labels configured on the provider.
        pub default_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project to apply any resources to.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region to operate under.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The zone to operate under.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetClientConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:organizations/getClientConfig:getClientConfig".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetClientConfigResult {
            access_token: o.get_field("accessToken"),
            default_labels: o.get_field("defaultLabels"),
            id: o.get_field("id"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            zone: o.get_field("zone"),
        }
    }
}
