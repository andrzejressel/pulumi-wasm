pub mod get_bucket {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketArgs {
        /// Name of the bucket
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetBucketResult {
        /// ARN of the bucket. Will be of format `arn:aws:s3:::bucketname`.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Bucket domain name. Will be of format `bucketname.s3.amazonaws.com`.
        pub bucket_domain_name: pulumi_wasm_rust::Output<String>,
        /// The bucket region-specific domain name. The bucket domain name including the region name. Please refer to the [S3 endpoints reference](https://docs.aws.amazon.com/general/latest/gr/s3.html#s3_region) for format. Note: AWS CloudFront allows specifying an S3 region-specific endpoint when creating an S3 origin. This will prevent redirect issues from CloudFront to the S3 Origin URL. For more information, see the [Virtual Hosted-Style Requests for Other Regions](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html#deprecated-global-endpoint) section in the AWS S3 User Guide.
        pub bucket_regional_domain_name: pulumi_wasm_rust::Output<String>,
        /// The [Route 53 Hosted Zone ID](https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_website_region_endpoints) for this bucket's region.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// AWS region this bucket resides in.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Domain of the website endpoint, if the bucket is configured with a website. If not, this will be an empty string. This is used to create Route 53 alias records.
        pub website_domain: pulumi_wasm_rust::Output<String>,
        /// Website endpoint, if the bucket is configured with a website. If not, this will be an empty string.
        pub website_endpoint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBucketArgs) -> GetBucketResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:s3/getBucket:getBucket".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "bucketDomainName".into(),
                },
                register_interface::ResultField {
                    name: "bucketRegionalDomainName".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "websiteDomain".into(),
                },
                register_interface::ResultField {
                    name: "websiteEndpoint".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBucketResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            bucket_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketDomainName").unwrap(),
            ),
            bucket_regional_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucketRegionalDomainName").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
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
