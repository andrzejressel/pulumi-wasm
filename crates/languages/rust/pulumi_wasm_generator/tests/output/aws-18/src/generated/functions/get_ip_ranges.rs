pub mod get_ip_ranges {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpRangesArgs {
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Filter IP ranges by regions (or include all regions, if
        /// omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
        /// (e.g., `eu-central-1`)
        #[builder(into, default)]
        pub regions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Filter IP ranges by services. Valid items are `amazon`
        /// (for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
        /// `codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
        /// `route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
        /// [`service` attribute][2] documentation for other possible values.
        ///
        /// > **NOTE:** If the specified combination of regions and services does not yield any
        /// CIDR blocks, this call will fail.
        #[builder(into)]
        pub services: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
        #[builder(into, default)]
        pub url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetIpRangesArgs,
    ) -> GetIpRangesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let regions_binding = args.regions.get_output(context).get_inner();
        let services_binding = args.services.get_output(context).get_inner();
        let url_binding = args.url.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getIpRanges:getIpRanges".into(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIpRangesResult {
            cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrBlocks"),
            ),
            create_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createDate"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ipv6_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipv6CidrBlocks"),
            ),
            regions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("regions"),
            ),
            services: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("services"),
            ),
            sync_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("syncToken"),
            ),
            url: pulumi_wasm_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
