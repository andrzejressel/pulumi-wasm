//! Provides a resource to manage an operation in API Shield Endpoint Management.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = api_shield_operation::create(
//!         "example",
//!         ApiShieldOperationArgs::builder()
//!             .endpoint("/path")
//!             .host("api.example.com")
//!             .method("GET")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldOperationArgs {
    /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub endpoint: pulumi_wasm_rust::Output<String>,
    /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub host: pulumi_wasm_rust::Output<String>,
    /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub method: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldOperationResult {
    /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
    pub endpoint: pulumi_wasm_rust::Output<String>,
    /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
    pub host: pulumi_wasm_rust::Output<String>,
    /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
    pub method: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldOperationArgs) -> ApiShieldOperationResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield_operation::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_operation::Args {
        endpoint: &args.endpoint.get_inner(),
        host: &args.host.get_inner(),
        method: &args.method.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ApiShieldOperationResult {
        endpoint: crate::into_domain(result.endpoint),
        host: crate::into_domain(result.host),
        method: crate::into_domain(result.method),
        zone_id: crate::into_domain(result.zone_id),
    }
}
