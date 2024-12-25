pub struct GetApiTokenPermissionGroupsResult {
    /// Map of permissions for account level resources.
    pub account: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Checksum of permissions.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Map of all permissions available. Should not be used as some permissions will overlap resource scope. Instead, use resource level specific attributes.
    pub permissions: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
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
#[allow(non_snake_case, unused_imports)]
pub fn invoke() -> GetApiTokenPermissionGroupsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getApiTokenPermissionGroups:getApiTokenPermissionGroups"
            .into(),
        object: Vec::from([]),
        results: vec![
            register_interface::ResultField { name : "account".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "permissions".into() },
            register_interface::ResultField { name : "r2".into() },
            register_interface::ResultField { name : "user".into() },
            register_interface::ResultField { name : "zone".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetApiTokenPermissionGroupsResult {
        account: into_domain(hashmap.remove("account").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        permissions: into_domain(hashmap.remove("permissions").unwrap()),
        r2: into_domain(hashmap.remove("r2").unwrap()),
        user: into_domain(hashmap.remove("user").unwrap()),
        zone: into_domain(hashmap.remove("zone").unwrap()),
    }
}
