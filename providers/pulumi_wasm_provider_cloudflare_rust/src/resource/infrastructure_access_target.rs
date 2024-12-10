//! The [Infrastructure Access Target](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/use-cases/ssh/ssh-infrastructure-access/#4-add-a-target) resource allows you to configure Infrastructure Access Targets for an account.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = infrastructure_access_target::create(
//!         "example",
//!         InfrastructureAccessTargetArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .hostname("example-target")
//!             .ip(
//!                 InfrastructureAccessTargetIp::builder()
//!                     .ipv4(
//!                         InfrastructureAccessTargetIpIpv4::builder()
//!                             .ipAddr("198.51.100.1")
//!                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                             .build_struct(),
//!                     )
//!                     .ipv6(
//!                         InfrastructureAccessTargetIpIpv6::builder()
//!                             .ipAddr("2001:db8::")
//!                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                             .build_struct(),
//!                     )
//!                     .build_struct(),
//!             )
//!             .build_struct(),
//!     );
//!     let ipv4OnlyExample = infrastructure_access_target::create(
//!         "ipv4OnlyExample",
//!         InfrastructureAccessTargetArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .hostname("example-ipv4-only")
//!             .ip(
//!                 InfrastructureAccessTargetIp::builder()
//!                     .ipv4(
//!                         InfrastructureAccessTargetIpIpv4::builder()
//!                             .ipAddr("198.51.100.1")
//!                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
//!                             .build_struct(),
//!                     )
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
//! $ pulumi import cloudflare:index/infrastructureAccessTarget:InfrastructureAccessTarget example <account_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct InfrastructureAccessTargetArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    #[builder(into)]
    pub ip: pulumi_wasm_rust::Output<crate::types::InfrastructureAccessTargetIp>,
}

pub struct InfrastructureAccessTargetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The date and time at which the target was created.
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    pub ip: pulumi_wasm_rust::Output<crate::types::InfrastructureAccessTargetIp>,
    /// The date and time at which the target was last modified.
    pub modified_at: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: InfrastructureAccessTargetArgs) -> InfrastructureAccessTargetResult {

    let result = crate::bindings::pulumi::cloudflare::infrastructure_access_target::invoke(name, &crate::bindings::pulumi::cloudflare::infrastructure_access_target::Args {
        account_id: &args.account_id.get_inner(),
        hostname: &args.hostname.get_inner(),
        ip: &args.ip.get_inner(),
    });

    InfrastructureAccessTargetResult {
        account_id: crate::into_domain(result.account_id),
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        ip: crate::into_domain(result.ip),
        modified_at: crate::into_domain(result.modified_at),
    }
}

