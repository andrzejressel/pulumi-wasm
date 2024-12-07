//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = r_2_bucket::create(
//!         "example",
//!         R2BucketArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .location("enam")
//!             .name("terraform-bucket")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! > Available location values can be found in the [R2 documentation](https://developers.cloudflare.com/r2/reference/data-location/#available-hints).
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/r2Bucket:R2Bucket default <account id>/<bucket name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct R2BucketArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The location hint of the R2 bucket. Available values: `WNAM`, `ENAM`, `WEUR`, `EEUR`, `APAC`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub location: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the R2 bucket.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct R2BucketResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The location hint of the R2 bucket. Available values: `WNAM`, `ENAM`, `WEUR`, `EEUR`, `APAC`
    pub location: pulumi_wasm_rust::Output<String>,
    /// The name of the R2 bucket.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: R2BucketArgs) -> R2BucketResult {

    let result = crate::bindings::pulumi::cloudflare::r2_bucket::invoke(name, &crate::bindings::pulumi::cloudflare::r2_bucket::Args {
        account_id: &args.account_id.get_inner(),
        location: &args.location.get_inner(),
        name: &args.name.get_inner(),
    });

    R2BucketResult {
        account_id: crate::into_domain(result.account_id),
        location: crate::into_domain(result.location),
        name: crate::into_domain(result.name),
    }
}
