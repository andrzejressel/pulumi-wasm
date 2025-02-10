#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user {
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// The user's email address.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The user's unique identifier.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The user's username.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetUserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getUser:getUser".into(),
            version: super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetUserResult {
            email: o.get_field("email"),
            id: o.get_field("id"),
            username: o.get_field("username"),
        }
    }
}
