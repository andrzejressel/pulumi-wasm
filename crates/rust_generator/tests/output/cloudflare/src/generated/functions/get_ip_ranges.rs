#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ip_ranges {
    #[allow(dead_code)]
    pub struct GetIpRangesResult {
        /// The lexically ordered list of only the IPv4 China CIDR blocks.
        pub china_ipv4_cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The lexically ordered list of only the IPv6 China CIDR blocks.
        pub china_ipv6_cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The lexically ordered list of all non-China CIDR blocks.
        pub cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The lexically ordered list of only the IPv4 CIDR blocks.
        pub ipv4_cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The lexically ordered list of only the IPv6 CIDR blocks.
        pub ipv6_cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::PulumiContext) -> GetIpRangesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getIpRanges:getIpRanges".into(),
            version: super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIpRangesResult {
            china_ipv4_cidr_blocks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("chinaIpv4CidrBlocks"),
            ),
            china_ipv6_cidr_blocks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("chinaIpv6CidrBlocks"),
            ),
            cidr_blocks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cidrBlocks"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ipv4_cidr_blocks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4CidrBlocks"),
            ),
            ipv6_cidr_blocks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6CidrBlocks"),
            ),
        }
    }
}
