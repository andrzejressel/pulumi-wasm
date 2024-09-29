//! Use this data source to lookup [Account Roles](https://api.cloudflare.com/#account-roles-properties).
//! 

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
