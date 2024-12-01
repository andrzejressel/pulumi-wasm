//! Use this data source to retrieve information about the currently authenticated user.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ApiToken
//!     properties:
//!       name: Terraform Cloud (Terraform)
//!       policies:
//!         - permissionGroups:
//!             - ${all.user"User Details Read"[%!s(MISSING)]}
//!           resources:
//!             com.cloudflare.api.user.${me.id}: '*'
//! variables:
//!   me:
//!     fn::invoke:
//!       Function: cloudflare:getUser
//!       Arguments: {}
//!   all:
//!     fn::invoke:
//!       Function: cloudflare:getApiTokenPermissionGroups
//!       Arguments: {}
//! ```
//! <!--End PulumiCodeChooser -->


pub struct GetUserResult {
    /// The user's email address.
    pub email: pulumi_wasm_rust::Output<String>,
    /// The user's unique identifier.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The user's username.
    pub username: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
) -> GetUserResult {

    let result = crate::bindings::pulumi::cloudflare::get_user::invoke(
    );

    GetUserResult {
        email: crate::into_domain(result.email),
        id: crate::into_domain(result.id),
        username: crate::into_domain(result.username),
    }
}
