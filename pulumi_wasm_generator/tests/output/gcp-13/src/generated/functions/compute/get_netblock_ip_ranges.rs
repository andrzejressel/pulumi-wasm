pub mod get_netblock_ip_ranges {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetblockIpRangesArgs {
        /// The type of range for which to provide results.
        ///
        /// Defaults to `cloud-netblocks`. The following `range_type`s are supported:
        ///
        /// * `cloud-netblocks` - Corresponds to the IP addresses used for resources on Google Cloud Platform. [More details.](https://cloud.google.com/compute/docs/faq#where_can_i_find_product_name_short_ip_ranges)
        ///
        /// * `google-netblocks` - Corresponds to IP addresses used for Google services. [More details.](https://cloud.google.com/compute/docs/faq#where_can_i_find_product_name_short_ip_ranges)
        ///
        /// * `restricted-googleapis` - Corresponds to the IP addresses used for Private Google Access only for services that support VPC Service Controls API access. [More details.](https://cloud.google.com/vpc/docs/private-access-options#domain-vips)
        ///
        /// * `private-googleapis` - Corresponds to the IP addresses used for Private Google Access for services that do not support VPC Service Controls. [More details.](https://cloud.google.com/vpc/docs/private-access-options#domain-vips)
        ///
        /// * `dns-forwarders` - Corresponds to the IP addresses used to originate Cloud DNS outbound forwarding. [More details.](https://cloud.google.com/dns/zones/#creating-forwarding-zones)
        ///
        /// * `iap-forwarders` - Corresponds to the IP addresses used for Cloud IAP for TCP forwarding. [More details.](https://cloud.google.com/iap/docs/using-tcp-forwarding)
        ///
        /// * `health-checkers` - Corresponds to the IP addresses used for health checking in Cloud Load Balancing. [More details.](https://cloud.google.com/load-balancing/docs/health-checks)
        ///
        /// * `legacy-health-checkers` - Corresponds to the IP addresses used for legacy style health checkers (used by Network Load Balancing). [ More details.](https://cloud.google.com/load-balancing/docs/health-checks)
        #[builder(into, default)]
        pub range_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetblockIpRangesResult {
        /// Retrieve list of all CIDR blocks.
        pub cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// Retrieve list of the IPv4 CIDR blocks
        pub cidr_blocks_ipv4s: pulumi_wasm_rust::Output<Vec<String>>,
        /// Retrieve list of the IPv6 CIDR blocks, if available.
        pub cidr_blocks_ipv6s: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub range_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetblockIpRangesArgs,
    ) -> GetNetblockIpRangesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let range_type_binding = args.range_type.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getNetblockIPRanges:getNetblockIPRanges".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "rangeType".into(),
                    value: &range_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlocksIpv4s".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlocksIpv6s".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "rangeType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetblockIpRangesResult {
            cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlocks").unwrap(),
            ),
            cidr_blocks_ipv4s: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlocksIpv4s").unwrap(),
            ),
            cidr_blocks_ipv6s: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlocksIpv6s").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            range_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rangeType").unwrap(),
            ),
        }
    }
}
