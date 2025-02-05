pub mod get_api_token_permission_groups {
    #[allow(dead_code)]
    pub struct GetApiTokenPermissionGroupsResult {
        /// Map of permissions for account level resources.
        pub account: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Checksum of permissions.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Map of all permissions available. Should not be used as some permissions will overlap resource scope. Instead, use resource level specific attributes.
        pub permissions: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of permissions for r2 level resources.
        pub r2: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Map of permissions for user level resources.
        pub user: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Map of permissions for zone level resources.
        pub zone: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
    ) -> GetApiTokenPermissionGroupsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getApiTokenPermissionGroups:getApiTokenPermissionGroups"
                .into(),
            version: super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetApiTokenPermissionGroupsResult {
            account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("account"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            r2: pulumi_wasm_rust::__private::into_domain(o.extract_field("r2")),
            user: pulumi_wasm_rust::__private::into_domain(o.extract_field("user")),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
