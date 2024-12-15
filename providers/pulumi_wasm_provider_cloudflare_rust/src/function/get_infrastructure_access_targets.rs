//! Use this data source to retrieve all Infrastructure Access Targets.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_infrastructure_access_targets::invoke(
//!         GetInfrastructureAccessTargetsArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .hostname_contains("example")
//!             .ipv_4("198.51.100.1")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetInfrastructureAccessTargetsArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A date and time after a target was created to filter on.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub created_after: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostname of the target.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// Partial match to the hostname of a target
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hostname_contains: pulumi_wasm_rust::Output<Option<String>>,
    /// The target's IPv4 address.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ipv4: pulumi_wasm_rust::Output<Option<String>>,
    /// The target's IPv6 address.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ipv6: pulumi_wasm_rust::Output<Option<String>>,
    /// A date and time after a target was modified to filter on.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub modified_after: pulumi_wasm_rust::Output<Option<String>>,
    /// The private virtual network identifier for the target.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetInfrastructureAccessTargetsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A date and time after a target was created to filter on.
    pub created_after: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostname of the target.
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// Partial match to the hostname of a target
    pub hostname_contains: pulumi_wasm_rust::Output<Option<String>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The target's IPv4 address.
    pub ipv4: pulumi_wasm_rust::Output<Option<String>>,
    /// The target's IPv6 address.
    pub ipv6: pulumi_wasm_rust::Output<Option<String>>,
    /// A date and time after a target was modified to filter on.
    pub modified_after: pulumi_wasm_rust::Output<Option<String>>,
    pub targets: pulumi_wasm_rust::Output<Vec<crate::types::GetInfrastructureAccessTargetsTarget>>,
    /// The private virtual network identifier for the target.
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetInfrastructureAccessTargetsArgs
) -> GetInfrastructureAccessTargetsResult {

    let result = crate::bindings::pulumi::cloudflare::get_infrastructure_access_targets::invoke(
        &crate::bindings::pulumi::cloudflare::get_infrastructure_access_targets::Args {
                account_id: &args.account_id.get_inner(),
                created_after: &args.created_after.get_inner(),
                hostname: &args.hostname.get_inner(),
                hostname_contains: &args.hostname_contains.get_inner(),
                ipv4: &args.ipv4.get_inner(),
                ipv6: &args.ipv6.get_inner(),
                modified_after: &args.modified_after.get_inner(),
                virtual_network_id: &args.virtual_network_id.get_inner(),
        }
    );

    GetInfrastructureAccessTargetsResult {
        account_id: crate::into_domain(result.account_id),
        created_after: crate::into_domain(result.created_after),
        hostname: crate::into_domain(result.hostname),
        hostname_contains: crate::into_domain(result.hostname_contains),
        id: crate::into_domain(result.id),
        ipv4: crate::into_domain(result.ipv4),
        ipv6: crate::into_domain(result.ipv6),
        modified_after: crate::into_domain(result.modified_after),
        targets: crate::into_domain(result.targets),
        virtual_network_id: crate::into_domain(result.virtual_network_id),
    }
}
