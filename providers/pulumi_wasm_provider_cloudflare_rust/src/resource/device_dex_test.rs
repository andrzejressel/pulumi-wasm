//! Provides a Cloudflare Device Dex Test resource. Device Dex Tests allow for building location-aware device settings policies.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = device_dex_test::create(
//!         "example",
//!         DeviceDexTestArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .data(
//!                 DeviceDexTestData::builder()
//!                     .host("https://example.com/home")
//!                     .kind("http")
//!                     .method("GET")
//!                     .build_struct(),
//!             )
//!             .description("Send a HTTP GET request to the home endpoint every half hour.")
//!             .enabled(true)
//!             .interval("0h30m0s")
//!             .name("GET homepage")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/deviceDexTest:DeviceDexTest example <account_id>/<device_dex_test_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct DeviceDexTestArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    #[builder(into)]
    pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
    /// Additional details about the test.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// Determines whether or not the test is active.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// How often the test will run.
    #[builder(into)]
    pub interval: pulumi_wasm_rust::Output<String>,
    /// The name of the Device Dex Test. Must be unique.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct DeviceDexTestResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the Dex Test was created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
    /// Additional details about the test.
    pub description: pulumi_wasm_rust::Output<String>,
    /// Determines whether or not the test is active.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// How often the test will run.
    pub interval: pulumi_wasm_rust::Output<String>,
    /// The name of the Device Dex Test. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the Dex Test was last updated.
    pub updated: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DeviceDexTestArgs) -> DeviceDexTestResult {

    let result = crate::bindings::pulumi::cloudflare::device_dex_test::invoke(name, &crate::bindings::pulumi::cloudflare::device_dex_test::Args {
        account_id: &args.account_id.get_inner(),
        data: &args.data.get_inner(),
        description: &args.description.get_inner(),
        enabled: &args.enabled.get_inner(),
        interval: &args.interval.get_inner(),
        name: &args.name.get_inner(),
    });

    DeviceDexTestResult {
        account_id: crate::into_domain(result.account_id),
        created: crate::into_domain(result.created),
        data: crate::into_domain(result.data),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        interval: crate::into_domain(result.interval),
        name: crate::into_domain(result.name),
        updated: crate::into_domain(result.updated),
    }
}

