#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ip_ranges {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpRangesArgs {
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Filter IP ranges by regions (or include all regions, if
        /// omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
        /// (e.g., `eu-central-1`)
        #[builder(into, default)]
        pub regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Filter IP ranges by services. Valid items are `amazon`
        /// (for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
        /// `codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
        /// `route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
        /// [`service` attribute][2] documentation for other possible values.
        ///
        /// > **NOTE:** If the specified combination of regions and services does not yield any
        /// CIDR blocks, this call will fail.
        #[builder(into)]
        pub services: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
        #[builder(into, default)]
        pub url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetIpRangesResult {
        /// Lexically ordered list of CIDR blocks.
        pub cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Publication time of the IP ranges (e.g., `2016-08-03-23-46-05`).
        pub create_date: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Lexically ordered list of IPv6 CIDR blocks.
        pub ipv6_cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        pub regions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub services: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Publication time of the IP ranges, in Unix epoch time format
        /// (e.g., `1470267965`).
        pub sync_token: pulumi_gestalt_rust::Output<i32>,
        pub url: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetIpRangesArgs,
    ) -> GetIpRangesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let regions_binding = args.regions.get_output(context);
        let services_binding = args.services.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getIpRanges:getIpRanges".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regions".into(),
                    value: regions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "services".into(),
                    value: services_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: url_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetIpRangesResult {
            cidr_blocks: o.get_field("cidrBlocks"),
            create_date: o.get_field("createDate"),
            id: o.get_field("id"),
            ipv6_cidr_blocks: o.get_field("ipv6CidrBlocks"),
            regions: o.get_field("regions"),
            services: o.get_field("services"),
            sync_token: o.get_field("syncToken"),
            url: o.get_field("url"),
        }
    }
}
