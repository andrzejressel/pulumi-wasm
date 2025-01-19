/// Creates an Amazon CloudFront web distribution.
///
/// For information about CloudFront distributions, see the [Amazon CloudFront Developer Guide](http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html). For specific information about creating CloudFront web distributions, see the [POST Distribution](https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_CreateDistribution.html) page in the Amazon CloudFront API Reference.
///
/// > **NOTE:** CloudFront distributions take about 15 minutes to reach a deployed state after creation or modification. During this time, deletes to resources will be blocked. If you need to delete a distribution that is enabled and you do not want to wait, you need to use the `retain_on_delete` flag.
///
/// ## Example Usage
///
/// ### S3 Origin
///
/// The example below creates a CloudFront distribution with an S3 origin.
///
/// ```yaml
/// resources:
///   b:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: mybucket
///       tags:
///         Name: My bucket
///   bAcl:
///     type: aws:s3:BucketAclV2
///     name: b_acl
///     properties:
///       bucket: ${b.id}
///       acl: private
///   s3Distribution:
///     type: aws:cloudfront:Distribution
///     name: s3_distribution
///     properties:
///       origins:
///         - domainName: ${b.bucketRegionalDomainName}
///           originAccessControlId: ${default.id}
///           originId: ${s3OriginId}
///       enabled: true
///       isIpv6Enabled: true
///       comment: Some comment
///       defaultRootObject: index.html
///       loggingConfig:
///         includeCookies: false
///         bucket: mylogs.s3.amazonaws.com
///         prefix: myprefix
///       aliases:
///         - mysite.example.com
///         - yoursite.example.com
///       defaultCacheBehavior:
///         allowedMethods:
///           - DELETE
///           - GET
///           - HEAD
///           - OPTIONS
///           - PATCH
///           - POST
///           - PUT
///         cachedMethods:
///           - GET
///           - HEAD
///         targetOriginId: ${s3OriginId}
///         forwardedValues:
///           queryString: false
///           cookies:
///             forward: none
///         viewerProtocolPolicy: allow-all
///         minTtl: 0
///         defaultTtl: 3600
///         maxTtl: 86400
///       orderedCacheBehaviors:
///         - pathPattern: /content/immutable/*
///           allowedMethods:
///             - GET
///             - HEAD
///             - OPTIONS
///           cachedMethods:
///             - GET
///             - HEAD
///             - OPTIONS
///           targetOriginId: ${s3OriginId}
///           forwardedValues:
///             queryString: false
///             headers:
///               - Origin
///             cookies:
///               forward: none
///           minTtl: 0
///           defaultTtl: 86400
///           maxTtl: 3.1536e+07
///           compress: true
///           viewerProtocolPolicy: redirect-to-https
///         - pathPattern: /content/*
///           allowedMethods:
///             - GET
///             - HEAD
///             - OPTIONS
///           cachedMethods:
///             - GET
///             - HEAD
///           targetOriginId: ${s3OriginId}
///           forwardedValues:
///             queryString: false
///             cookies:
///               forward: none
///           minTtl: 0
///           defaultTtl: 3600
///           maxTtl: 86400
///           compress: true
///           viewerProtocolPolicy: redirect-to-https
///       priceClass: PriceClass_200
///       restrictions:
///         geoRestriction:
///           restrictionType: whitelist
///           locations:
///             - US
///             - CA
///             - GB
///             - DE
///       tags:
///         Environment: production
///       viewerCertificate:
///         cloudfrontDefaultCertificate: true
/// variables:
///   s3OriginId: myS3Origin
/// ```
///
/// ### With Failover Routing
///
/// The example below creates a CloudFront distribution with an origin group for failover routing.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let s3Distribution = distribution::create(
///         "s3Distribution",
///         DistributionArgs::builder()
///             .default_cache_behavior(
///                 DistributionDefaultCacheBehavior::builder()
///                     .targetOriginId("groupS3")
///                     .build_struct(),
///             )
///             .origin_groups(
///                 vec![
///                     DistributionOriginGroup::builder()
///                     .failoverCriteria(DistributionOriginGroupFailoverCriteria::builder()
///                     .statusCodes(vec![403, 404, 500, 502,]).build_struct())
///                     .members(vec![DistributionOriginGroupMember::builder()
///                     .originId("primaryS3").build_struct(),
///                     DistributionOriginGroupMember::builder().originId("failoverS3")
///                     .build_struct(),]).originId("groupS3").build_struct(),
///                 ],
///             )
///             .origins(
///                 vec![
///                     DistributionOrigin::builder()
///                     .domainName("${primary.bucketRegionalDomainName}")
///                     .originId("primaryS3")
///                     .s3OriginConfig(DistributionOriginS3OriginConfig::builder()
///                     .originAccessIdentity("${default.cloudfrontAccessIdentityPath}")
///                     .build_struct()).build_struct(), DistributionOrigin::builder()
///                     .domainName("${failover.bucketRegionalDomainName}")
///                     .originId("failoverS3")
///                     .s3OriginConfig(DistributionOriginS3OriginConfig::builder()
///                     .originAccessIdentity("${default.cloudfrontAccessIdentityPath}")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Managed Caching Policy
///
/// The example below creates a CloudFront distribution with an [AWS managed caching policy](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-cache-policies.html).
///
/// ```yaml
/// resources:
///   s3Distribution:
///     type: aws:cloudfront:Distribution
///     name: s3_distribution
///     properties:
///       origins:
///         - domainName: ${primary.bucketRegionalDomainName}
///           originId: myS3Origin
///           s3OriginConfig:
///             originAccessIdentity: ${default.cloudfrontAccessIdentityPath}
///       enabled: true
///       isIpv6Enabled: true
///       comment: Some comment
///       defaultRootObject: index.html
///       defaultCacheBehavior:
///         cachePolicyId: 4135ea2d-6df8-44a3-9df3-4b5a84be39ad
///         allowedMethods:
///           - GET
///           - HEAD
///           - OPTIONS
///         targetOriginId: ${s3OriginId}
///       restrictions:
///         geoRestriction:
///           restrictionType: whitelist
///           locations:
///             - US
///             - CA
///             - GB
///             - DE
///       viewerCertificate:
///         cloudfrontDefaultCertificate: true
/// variables:
///   s3OriginId: myS3Origin
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Distributions using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/distribution:Distribution distribution E74FTE3EXAMPLE
/// ```
pub mod distribution {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DistributionArgs {
        #[builder(into, default)]
        pub aliases: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub continuous_deployment_policy_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub custom_error_responses: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudfront::DistributionCustomErrorResponse>>,
        >,
        #[builder(into)]
        pub default_cache_behavior: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::DistributionDefaultCacheBehavior,
        >,
        #[builder(into, default)]
        pub default_root_object: pulumi_wasm_rust::Output<Option<String>>,
        /// `true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        #[builder(into, default)]
        pub http_version: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub is_ipv6_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudfront::DistributionLoggingConfig>,
        >,
        #[builder(into, default)]
        pub ordered_cache_behaviors: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cloudfront::DistributionOrderedCacheBehavior>,
            >,
        >,
        #[builder(into, default)]
        pub origin_groups: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudfront::DistributionOriginGroup>>,
        >,
        #[builder(into)]
        pub origins: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudfront::DistributionOrigin>,
        >,
        #[builder(into, default)]
        pub price_class: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub restrictions: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::DistributionRestrictions,
        >,
        #[builder(into, default)]
        pub retain_on_delete: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub staging: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into)]
        pub viewer_certificate: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::DistributionViewerCertificate,
        >,
        #[builder(into, default)]
        pub wait_for_deployment: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub web_acl_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DistributionResult {
        pub aliases: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ARN for the distribution. For example: `arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5`, where `123456789012` is your AWS account ID.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Internal value used by CloudFront to allow future updates to the distribution configuration.
        pub caller_reference: pulumi_wasm_rust::Output<String>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub continuous_deployment_policy_id: pulumi_wasm_rust::Output<String>,
        pub custom_error_responses: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudfront::DistributionCustomErrorResponse>>,
        >,
        pub default_cache_behavior: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::DistributionDefaultCacheBehavior,
        >,
        pub default_root_object: pulumi_wasm_rust::Output<Option<String>>,
        /// Domain name corresponding to the distribution. For example: `d604721fxaaqy9.cloudfront.net`.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// `true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Current version of the distribution's information. For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// CloudFront Route 53 zone ID that can be used to route an [Alias Resource Record Set](http://docs.aws.amazon.com/Route53/latest/APIReference/CreateAliasRRSAPI.html) to. This attribute is simply an alias for the zone ID `Z2FDTNDATAQYW2`.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        pub http_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of invalidation batches currently in progress.
        pub in_progress_validation_batches: pulumi_wasm_rust::Output<i32>,
        pub is_ipv6_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Date and time the distribution was last modified.
        pub last_modified_time: pulumi_wasm_rust::Output<String>,
        pub logging_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudfront::DistributionLoggingConfig>,
        >,
        pub ordered_cache_behaviors: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cloudfront::DistributionOrderedCacheBehavior>,
            >,
        >,
        pub origin_groups: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudfront::DistributionOriginGroup>>,
        >,
        pub origins: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudfront::DistributionOrigin>,
        >,
        pub price_class: pulumi_wasm_rust::Output<Option<String>>,
        pub restrictions: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::DistributionRestrictions,
        >,
        pub retain_on_delete: pulumi_wasm_rust::Output<Option<bool>>,
        pub staging: pulumi_wasm_rust::Output<Option<bool>>,
        /// Current status of the distribution. `Deployed` if the distribution's information is fully propagated throughout the Amazon CloudFront system.
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of nested attributes for active trusted key groups, if the distribution is set up to serve private content with signed URLs.
        pub trusted_key_groups: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudfront::DistributionTrustedKeyGroup>,
        >,
        /// List of nested attributes for active trusted signers, if the distribution is set up to serve private content with signed URLs.
        pub trusted_signers: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudfront::DistributionTrustedSigner>,
        >,
        pub viewer_certificate: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::DistributionViewerCertificate,
        >,
        pub wait_for_deployment: pulumi_wasm_rust::Output<Option<bool>>,
        pub web_acl_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DistributionArgs) -> DistributionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aliases_binding = args.aliases.get_inner();
        let comment_binding = args.comment.get_inner();
        let continuous_deployment_policy_id_binding = args
            .continuous_deployment_policy_id
            .get_inner();
        let custom_error_responses_binding = args.custom_error_responses.get_inner();
        let default_cache_behavior_binding = args.default_cache_behavior.get_inner();
        let default_root_object_binding = args.default_root_object.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let http_version_binding = args.http_version.get_inner();
        let is_ipv6_enabled_binding = args.is_ipv6_enabled.get_inner();
        let logging_config_binding = args.logging_config.get_inner();
        let ordered_cache_behaviors_binding = args.ordered_cache_behaviors.get_inner();
        let origin_groups_binding = args.origin_groups.get_inner();
        let origins_binding = args.origins.get_inner();
        let price_class_binding = args.price_class.get_inner();
        let restrictions_binding = args.restrictions.get_inner();
        let retain_on_delete_binding = args.retain_on_delete.get_inner();
        let staging_binding = args.staging.get_inner();
        let tags_binding = args.tags.get_inner();
        let viewer_certificate_binding = args.viewer_certificate.get_inner();
        let wait_for_deployment_binding = args.wait_for_deployment.get_inner();
        let web_acl_id_binding = args.web_acl_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/distribution:Distribution".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aliases".into(),
                    value: &aliases_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "continuousDeploymentPolicyId".into(),
                    value: &continuous_deployment_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "customErrorResponses".into(),
                    value: &custom_error_responses_binding,
                },
                register_interface::ObjectField {
                    name: "defaultCacheBehavior".into(),
                    value: &default_cache_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRootObject".into(),
                    value: &default_root_object_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "httpVersion".into(),
                    value: &http_version_binding,
                },
                register_interface::ObjectField {
                    name: "isIpv6Enabled".into(),
                    value: &is_ipv6_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "orderedCacheBehaviors".into(),
                    value: &ordered_cache_behaviors_binding,
                },
                register_interface::ObjectField {
                    name: "originGroups".into(),
                    value: &origin_groups_binding,
                },
                register_interface::ObjectField {
                    name: "origins".into(),
                    value: &origins_binding,
                },
                register_interface::ObjectField {
                    name: "priceClass".into(),
                    value: &price_class_binding,
                },
                register_interface::ObjectField {
                    name: "restrictions".into(),
                    value: &restrictions_binding,
                },
                register_interface::ObjectField {
                    name: "retainOnDelete".into(),
                    value: &retain_on_delete_binding,
                },
                register_interface::ObjectField {
                    name: "staging".into(),
                    value: &staging_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "viewerCertificate".into(),
                    value: &viewer_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "waitForDeployment".into(),
                    value: &wait_for_deployment_binding,
                },
                register_interface::ObjectField {
                    name: "webAclId".into(),
                    value: &web_acl_id_binding,
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
                    name: "callerReference".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "continuousDeploymentPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "customErrorResponses".into(),
                },
                register_interface::ResultField {
                    name: "defaultCacheBehavior".into(),
                },
                register_interface::ResultField {
                    name: "defaultRootObject".into(),
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
                    name: "httpVersion".into(),
                },
                register_interface::ResultField {
                    name: "inProgressValidationBatches".into(),
                },
                register_interface::ResultField {
                    name: "isIpv6Enabled".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfig".into(),
                },
                register_interface::ResultField {
                    name: "orderedCacheBehaviors".into(),
                },
                register_interface::ResultField {
                    name: "originGroups".into(),
                },
                register_interface::ResultField {
                    name: "origins".into(),
                },
                register_interface::ResultField {
                    name: "priceClass".into(),
                },
                register_interface::ResultField {
                    name: "restrictions".into(),
                },
                register_interface::ResultField {
                    name: "retainOnDelete".into(),
                },
                register_interface::ResultField {
                    name: "staging".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "trustedKeyGroups".into(),
                },
                register_interface::ResultField {
                    name: "trustedSigners".into(),
                },
                register_interface::ResultField {
                    name: "viewerCertificate".into(),
                },
                register_interface::ResultField {
                    name: "waitForDeployment".into(),
                },
                register_interface::ResultField {
                    name: "webAclId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DistributionResult {
            aliases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliases").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            caller_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callerReference").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            continuous_deployment_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("continuousDeploymentPolicyId").unwrap(),
            ),
            custom_error_responses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customErrorResponses").unwrap(),
            ),
            default_cache_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultCacheBehavior").unwrap(),
            ),
            default_root_object: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRootObject").unwrap(),
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
            http_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpVersion").unwrap(),
            ),
            in_progress_validation_batches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inProgressValidationBatches").unwrap(),
            ),
            is_ipv6_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isIpv6Enabled").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfig").unwrap(),
            ),
            ordered_cache_behaviors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orderedCacheBehaviors").unwrap(),
            ),
            origin_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originGroups").unwrap(),
            ),
            origins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("origins").unwrap(),
            ),
            price_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priceClass").unwrap(),
            ),
            restrictions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restrictions").unwrap(),
            ),
            retain_on_delete: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retainOnDelete").unwrap(),
            ),
            staging: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staging").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            trusted_key_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedKeyGroups").unwrap(),
            ),
            trusted_signers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedSigners").unwrap(),
            ),
            viewer_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("viewerCertificate").unwrap(),
            ),
            wait_for_deployment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForDeployment").unwrap(),
            ),
            web_acl_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webAclId").unwrap(),
            ),
        }
    }
}
