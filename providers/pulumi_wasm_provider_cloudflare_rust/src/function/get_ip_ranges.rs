//! Use this data source to get the [IP ranges](https://www.cloudflare.com/ips/) of Cloudflare network.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   example:
//!     type: example:firewallResource
//!     properties:
//!       name: from-cloudflare
//!       network: default
//!       sourceRanges: ${cloudflare.ipv4CidrBlocks}
//!       allow:
//!         - ports: '443'
//!           protocol: tcp
//! variables:
//!   cloudflare:
//!     fn::invoke:
//!       Function: cloudflare:getIpRanges
//!       Arguments: {}
//! ```
//! <!--End PulumiCodeChooser -->


pub struct GetIpRangesResult {
    /// The lexically ordered list of only the IPv4 China CIDR blocks.
    pub china_ipv4_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The lexically ordered list of only the IPv6 China CIDR blocks.
    pub china_ipv6_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The lexically ordered list of all non-China CIDR blocks.
    pub cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The lexically ordered list of only the IPv4 CIDR blocks.
    pub ipv4_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
    /// The lexically ordered list of only the IPv6 CIDR blocks.
    pub ipv6_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
) -> GetIpRangesResult {

    let result = crate::bindings::pulumi::cloudflare::get_ip_ranges::invoke(
    );

    GetIpRangesResult {
        china_ipv4_cidr_blocks: crate::into_domain(result.china_ipv4_cidr_blocks),
        china_ipv6_cidr_blocks: crate::into_domain(result.china_ipv6_cidr_blocks),
        cidr_blocks: crate::into_domain(result.cidr_blocks),
        id: crate::into_domain(result.id),
        ipv4_cidr_blocks: crate::into_domain(result.ipv4_cidr_blocks),
        ipv6_cidr_blocks: crate::into_domain(result.ipv6_cidr_blocks),
    }
}
