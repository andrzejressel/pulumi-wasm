pub mod get_distribution {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDistributionArgs {
        /// Identifier for the distribution. For example: `EDFDVBD632BHDS5`.
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDistributionResult {
        /// List that contains information about CNAMEs (alternate domain names), if any, for this distribution.
        pub aliases: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN (Amazon Resource Name) for the distribution. For example: arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5, where 123456789012 is your AWS account ID.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Domain name corresponding to the distribution. For
        /// example: `d604721fxaaqy9.cloudfront.net`.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Current version of the distribution's information. For example:
        /// `E2QWRUHAPOMQZL`.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// CloudFront Route 53 zone ID that can be used to
        /// route an [Alias Resource Record Set][7] to. This attribute is simply an
        /// alias for the zone ID `Z2FDTNDATAQYW2`.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Identifier for the distribution. For example: `EDFDVBD632BHDS5`.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The number of invalidation batches
        /// currently in progress.
        pub in_progress_validation_batches: pulumi_wasm_rust::Output<i32>,
        /// Date and time the distribution was last modified.
        pub last_modified_time: pulumi_wasm_rust::Output<String>,
        /// Current status of the distribution. `Deployed` if the
        /// distribution's information is fully propagated throughout the Amazon
        /// CloudFront system.
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// AWS WAF web ACL associated with this distribution.
        pub web_acl_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDistributionArgs) -> GetDistributionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getDistribution:getDistribution".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aliases".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inProgressValidationBatches".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "webAclId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDistributionResult {
            aliases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliases").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            in_progress_validation_batches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inProgressValidationBatches").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            web_acl_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webAclId").unwrap(),
            ),
        }
    }
}
