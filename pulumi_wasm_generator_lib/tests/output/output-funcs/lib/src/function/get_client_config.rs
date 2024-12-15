//! Failing example taken from azure-native. Original doc: Use this function to access the current configuration of the native Azure provider.


pub struct GetClientConfigResult {
    /// Azure Client ID (Application Object ID).
    pub client_id: pulumi_wasm_rust::Output<String>,
    /// Azure Object ID of the current user or service principal.
    pub object_id: pulumi_wasm_rust::Output<String>,
    /// Azure Subscription ID
    pub subscription_id: pulumi_wasm_rust::Output<String>,
    /// Azure Tenant ID
    pub tenant_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
) -> GetClientConfigResult {

    let result = crate::bindings::pulumi::mypkg::get_client_config::invoke(
    );

    GetClientConfigResult {
        client_id: crate::into_domain(result.client_id),
        object_id: crate::into_domain(result.object_id),
        subscription_id: crate::into_domain(result.subscription_id),
        tenant_id: crate::into_domain(result.tenant_id),
    }
}
