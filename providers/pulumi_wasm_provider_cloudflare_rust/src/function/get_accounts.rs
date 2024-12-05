//! Data source for looking up Cloudflare Accounts.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_accounts::invoke(
//!         GetAccountsArgs::builder().name("example account").build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetAccountsArgs {
    /// The account name to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetAccountsResult {
    pub accounts: pulumi_wasm_rust::Output<Vec<crate::types::GetAccountsAccount>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The account name to target for the resource.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetAccountsArgs
) -> GetAccountsResult {

    let result = crate::bindings::pulumi::cloudflare::get_accounts::invoke(
        &crate::bindings::pulumi::cloudflare::get_accounts::Args {
                name: &args.name.get_inner(),
        }
    );

    GetAccountsResult {
        accounts: crate::into_domain(result.accounts),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
    }
}
