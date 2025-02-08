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
    pub fn invoke(context: &pulumi_gestalt_rust::PulumiContext) -> GetUserResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getUser:getUser".into(),
            version: super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserResult {
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
