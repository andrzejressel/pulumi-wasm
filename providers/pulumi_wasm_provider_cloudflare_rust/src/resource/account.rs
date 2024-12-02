//! Provides a Cloudflare Account resource. Account is the basic resource for
//! working with Cloudflare zones, teams and users.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = account::create(
//!         "example",
//!         AccountArgs::builder()
//!             .enforce_twofactor(true)
//!             .name("some-enterprise-account")
//!             .type_("enterprise")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/account:Account example <account_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccountArgs {
    /// Whether 2FA is enforced on the account. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the account that is displayed in the Cloudflare dashboard.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccountResult {
    /// Whether 2FA is enforced on the account. Defaults to `false`.
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the account that is displayed in the Cloudflare dashboard.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccountArgs) -> AccountResult {

    let result = crate::bindings::pulumi::cloudflare::account::invoke(name, &crate::bindings::pulumi::cloudflare::account::Args {
        enforce_twofactor: &args.enforce_twofactor.get_inner(),
        name: &args.name.get_inner(),
        type_: &args.type_.get_inner(),
    });

    AccountResult {
        enforce_twofactor: crate::into_domain(result.enforce_twofactor),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
