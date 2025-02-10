#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_client_open_id_user_info {
    #[allow(dead_code)]
    pub struct GetClientOpenIdUserInfoResult {
        /// The email of the account used by the provider to authenticate with GCP.
        pub email: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
    ) -> GetClientOpenIdUserInfoResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:organizations/getClientOpenIdUserInfo:getClientOpenIdUserInfo"
                .into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetClientOpenIdUserInfoResult {
            email: o.get_field("email"),
            id: o.get_field("id"),
        }
    }
}
