/// Provides an S3 bucket website configuration resource. For more information, see [Hosting Websites on S3](https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html).
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### With `routing_rule` configured
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_website_configuration_v_2::create(
///         "example",
///         BucketWebsiteConfigurationV2Args::builder()
///             .bucket("${exampleAwsS3Bucket.id}")
///             .error_document(
///                 BucketWebsiteConfigurationV2ErrorDocument::builder()
///                     .key("error.html")
///                     .build_struct(),
///             )
///             .index_document(
///                 BucketWebsiteConfigurationV2IndexDocument::builder()
///                     .suffix("index.html")
///                     .build_struct(),
///             )
///             .routing_rules(
///                 vec![
///                     BucketWebsiteConfigurationV2RoutingRule::builder()
///                     .condition(BucketWebsiteConfigurationV2RoutingRuleCondition::builder()
///                     .keyPrefixEquals("docs/").build_struct())
///                     .redirect(BucketWebsiteConfigurationV2RoutingRuleRedirect::builder()
///                     .replaceKeyPrefixWith("documents/").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With `routing_rules` configured
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_website_configuration_v_2::create(
///         "example",
///         BucketWebsiteConfigurationV2Args::builder()
///             .bucket("${exampleAwsS3Bucket.id}")
///             .error_document(
///                 BucketWebsiteConfigurationV2ErrorDocument::builder()
///                     .key("error.html")
///                     .build_struct(),
///             )
///             .index_document(
///                 BucketWebsiteConfigurationV2IndexDocument::builder()
///                     .suffix("index.html")
///                     .build_struct(),
///             )
///             .routing_rule_details(
///                 "[{\n    \"Condition\": {\n        \"KeyPrefixEquals\": \"docs/\"\n    },\n    \"Redirect\": {\n        \"ReplaceKeyPrefixWith\": \"\"\n    }\n}]",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// __Using `pulumi import` to import__ S3 bucket website configuration using the `bucket` or using the `bucket` and `expected_bucket_owner` separated by a comma (`,`). For example:
///
/// If the owner (account ID) of the source bucket is the same account used to configure the AWS Provider, import using the `bucket`:
///
/// ```sh
/// $ pulumi import aws:s3/bucketWebsiteConfigurationV2:BucketWebsiteConfigurationV2 example bucket-name
/// ```
/// If the owner (account ID) of the source bucket differs from the account used to configure the AWS Provider, import using the `bucket` and `expected_bucket_owner` separated by a comma (`,`):
///
/// ```sh
/// $ pulumi import aws:s3/bucketWebsiteConfigurationV2:BucketWebsiteConfigurationV2 example bucket-name,123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_website_configuration_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketWebsiteConfigurationV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the error document for the website. See below.
        #[builder(into, default)]
        pub error_document: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2ErrorDocument>,
        >,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the index document for the website. See below.
        #[builder(into, default)]
        pub index_document: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2IndexDocument>,
        >,
        /// Redirect behavior for every request to this bucket's website endpoint. See below. Conflicts with `error_document`, `index_document`, and `routing_rule`.
        #[builder(into, default)]
        pub redirect_all_requests_to: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::s3::BucketWebsiteConfigurationV2RedirectAllRequestsTo,
            >,
        >,
        /// JSON array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
        /// describing redirect behavior and when redirects are applied. Use this parameter when your routing rules contain empty String values (`""`) as seen in the example above.
        #[builder(into, default)]
        pub routing_rule_details: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of rules that define when a redirect is applied and the redirect behavior. See below.
        #[builder(into, default)]
        pub routing_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketWebsiteConfigurationV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Name of the error document for the website. See below.
        pub error_document: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2ErrorDocument>,
        >,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the index document for the website. See below.
        pub index_document: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2IndexDocument>,
        >,
        /// Redirect behavior for every request to this bucket's website endpoint. See below. Conflicts with `error_document`, `index_document`, and `routing_rule`.
        pub redirect_all_requests_to: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::s3::BucketWebsiteConfigurationV2RedirectAllRequestsTo,
            >,
        >,
        /// JSON array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
        /// describing redirect behavior and when redirects are applied. Use this parameter when your routing rules contain empty String values (`""`) as seen in the example above.
        pub routing_rule_details: pulumi_gestalt_rust::Output<String>,
        /// List of rules that define when a redirect is applied and the redirect behavior. See below.
        pub routing_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRule>,
        >,
        /// Domain of the website endpoint. This is used to create Route 53 alias records.
        pub website_domain: pulumi_gestalt_rust::Output<String>,
        /// Website endpoint.
        pub website_endpoint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketWebsiteConfigurationV2Args,
    ) -> BucketWebsiteConfigurationV2Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let error_document_binding = args.error_document.get_output(context);
        let expected_bucket_owner_binding = args
            .expected_bucket_owner
            .get_output(context);
        let index_document_binding = args.index_document.get_output(context);
        let redirect_all_requests_to_binding = args
            .redirect_all_requests_to
            .get_output(context);
        let routing_rule_details_binding = args.routing_rule_details.get_output(context);
        let routing_rules_binding = args.routing_rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketWebsiteConfigurationV2:BucketWebsiteConfigurationV2"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "errorDocument".into(),
                    value: error_document_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBucketOwner".into(),
                    value: expected_bucket_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexDocument".into(),
                    value: index_document_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redirectAllRequestsTo".into(),
                    value: redirect_all_requests_to_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingRuleDetails".into(),
                    value: routing_rule_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingRules".into(),
                    value: routing_rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketWebsiteConfigurationV2Result {
            bucket: o.get_field("bucket"),
            error_document: o.get_field("errorDocument"),
            expected_bucket_owner: o.get_field("expectedBucketOwner"),
            index_document: o.get_field("indexDocument"),
            redirect_all_requests_to: o.get_field("redirectAllRequestsTo"),
            routing_rule_details: o.get_field("routingRuleDetails"),
            routing_rules: o.get_field("routingRules"),
            website_domain: o.get_field("websiteDomain"),
            website_endpoint: o.get_field("websiteEndpoint"),
        }
    }
}
