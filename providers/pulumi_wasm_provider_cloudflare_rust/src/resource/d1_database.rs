//! The [D1 Database](https://developers.cloudflare.com/d1/) resource allows you to manage Cloudflare D1 databases.
//! 
//! !> When a D1 Database is replaced all the data is lost. Please ensure you have a
//!    backup of your data before replacing a D1 Database.
//! 
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = d_1_database::create(
//!         "example",
//!         D1DatabaseArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("terraform-database")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/d1Database:D1Database example <account id>/<database id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct D1DatabaseArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the D1 Database.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct D1DatabaseResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the D1 Database.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The backend version of the database.
    pub version: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: D1DatabaseArgs) -> D1DatabaseResult {

    let result = crate::bindings::pulumi::cloudflare::d1_database::invoke(name, &crate::bindings::pulumi::cloudflare::d1_database::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    D1DatabaseResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
        version: crate::into_domain(result.version),
    }
}
