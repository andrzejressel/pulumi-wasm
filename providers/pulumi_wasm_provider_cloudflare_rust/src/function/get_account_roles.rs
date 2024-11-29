//! Use this data source to lookup [Account Roles](https://api.cloudflare.com/#account-roles-properties).
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const accountRoles = cloudflare.getAccountRoles({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//! });
//! const rolesByName = accountRoles.then(accountRoles => .reduce((__obj, role) => ({ ...__obj, [role.name]: role })));
//! const member = new cloudflare.AccountMember("member", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     emailAddress: "user@example.com",
//!     roleIds: [rolesByName.Administrator?.id],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! account_roles = cloudflare.get_account_roles(account_id="f037e56e89293a057740de681ac9abbe")
//! roles_by_name = {role.name: role for role in account_roles.roles}
//! member = cloudflare.AccountMember("member",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     email_address="user@example.com",
//!     role_ids=[roles_by_name["Administrator"]])
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var accountRoles = Cloudflare.GetAccountRoles.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!     });
//! 
//!     var rolesByName = ;
//! 
//!     var member = new Cloudflare.AccountMember("member", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         EmailAddress = "user@example.com",
//!         RoleIds = new[]
//!         {
//!             rolesByName.Apply(rolesByName => rolesByName.Administrator.Id),
//!         },
//!     });
//! 
//! });
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetAccountRolesArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetAccountRolesResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of roles object.
    pub roles: pulumi_wasm_rust::Output<Vec<crate::types::GetAccountRolesRole>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetAccountRolesArgs
) -> GetAccountRolesResult {

    let result = crate::bindings::pulumi::cloudflare::get_account_roles::invoke(
        &crate::bindings::pulumi::cloudflare::get_account_roles::Args {
                account_id: &args.account_id.get_inner(),
        }
    );

    GetAccountRolesResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        roles: crate::into_domain(result.roles),
    }
}
