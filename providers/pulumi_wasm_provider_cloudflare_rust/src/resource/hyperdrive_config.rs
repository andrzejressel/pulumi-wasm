//! The [Hyperdrive Config](https://developers.cloudflare.com/hyperdrive/) resource allows you to manage Cloudflare Hyperdrive Configs.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let noDefaults = hyperdrive_config::create(
//!         "noDefaults",
//!         HyperdriveConfigArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("my-hyperdrive-config")
//!             .origin(
//!                 HyperdriveConfigOrigin::builder()
//!                     .database("postgres")
//!                     .host("my-database.example.com")
//!                     .password("my-password")
//!                     .port(5432)
//!                     .scheme("postgres")
//!                     .user("my-user")
//!                     .build_struct(),
//!             )
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/hyperdriveConfig:HyperdriveConfig example <account_id>/<hyperdrive_config_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct HyperdriveConfigArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The caching details for the Hyperdrive configuration.
    #[builder(into, default)]
    pub caching: pulumi_wasm_rust::Output<Option<crate::types::HyperdriveConfigCaching>>,
    /// The name of the Hyperdrive configuration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The origin details for the Hyperdrive configuration.
    #[builder(into)]
    pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
    /// The identifier of this resource. This is the hyperdrive config value.
    #[builder(into, default)]
    pub resource_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct HyperdriveConfigResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The caching details for the Hyperdrive configuration.
    pub caching: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigCaching>,
    /// The name of the Hyperdrive configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The origin details for the Hyperdrive configuration.
    pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
    /// The identifier of this resource. This is the hyperdrive config value.
    pub resource_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: HyperdriveConfigArgs
) -> HyperdriveConfigResult {

    let result = crate::bindings::pulumi::cloudflare::hyperdrive_config::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::hyperdrive_config::Args {
                account_id: &args.account_id.get_inner(),
                caching: &args.caching.get_inner(),
                name: &args.name.get_inner(),
                origin: &args.origin.get_inner(),
                resource_id: &args.resource_id.get_inner(),
        }
    );

    HyperdriveConfigResult {
        account_id: crate::into_domain(result.account_id),
        caching: crate::into_domain(result.caching),
        name: crate::into_domain(result.name),
        origin: crate::into_domain(result.origin),
        resource_id: crate::into_domain(result.resource_id),
    }
}
