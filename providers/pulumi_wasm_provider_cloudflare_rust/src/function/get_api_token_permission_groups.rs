//! Use this data source to look up [API Token Permission Groups](https://developers.cloudflare.com/api/tokens/create/permissions).
//! Commonly used as references within [`cloudflare_token`](https://www.terraform.io/docs/providers/cloudflare/r/api_token.html) resources.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let all = get_api_token_permission_groups::invoke(
//!         GetApiTokenPermissionGroupsArgs::builder().build_struct(),
//!     );
//! }
//! ```


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
pub fn invoke(
) -> GetApiTokenPermissionGroupsResult {

    let result = crate::bindings::pulumi::cloudflare::get_api_token_permission_groups::invoke(
    );

    GetApiTokenPermissionGroupsResult {
        account: crate::into_domain(result.account),
        id: crate::into_domain(result.id),
        permissions: crate::into_domain(result.permissions),
        r2: crate::into_domain(result.r2),
        user: crate::into_domain(result.user),
        zone: crate::into_domain(result.zone),
    }
}
