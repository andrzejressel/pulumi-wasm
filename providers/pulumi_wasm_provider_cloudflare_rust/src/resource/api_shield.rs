//! Provides a resource to manage API Shield configurations.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = api_shield::create(
//!         "example",
//!         ApiShieldArgs::builder()
//!             .auth_id_characteristics(
//!                 vec![
//!                     ApiShieldAuthIdCharacteristic::builder().name("my-example-header").
//!                     type ("header").build_struct(),
//!                 ],
//!             )
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldArgs {
    /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
    #[builder(into, default)]
    pub auth_id_characteristics: pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldResult {
    /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
    pub auth_id_characteristics: pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldArgs) -> ApiShieldResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield::Args {
        auth_id_characteristics: &args.auth_id_characteristics.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ApiShieldResult {
        auth_id_characteristics: crate::into_domain(result.auth_id_characteristics),
        zone_id: crate::into_domain(result.zone_id),
    }
}
