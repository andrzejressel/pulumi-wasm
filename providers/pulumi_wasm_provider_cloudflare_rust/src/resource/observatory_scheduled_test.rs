//! Provides a Cloudflare Observatory Scheduled Test resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = observatory_scheduled_test::create(
//!         "example",
//!         ObservatoryScheduledTestArgs::builder()
//!             .frequency("WEEKLY")
//!             .region("us-central1")
//!             .url("example.com")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/observatoryScheduledTest:ObservatoryScheduledTest example <zone_id>:<url>:<region>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ObservatoryScheduledTestArgs {
    /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub frequency: pulumi_wasm_rust::Output<String>,
    /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub region: pulumi_wasm_rust::Output<String>,
    /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ObservatoryScheduledTestResult {
    /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
    pub frequency: pulumi_wasm_rust::Output<String>,
    /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
    pub region: pulumi_wasm_rust::Output<String>,
    /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ObservatoryScheduledTestArgs) -> ObservatoryScheduledTestResult {

    let result = crate::bindings::pulumi::cloudflare::observatory_scheduled_test::invoke(name, &crate::bindings::pulumi::cloudflare::observatory_scheduled_test::Args {
        frequency: &args.frequency.get_inner(),
        region: &args.region.get_inner(),
        url: &args.url.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ObservatoryScheduledTestResult {
        frequency: crate::into_domain(result.frequency),
        region: crate::into_domain(result.region),
        url: crate::into_domain(result.url),
        zone_id: crate::into_domain(result.zone_id),
    }
}
