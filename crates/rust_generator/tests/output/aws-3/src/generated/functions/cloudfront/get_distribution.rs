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
        context: &pulumi_gestalt_rust::Context,
        args: GetDistributionArgs,
    ) -> GetDistributionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudfront/getDistribution:getDistribution".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDistributionResult {
            aliases: o.get_field("aliases"),
            arn: o.get_field("arn"),
            domain_name: o.get_field("domainName"),
            enabled: o.get_field("enabled"),
            etag: o.get_field("etag"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            id: o.get_field("id"),
            in_progress_validation_batches: o.get_field("inProgressValidationBatches"),
            last_modified_time: o.get_field("lastModifiedTime"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            web_acl_id: o.get_field("webAclId"),
        }
    }
}
