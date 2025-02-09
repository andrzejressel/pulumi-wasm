#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_distribution {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDistributionArgs {
        /// Identifier for the distribution. For example: `EDFDVBD632BHDS5`.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDistributionResult {
        /// List that contains information about CNAMEs (alternate domain names), if any, for this distribution.
        pub aliases: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN (Amazon Resource Name) for the distribution. For example: arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5, where 123456789012 is your AWS account ID.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Domain name corresponding to the distribution. For
        /// example: `d604721fxaaqy9.cloudfront.net`.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Current version of the distribution's information. For example:
        /// `E2QWRUHAPOMQZL`.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// CloudFront Route 53 zone ID that can be used to
        /// route an [Alias Resource Record Set][7] to. This attribute is simply an
        /// alias for the zone ID `Z2FDTNDATAQYW2`.
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the distribution. For example: `EDFDVBD632BHDS5`.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The number of invalidation batches
        /// currently in progress.
        pub in_progress_validation_batches: pulumi_gestalt_rust::Output<i32>,
        /// Date and time the distribution was last modified.
        pub last_modified_time: pulumi_gestalt_rust::Output<String>,
        /// Current status of the distribution. `Deployed` if the
        /// distribution's information is fully propagated throughout the Amazon
        /// CloudFront system.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// AWS WAF web ACL associated with this distribution.
        pub web_acl_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDistributionArgs,
    ) -> GetDistributionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding_1 = args.id.get_output(context);
        let id_binding = id_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getDistribution:getDistribution".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDistributionResult {
            aliases: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aliases"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            hosted_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostedZoneId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            in_progress_validation_batches: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inProgressValidationBatches"),
            ),
            last_modified_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedTime"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            web_acl_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webAclId"),
            ),
        }
    }
}
