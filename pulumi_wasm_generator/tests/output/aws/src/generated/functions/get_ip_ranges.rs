pub mod get_ip_ranges {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpRangesArgs {
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Filter IP ranges by regions (or include all regions, if
        /// omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
        /// (e.g., `eu-central-1`)
        #[builder(into, default)]
        pub regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Filter IP ranges by services. Valid items are `amazon`
        /// (for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
        /// `codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
        /// `route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
        /// [`service` attribute][2] documentation for other possible values.
        ///
        /// > **NOTE:** If the specified combination of regions and services does not yield any
        /// CIDR blocks, this call will fail.
        #[builder(into)]
        pub services: pulumi_wasm_rust::Output<Vec<String>>,
        /// Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
        #[builder(into, default)]
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetIpRangesResult {
        /// Lexically ordered list of CIDR blocks.
        pub cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// Publication time of the IP ranges (e.g., `2016-08-03-23-46-05`).
        pub create_date: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Lexically ordered list of IPv6 CIDR blocks.
        pub ipv6_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        pub regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub services: pulumi_wasm_rust::Output<Vec<String>>,
        /// Publication time of the IP ranges, in Unix epoch time format
        /// (e.g., `1470267965`).
        pub sync_token: pulumi_wasm_rust::Output<i32>,
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetIpRangesArgs) -> GetIpRangesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let regions_binding = args.regions.get_inner();
        let services_binding = args.services.get_inner();
        let url_binding = args.url.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getIpRanges:getIpRanges".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "regions".into(),
                    value: &regions_binding,
                },
                register_interface::ObjectField {
                    name: "services".into(),
                    value: &services_binding,
                },
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "createDate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipv6CidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "regions".into(),
                },
                register_interface::ResultField {
                    name: "services".into(),
                },
                register_interface::ResultField {
                    name: "syncToken".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetIpRangesResult {
            cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlocks").unwrap(),
            ),
            create_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createDate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipv6_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6CidrBlocks").unwrap(),
            ),
            regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regions").unwrap(),
            ),
            services: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("services").unwrap(),
            ),
            sync_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncToken").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
