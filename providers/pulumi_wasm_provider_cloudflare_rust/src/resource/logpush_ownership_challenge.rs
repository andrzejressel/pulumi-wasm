//! Provides a resource which manages Cloudflare Logpush ownership
//! challenges to use in a Logpush Job. On it's own, doesn't do much
//! however this resource should be used in conjunction to create
//! Logpush jobs.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = logpush_ownership_challenge::create(
//!         "example",
//!         LogpushOwnershipChallengeArgs::builder()
//!             .destination_conf("s3://my-bucket-path?region=us-west-2")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct LogpushOwnershipChallengeArgs {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#destination). **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct LogpushOwnershipChallengeResult {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#destination). **Modifying this attribute will force creation of a new resource.**
    pub destination_conf: pulumi_wasm_rust::Output<String>,
    /// The filename of the ownership challenge which	contains the contents required for Logpush Job creation.
    pub ownership_challenge_filename: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LogpushOwnershipChallengeArgs) -> LogpushOwnershipChallengeResult {

    let result = crate::bindings::pulumi::cloudflare::logpush_ownership_challenge::invoke(name, &crate::bindings::pulumi::cloudflare::logpush_ownership_challenge::Args {
        account_id: &args.account_id.get_inner(),
        destination_conf: &args.destination_conf.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    LogpushOwnershipChallengeResult {
        account_id: crate::into_domain(result.account_id),
        destination_conf: crate::into_domain(result.destination_conf),
        ownership_challenge_filename: crate::into_domain(result.ownership_challenge_filename),
        zone_id: crate::into_domain(result.zone_id),
    }
}

