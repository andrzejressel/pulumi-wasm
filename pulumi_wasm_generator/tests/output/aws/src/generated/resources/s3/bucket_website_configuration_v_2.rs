/// Provides an S3 bucket website configuration resource. For more information, see [Hosting Websites on S3](https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html).
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### With `routing_rule` configured
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod bucket_website_configuration_v_2 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketWebsiteConfigurationV2Args {
        /// Name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Name of the error document for the website. See below.
        #[builder(into, default)]
        pub error_document: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2ErrorDocument>,
        >,
        /// Account ID of the expected bucket owner.
        #[builder(into, default)]
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the index document for the website. See below.
        #[builder(into, default)]
        pub index_document: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2IndexDocument>,
        >,
        /// Redirect behavior for every request to this bucket's website endpoint. See below. Conflicts with `error_document`, `index_document`, and `routing_rule`.
        #[builder(into, default)]
        pub redirect_all_requests_to: pulumi_wasm_rust::Output<
            Option<
                super::super::types::s3::BucketWebsiteConfigurationV2RedirectAllRequestsTo,
            >,
        >,
        /// JSON array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
        /// describing redirect behavior and when redirects are applied. Use this parameter when your routing rules contain empty String values (`""`) as seen in the example above.
        #[builder(into, default)]
        pub routing_rule_details: pulumi_wasm_rust::Output<Option<String>>,
        /// List of rules that define when a redirect is applied and the redirect behavior. See below.
        #[builder(into, default)]
        pub routing_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketWebsiteConfigurationV2Result {
        /// Name of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Name of the error document for the website. See below.
        pub error_document: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2ErrorDocument>,
        >,
        /// Account ID of the expected bucket owner.
        pub expected_bucket_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the index document for the website. See below.
        pub index_document: pulumi_wasm_rust::Output<
            Option<super::super::types::s3::BucketWebsiteConfigurationV2IndexDocument>,
        >,
        /// Redirect behavior for every request to this bucket's website endpoint. See below. Conflicts with `error_document`, `index_document`, and `routing_rule`.
        pub redirect_all_requests_to: pulumi_wasm_rust::Output<
            Option<
                super::super::types::s3::BucketWebsiteConfigurationV2RedirectAllRequestsTo,
            >,
        >,
        /// JSON array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
        /// describing redirect behavior and when redirects are applied. Use this parameter when your routing rules contain empty String values (`""`) as seen in the example above.
        pub routing_rule_details: pulumi_wasm_rust::Output<String>,
        /// List of rules that define when a redirect is applied and the redirect behavior. See below.
        pub routing_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRule>,
        >,
        /// Domain of the website endpoint. This is used to create Route 53 alias records.
        pub website_domain: pulumi_wasm_rust::Output<String>,
        /// Website endpoint.
        pub website_endpoint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketWebsiteConfigurationV2Args,
    ) -> BucketWebsiteConfigurationV2Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let error_document_binding = args.error_document.get_inner();
        let expected_bucket_owner_binding = args.expected_bucket_owner.get_inner();
        let index_document_binding = args.index_document.get_inner();
        let redirect_all_requests_to_binding = args.redirect_all_requests_to.get_inner();
        let routing_rule_details_binding = args.routing_rule_details.get_inner();
        let routing_rules_binding = args.routing_rules.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3/bucketWebsiteConfigurationV2:BucketWebsiteConfigurationV2"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "errorDocument".into(),
                    value: &error_document_binding,
                },
                register_interface::ObjectField {
                    name: "expectedBucketOwner".into(),
                    value: &expected_bucket_owner_binding,
                },
                register_interface::ObjectField {
                    name: "indexDocument".into(),
                    value: &index_document_binding,
                },
                register_interface::ObjectField {
                    name: "redirectAllRequestsTo".into(),
                    value: &redirect_all_requests_to_binding,
                },
                register_interface::ObjectField {
                    name: "routingRuleDetails".into(),
                    value: &routing_rule_details_binding,
                },
                register_interface::ObjectField {
                    name: "routingRules".into(),
                    value: &routing_rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "errorDocument".into(),
                },
                register_interface::ResultField {
                    name: "expectedBucketOwner".into(),
                },
                register_interface::ResultField {
                    name: "indexDocument".into(),
                },
                register_interface::ResultField {
                    name: "redirectAllRequestsTo".into(),
                },
                register_interface::ResultField {
                    name: "routingRuleDetails".into(),
                },
                register_interface::ResultField {
                    name: "routingRules".into(),
                },
                register_interface::ResultField {
                    name: "websiteDomain".into(),
                },
                register_interface::ResultField {
                    name: "websiteEndpoint".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketWebsiteConfigurationV2Result {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            error_document: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorDocument").unwrap(),
            ),
            expected_bucket_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBucketOwner").unwrap(),
            ),
            index_document: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexDocument").unwrap(),
            ),
            redirect_all_requests_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redirectAllRequestsTo").unwrap(),
            ),
            routing_rule_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingRuleDetails").unwrap(),
            ),
            routing_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingRules").unwrap(),
            ),
            website_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("websiteDomain").unwrap(),
            ),
            website_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("websiteEndpoint").unwrap(),
            ),
        }
    }
}