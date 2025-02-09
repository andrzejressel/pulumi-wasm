#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_api_token_permission_groups {
    #[allow(dead_code)]
    pub struct GetApiTokenPermissionGroupsResult {
        /// Map of permissions for account level resources.
        pub account: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Checksum of permissions.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Map of all permissions available. Should not be used as some permissions will overlap resource scope. Instead, use resource level specific attributes.
        pub permissions: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of permissions for r2 level resources.
        pub r2: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Map of permissions for user level resources.
        pub user: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Map of permissions for zone level resources.
        pub zone: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
    ) -> GetApiTokenPermissionGroupsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getApiTokenPermissionGroups:getApiTokenPermissionGroups"
                .into(),
            version: super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetApiTokenPermissionGroupsResult {
            account: o.get_field("account"),
            id: o.get_field("id"),
            permissions: o.get_field("permissions"),
            r2: o.get_field("r2"),
            user: o.get_field("user"),
            zone: o.get_field("zone"),
        }
    }
}
